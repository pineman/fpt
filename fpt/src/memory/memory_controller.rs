trait MemoryController {
    fn write(&mut self, address: GBAddress, value: u8);
    fn read(&self, address: GBAddress) -> u8;
}