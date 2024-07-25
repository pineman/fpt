use std::cell::RefCell;
use std::rc::Rc;

use super::cartridge::{get_cartridge_type, EmptyCartridge};
use super::mbc_none::NoMbcCartridge;
use super::Cartridge;

pub fn create_mbc(cartridge_data: &[u8]) -> Option<Rc<RefCell<dyn Cartridge>>> {
    // https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
    match get_cartridge_type(cartridge_data) {
        0x00 => Some(Rc::new(RefCell::new(NoMbcCartridge::new(cartridge_data)))), // rom only
        _ => None,
    }
}

pub fn create_empty_mbc() -> Rc<RefCell<dyn Cartridge>> {
    Rc::new(RefCell::new(EmptyCartridge::new()))
}
