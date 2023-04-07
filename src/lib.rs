#![no_std]
#![feature(error_in_core)]

use core::error::Error;

use registers::*;
use thiserror::Error;

pub struct Radio {}

impl Radio {
    pub fn new(&self) -> Self {
        todo!()
    }

    pub fn begin_packet(&self, implicit_header: bool) {
        todo!()
    }

    pub fn end_packet(&self, nonblock: bool) {
        todo!()
    }

    pub fn is_transmitting(&self) -> bool {
        todo!()
    }

    pub fn parse_packet(&self, size: i32) {
        todo!()
    }

    pub fn last_packet_rssi(&self) -> i32 {
        todo!()
    }

    pub fn last_packet_snr(&self) -> f64 {
        return self.read_register(REG_PKT_SNR_VALUE) as f64 * 0.25;
    }

    pub fn packet_frequency_error(&self) -> i64 {
        todo!()
    }

    pub fn rssi(&self) -> i32 {
        todo!()
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let current_length = self.read_register(REG_PAYLOAD_LENGTH) as usize;

        let mut size = buf.len();
        
        if (current_length + size) > MAX_PKT_LENGTH as usize {
            size = MAX_PKT_LENGTH as usize - current_length;
        }

        for i in 0..size {
            self.write_register(REG_FIFO, buf[i]);
        }

        // THIS WILL NEVER FAIL.
        let new_length = (current_length + size).try_into().unwrap();

        self.write_register(REG_PAYLOAD_LENGTH, new_length);
        
        return size;
    }

    pub fn is_available(&self) -> bool {
        todo!()
    }

    pub fn read(&self) -> i32 {
        todo!()
    }

    pub fn receive(&self, size: i32) {
        todo!()
    }

    pub fn set_idle(&self) {
        self.write_register(REG_OP_MODE, MODE_LONG_RANGE_MODE | MODE_STDBY);
    }

    pub fn set_sleep(&self) {
        self.write_register(REG_OP_MODE, MODE_LONG_RANGE_MODE | MODE_SLEEP);
    }

    pub fn set_tx_power(&self, level: i32) -> Result<(), RadioError> {
        todo!()
    }

    pub fn set_frequency(&self, freq: i64) -> Result<(), RadioError> {
        todo!()
    }

    pub fn set_spreading_factor(&self, factor: u8) -> Result<(), RadioError> {
        if factor < 6 || factor > 12 {
            return Err(RadioError::ValueOutOfRange(6, 12));
        }

        if factor == 6 {
            self.write_register(REG_DETECTION_OPTIMIZE, 0xc5);
            self.write_register(REG_DETECTION_THRESHOLD, 0x0c);
        } else {
            self.write_register(REG_DETECTION_OPTIMIZE, 0xc3);
            self.write_register(REG_DETECTION_THRESHOLD, 0x0a);
        }

        self.write_register(
            REG_MODEM_CONFIG_2,
            (self.read_register(REG_MODEM_CONFIG_2) & 0x0f) | ((factor << 4) & 0xf0),
        );

        self.set_ldo_flag();
        Ok(())
    }

    pub fn get_spreading_factor(&self) -> u8 {
        return self.read_register(REG_MODEM_CONFIG_2) >> 4;
    }

    pub fn get_signal_bandwidth(&self) -> Result<SignalBandwidth, RadioError> {
        let bw = self.read_register(REG_MODEM_CONFIG_1) >> 4;

        return SignalBandwidth::try_from(bw);
    }

    pub fn set_signal_bandwidth(&self, bw: SignalBandwidth) {
        let a = bw as u8;
        self.write_register(
            REG_MODEM_CONFIG_1,
            (self.read_register(REG_MODEM_CONFIG_1) & 0x0f) | (a << 4),
        );
        self.set_ldo_flag();
    }

    pub fn set_gain(&self, gain: i32) -> Result<(), RadioError> {
        todo!()
    }

    pub fn set_crc_enabled(&self, enabled: bool) {
        todo!()
    }

    pub fn set_invert_iq(&self, enabled: bool) {
        todo!()
    }

    pub fn set_sync_word(&self, word: i32) {
        todo!()
    }

    pub fn set_preamble_length(&self, word: i64) {
        todo!()
    }

    pub fn set_coding_rate4(&self, denominator: u8) -> Result<(), RadioError> {
        if denominator < 5 || denominator > 8 {
            return Err(RadioError::ValueOutOfRange(5, 8));
        }

        let rate = denominator - 4;

        self.write_register(
            REG_MODEM_CONFIG_1,
            (self.read_register(REG_MODEM_CONFIG_1) & 0xf1) | (rate << 1),
        );

        Ok(())
    }

    fn set_ldo_flag(&self) {
        todo!()
    }

    //todo: add the rest of the shit

    fn read_register(&self, addr: u8) -> u8 {
        return self.single_transfer(addr & 0x7f, 0x00);
    }

    fn write_register(&self, addr: u8, value: u8) {
        self.single_transfer(addr, value);
    }

    fn single_transfer(&self, addr: u8, value: u8) -> u8 {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum RadioError {
    #[error("The packet is too big. The max size is {}", registers::MAX_PKT_LENGTH)]
    PacketTooBig,
    #[error("Value out of range. The allowed range is [{0}-{1}]")]
    ValueOutOfRange(u8, u8),
    #[error("Unknown error.")]
    Unknown,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignalBandwidth {
    a,
}

impl TryFrom<u8> for SignalBandwidth {
    type Error = RadioError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Err(RadioError::Unknown)
    }
}

mod registers;
