#![no_main]
#![no_std]

use defmt::unwrap;
use logicalrust as _; // global logger + panicking-behavior + memory layout

use crate::hal::{prelude::*, stm32};
use nb::block;
use stm32f4xx_hal as hal;

use logicalrust::sump::*;

trait PutC {
    fn putc(&mut self, byte: u8);
    fn put<'a, I>(&mut self, bytes: I)
    where
        I: IntoIterator<Item = &'a u8>;
}

impl PutC for hal::serial::Tx<hal::stm32::USART2> {
    fn putc(&mut self, byte: u8) {
        block!(self.write(byte)).unwrap();
    }
    fn put<'a, I>(&mut self, bytes: I)
    where
        I: IntoIterator<Item = &'a u8>,
    {
        bytes.into_iter().for_each(|b| self.putc(*b));
    }
}

trait GetC {
    fn getc(&mut self) -> u8;
    fn get_u16(&mut self) -> u16;
    fn get_u32(&mut self) -> u32;
}

impl GetC for hal::serial::Rx<hal::stm32::USART2> {
    fn getc(&mut self) -> u8 {
        block!(self.read()).unwrap()
    }
    fn get_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.getc(), self.getc()])
    }
    fn get_u32(&mut self) -> u32 {
        u32::from_le_bytes([self.getc(), self.getc(), self.getc(), self.getc()])
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Rust Logic Analyzer!");
    let dp = unwrap!(stm32::Peripherals::take());
    let cp = unwrap!(cortex_m::peripheral::Peripherals::take());

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(100.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    unwrap!(led.set_high());

    let serial = hal::serial::Serial::usart2(
        dp.USART2,
        (
            gpioa.pa2.into_alternate_af7(),
            gpioa.pa3.into_alternate_af7(),
        ),
        hal::serial::config::Config::default().baudrate(115200.bps()),
        clocks,
    )
    .unwrap();
    let (mut tx, mut rx) = serial.split();

    let gpiob = dp.GPIOB.split();
    gpiob.pb0.into_floating_input();
    gpiob.pb1.into_floating_input();
    gpiob.pb2.into_floating_input();
    gpiob.pb3.into_floating_input();
    gpiob.pb4.into_floating_input();
    gpiob.pb5.into_floating_input();
    gpiob.pb6.into_floating_input();
    gpiob.pb7.into_floating_input();

    let mut sampler = Sampler::new(hal::delay::Delay::new(cp.SYST, clocks));

    loop {
        match block!(rx.read()).unwrap() {
            Cmd::RESET => defmt::info!("reset"),
            Cmd::ID => {
                defmt::info!("ID");
                tx.put(b"1ALS");
            }
            Cmd::ARM => {
                defmt::info!("ARM");
                let mut data = [0u8; Sampler::SAMPLE_MEMORY];
                sampler.run(&mut data);
                tx.put(&data[0..sampler.read_cnt]);
            }
            Cmd::GET_METADATA => {
                defmt::info!("META");

                tx.putc(Meta::NAME);
                tx.put(b"logicalrust");
                tx.putc(Meta::END);

                tx.putc(Meta::SAMPLE_MEMORY);
                tx.put(&Sampler::SAMPLE_MEMORY.to_be_bytes());

                tx.putc(Meta::DYNAMIC_MEMORY);
                tx.put(&0usize.to_be_bytes());

                tx.putc(Meta::MAX_SAMPLERATE);
                tx.put(&Sampler::MAX_SAMPLERATE.to_be_bytes());

                tx.putc(Meta::NUM_PROBES);
                tx.putc(8);

                tx.putc(Meta::PROTOCOL_VERSION);
                tx.putc(2);

                tx.putc(Meta::END);
            }
            Cmd::SET_DIVIDER => {
                sampler.period = 10 + 10 * rx.get_u32();
                defmt::info!("period {}", sampler.period);
            }
            Cmd::SET_READ_DELAY => {
                sampler.read_cnt = 4 + 4 * rx.get_u16() as usize;
                sampler.start_delay = 4 * rx.get_u16() as u32;
                defmt::info!("count {} delay {}", sampler.read_cnt, sampler.start_delay);
            }
            Cmd::SET_FLAGS => {
                sampler.flags = rx.get_u32();
                defmt::info!("flags {=u32:x}", sampler.flags);
            }
            Cmd::SET_TRIGGER_MASK => {
                sampler.trigger_mask = rx.get_u32();
                defmt::info!("trigmask {=u32:x}", sampler.trigger_mask);
            }
            Cmd::SET_TRIGGER_VALUE => {
                sampler.trigger_val = rx.get_u32();
                defmt::info!("trigval {}", sampler.trigger_val);
            }
            Cmd::SET_TRIGGER_CONF => {
                sampler.trigger_conf = rx.get_u32();
                defmt::info!("trigconf {=u32:x}", sampler.trigger_conf);
            }
            unhandled => {
                defmt::info!("UNHANDLED {}", unhandled);
                break;
            }
        }
    }

    logicalrust::exit()
}
