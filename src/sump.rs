use cortex_m::prelude::*;
use hal::stm32;
use stm32f4xx_hal as hal;

pub struct Meta;

impl Meta {
    pub const END: u8 = 0x00;
    pub const NAME: u8 = 0x01;
    pub const SAMPLE_MEMORY: u8 = 0x21;
    pub const DYNAMIC_MEMORY: u8 = 0x22;
    pub const MAX_SAMPLERATE: u8 = 0x23;
    pub const NUM_PROBES: u8 = 0x40;
    pub const PROTOCOL_VERSION: u8 = 0x41;
}

pub struct Cmd;

impl Cmd {
    pub const RESET: u8 = 0x00;
    pub const ARM: u8 = 0x01;
    pub const ID: u8 = 0x02;
    pub const GET_METADATA: u8 = 0x04;
    pub const SET_DIVIDER: u8 = 0x80;
    pub const SET_READ_DELAY: u8 = 0x81;
    pub const SET_FLAGS: u8 = 0x82;
    pub const SET_TRIGGER_MASK: u8 = 0xC0;
    pub const SET_TRIGGER_VALUE: u8 = 0xC1;
    pub const SET_TRIGGER_CONF: u8 = 0xC2;
}

pub struct Sampler {
    pub period: u32, // ns
    pub read_cnt: usize,
    pub start_delay: u32, // us
    pub flags: u32,
    pub trigger_mask: u32,
    pub trigger_val: u32,
    pub trigger_conf: u32,
    delay: hal::delay::Delay,
}

impl Sampler {
    pub const SAMPLE_MEMORY: usize = 100_000;
    pub const MAX_SAMPLERATE: usize = 50_000_000; // Hz

    pub fn new(delay: hal::delay::Delay) -> Self {
        Self {
            period: 0,
            read_cnt: 0,
            start_delay: 0,
            flags: 0,
            trigger_mask: 0,
            trigger_val: 0,
            trigger_conf: 0,
            delay,
        }
    }

    pub fn run(&mut self, data: &mut [u8; Sampler::SAMPLE_MEMORY]) {
        if self.read_cnt > Sampler::SAMPLE_MEMORY {
            self.read_cnt = Sampler::SAMPLE_MEMORY;
        }
        if self.period < 20 {
            self.period = 20;
        }
        self.delay.delay_us(self.start_delay);

        defmt::info!(
            "start collecting {} samples at {} ns interval",
            self.read_cnt,
            self.period
        );

        match self.period {
            20 => {
                // not really 50MHz, more like 40MHz
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                }
            }
            50 => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    cortex_m::asm::nop();
                }
            }
            100 => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    for _ in 0..7 {
                        cortex_m::asm::nop();
                    }
                }
            }
            200 => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    for _ in 0..16 {
                        cortex_m::asm::nop();
                    }
                }
            }
            500 => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    for _ in 0..40 {
                        cortex_m::asm::nop();
                    }
                }
            }
            1000 => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    for _ in 0..100 {
                        cortex_m::asm::nop();
                    }
                }
            }
            period => {
                for data in data[0..self.read_cnt].iter_mut() {
                    *data = unsafe { ((*stm32::GPIOB::ptr()).idr.read().bits()) as u8 };
                    self.delay.delay_us(period / 1000)
                }
            }
        }

        defmt::info!("done collecting samples");

        data[0..self.read_cnt].reverse(); // SUMP protocol has last samples first
    }
}
