struct Mbc1 {}

impl MemoryController for Mbc1 {
    fn write(&mut self, _address: GBAddress, value: u8) {

    }

    fn read(&self, _address: GBAdress) -> u8 {

    }
}

