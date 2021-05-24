#![no_main]
#![no_std]

use logicalrust as _; // global logger + panicking-behavior + memory layout
use defmt::unwrap;

use stm32f4xx_hal as hal;
use crate::hal::{prelude::*, stm32};
use core::fmt::Write; // for pretty formatting of the serial output

fn read_with_timeout<S>(timer: &mut hal::timer::Timer<stm32::TIM1>, rx: &mut S) -> Option<u8>
where S: embedded_hal::serial::Read<u8>,
{
    timer.start(1.hz());
    loop {
        match rx.read() {
            Ok(byte) => return Some(byte),
            Err(hal::nb::Error::WouldBlock) => (),
            Err(_) => break,
        }
        match timer.wait() {
            Err(hal::nb::Error::WouldBlock) => continue,
            _ => break,
        }
    }
    None
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Rust Logical Analyzer!");
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

    for i in 0..5 {
        writeln!(tx, "i: {:02}\r", i).unwrap();
        // block!(tx.flush()).unwrap();

        // On for 1s, off for 1s.
        led.set_high().unwrap();
        if let Some(b) = read_with_timeout(&mut timer, &mut rx) {
            defmt::info!("read {}", b);
        } else {
            defmt::info!("timeout");
        }
        // delay.delay_ms(1000_u32);
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }

    logicalrust::exit()
}
