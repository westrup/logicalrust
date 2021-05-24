#![no_main]
#![no_std]

use logicalrust as _; // global logger + panicking-behavior + memory layout
use defmt::unwrap;

use stm32f4xx_hal as hal;
use crate::hal::{block, prelude::*, stm32};
use core::fmt::Write; // for pretty formatting of the serial output

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
        for _ in 0..100 {
            if let Ok(rec) = rx.read() {
                defmt::info!("read {}", rec);
            }
        }
        writeln!(tx, "i: {:02}\r", i).unwrap();
        // block!(tx.flush()).unwrap();

        // On for 1s, off for 1s.
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }

    logicalrust::exit()
}
