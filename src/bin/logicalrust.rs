#![no_main]
#![no_std]

use logicalrust as _; // global logger + panicking-behavior + memory layout
use defmt::unwrap;

use stm32f4xx_hal as hal;
use crate::hal::{prelude::*, stm32};
use core::{fmt::Write}; // for pretty formatting of the serial output
use nb::block;

enum Sump {
    Reset,
    Arm,
    Id,
    Test,
    GetMetadata,
    RleFinish,
    XOn,
    XOff,
    SetTriggerMask,
    SetTriggerValues,
    SetTriggerConf,
    SetDivider,
    SetReadDelayCount,
    SetFlags,
    Unknown(u8),
}

enum SumpMeta {
    End = 0x00,
    Name = 0x01,
    SampleMemory = 0x21,
    DynamicMemory = 0x22,
    MaxSampleRate = 0x23,
    NumProbes = 0x40,
    ProtocolVersion = 0x41,
}

impl Sump {
    fn from_byte(byte: u8) -> Sump {
        match byte {
            0x00 => Sump::Reset,
            0x01 => Sump::Arm,
            0x02 => Sump::Id,
            0x03 => Sump::Test,
            0x04 => Sump::GetMetadata,
            0x05 => Sump::RleFinish,
            0x11 => Sump::XOn,
            0x12 => Sump::XOff,
            0xC0 => Sump::SetTriggerMask,
            0xC1 => Sump::SetTriggerValues,
            0xC2 => Sump::SetTriggerConf,
            0x80 => Sump::SetDivider,
            0x81 => Sump::SetReadDelayCount,
            0x82 => Sump::SetFlags,
            _ => Sump::Unknown(byte),
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Rust Logic Analyzer!");
    let dp = unwrap!(stm32::Peripherals::take());
    let cp = unwrap!(cortex_m::peripheral::Peripherals::take());

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(100.mhz()).freeze();

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    let mut timer = hal::timer::Timer::tim1(dp.TIM1, 100.hz(), clocks);

    let tx_pin = gpioa.pa2.into_alternate_af7();
    let rx_pin = gpioa.pa3.into_alternate_af7();

    // configure serial
    let serial = hal::serial::Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        hal::serial::config::Config::default().baudrate(115200.bps()),
        clocks,
    ).unwrap();

    let (mut tx, mut rx) = serial.split();

    unwrap!(led.set_high());

    loop {
        // if let Some(cmd) = read_with_timeout(&mut timer, &mut rx) {
        match Sump::from_byte(block!(rx.read()).unwrap()) {
            Sump::Reset => defmt::info!("reset"),
            Sump::Id => {
                defmt::info!("ID");
                write!(tx, "1ALS").unwrap();
            },
            Sump::GetMetadata => {
                defmt::info!("meta");

                block!(tx.write(SumpMeta::Name as u8)).unwrap();
                write!(tx, "logicalrust").unwrap();
                block!(tx.write(SumpMeta::End as u8)).unwrap();

                block!(tx.write(SumpMeta::SampleMemory as u8)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(1)).unwrap();
                block!(tx.write(0)).unwrap();

                block!(tx.write(SumpMeta::DynamicMemory as u8)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(0)).unwrap();

                block!(tx.write(SumpMeta::MaxSampleRate as u8)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(1)).unwrap();
                block!(tx.write(0)).unwrap();
                block!(tx.write(0)).unwrap();

                block!(tx.write(SumpMeta::NumProbes as u8)).unwrap();
                block!(tx.write(0x8)).unwrap();

                block!(tx.write(SumpMeta::ProtocolVersion as u8)).unwrap();
                block!(tx.write(0x2)).unwrap();

                block!(tx.write(SumpMeta::End as u8)).unwrap();
            },
            Sump::Unknown(b) => defmt::info!("unknown {}", b),
            _ => defmt::info!("unhandled"),
        }
    }
    // unwrap!(led.set_low());

    // for i in 0..5 {
    //     writeln!(tx, "i: {:02}\r", i).unwrap();
    //     // block!(tx.flush()).unwrap();

    //     delay.delay_ms(1000_u32);
    // }

    logicalrust::exit()
}
