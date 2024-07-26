use std::cell::RefCell;

use super::cartridge::{get_cartridge_type, EmptyCartridge};
use super::mbc_none::NoMbcCartridge;
use super::Cartridge;

pub fn create_mbc(cartridge_data: &[u8]) -> Option<Box<RefCell<dyn Cartridge>>> {
    // https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
    match get_cartridge_type(cartridge_data) {
        0x00 => Some(Box::new(RefCell::new(NoMbcCartridge::new(cartridge_data)))), // rom only
        _ => None,
    }
}

pub fn create_empty_mbc() -> Box<RefCell<dyn Cartridge>> {
    Box::new(RefCell::new(EmptyCartridge::new()))
}
