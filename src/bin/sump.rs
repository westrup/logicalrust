use crate::hal::{prelude::*,stm32};
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
    data: [u8; Sampler::SAMPLE_MEMORY],
    delay: hal::delay::Delay,
}

impl Sampler {
    pub const SAMPLE_MEMORY: usize = 100_000;
    pub const MAX_SAMPLERATE: usize = 20_000_000; // Hz

    pub fn new(delay: hal::delay::Delay) -> Self {
        Self {
            period: 0,
            read_cnt: 0,
            start_delay: 0,
            flags: 0,
            trigger_mask: 0,
            trigger_val: 0,
            trigger_conf: 0,
            data: [0u8; Sampler::SAMPLE_MEMORY],
            delay,
        }
    }

    pub fn run(&mut self) -> impl IntoIterator<Item = &u8> {

        self.delay.delay_us(self.start_delay);

        defmt::info!("start collecting {} samples at {} ns interval", self.read_cnt, self.period);

        match self.period {
            50 => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                }
            },
            100 => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                }
            },
            200 => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                }
            },
            500 => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                }
            },
            1000 => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                    unsafe{asm!("nop");asm!("nop");asm!("nop");asm!("nop");};
                }
            },
            _ => {
                for i in 0..self.read_cnt {
                    self.data[i] = unsafe {((*stm32::GPIOB::ptr()).idr.read().bits()) as u8};
                    self.delay.delay_us(self.period / 1000)
                }
            },
        }

        defmt::info!("done collecting samples");

        self.data[0..self.read_cnt].iter()
    }
}
