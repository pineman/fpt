use std::cell::RefCell;
use std::rc::Rc;

use super::cartridge::{get_cartridge_type, EmptyCartridge};
use super::mbc_none::NoMbcCartridge;
use super::Cartridge;

pub fn create_mbc(cartridge_data: &[u8]) -> Rc<RefCell<dyn Cartridge>> {
    let cartridge_type = get_cartridge_type(cartridge_data);

    match dbg!(cartridge_type) {
        0x00 => Rc::new(RefCell::new(NoMbcCartridge::new(cartridge_data))),
        _ => panic!(),
    }
}

pub fn create_empty_mbc() -> Rc<RefCell<dyn Cartridge>> {
    Rc::new(RefCell::new(EmptyCartridge::new()))
}
