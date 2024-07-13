use std::rc::Rc;
use std::cell::RefCell;

use super::Cartridge;
use super::cartridge::{EmptyCartridge, get_cartridge_type};
use super::mbc_none::NoMbcCartridge;
use super::mbc1::Mbc1Cartridge;
use super::mbc3::Mbc3Cartridge;

pub fn create_mbc(cartridge_data: &[u8]) -> Rc<RefCell<dyn Cartridge>> {
    let cartridge_type = get_cartridge_type(cartridge_data);

    match dbg!(cartridge_type) {
        0x00 => Rc::new(RefCell::new(NoMbcCartridge::new(cartridge_data))),
        0x01 | 0x02 | 0x03 => Rc::new(RefCell::new(Mbc1Cartridge::new(cartridge_data))),
        0x0F | 0x10 | 0x11 | 0x12 | 0x13  => Rc::new(RefCell::new(Mbc3Cartridge::new(cartridge_data))),
        _ => panic!(),
    }
}

pub fn create_empty_mbc() -> Rc<RefCell<dyn Cartridge>> {
    Rc::new(RefCell::new(EmptyCartridge::new()))
}
