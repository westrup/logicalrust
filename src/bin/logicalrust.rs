#![no_main]
#![no_std]

use logicalrust as _; // global logger + panicking-behavior + memory layout
use defmt::unwrap;

use stm32f4xx_hal as hal;
use crate::hal::{prelude::*, stm32};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Rust Logic Analyzer!");
    let dp = unwrap!(stm32::Peripherals::take());
    let cp = unwrap!(cortex_m::peripheral::Peripherals::take());

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    for _ in 0..5 {
        // On for 1s, off for 1s.
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }

    logicalrust::exit()
}
