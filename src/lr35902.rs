use std::{thread, time::Duration};

mod instructions;
use instructions::{cb_instructions, instructions, Instruction};

#[allow(dead_code)]
pub struct LR35902 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    mem: [u8; 65536],
    instructions: Vec<Instruction>,
    cb_instructions: Vec<Instruction>,
}

impl Default for LR35902 {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl LR35902 {
    pub fn new() -> Self {
        let mut m = Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            mem: [0; 65536],
            instructions: instructions(),
            cb_instructions: cb_instructions(),
        };
        m.load_bootrom(include_bytes!("../dmg0.bin"));
        m
    }

    fn a(&self) -> u8 {
        ((self.af >> 8) & 0xFF) as u8
    }

    fn b(&self) -> u8 {
        ((self.bc >> 8) & 0xFF) as u8
    }

    fn c(&self) -> u8 {
        (self.bc & 0xFF) as u8
    }

    fn d(&self) -> u8 {
        ((self.de >> 8) & 0xFF) as u8
    }

    fn e(&self) -> u8 {
        (self.de & 0xFF) as u8
    }

    fn h(&self) -> u8 {
        ((self.hl >> 8) & 0xFF) as u8
    }

    fn l(&self) -> u8 {
        (self.hl & 0xFF) as u8
    }

    fn set_a(&mut self, value: u8) {
        self.af = (self.af & 0xFF) | ((value as u16) << 8);
    }

    fn set_b(&mut self, value: u8) {
        self.bc = (self.bc & 0xFF) | ((value as u16) << 8);
    }

    fn set_c(&mut self, value: u8) {
        self.bc = (self.bc & 0xFF00) | (value as u16);
    }

    fn set_d(&mut self, value: u8) {
        self.de = (self.de & 0xFF) | ((value as u16) << 8);
    }

    fn set_e(&mut self, value: u8) {
        self.de = (self.de & 0xFF00) | (value as u16);
    }

    fn set_h(&mut self, value: u8) {
        self.hl = (self.hl & 0xFF) | ((value as u16) << 8);
    }

    fn set_l(&mut self, value: u8) {
        self.hl = (self.hl & 0xFF00) | (value as u16);
    }

    fn set_memory8(&mut self, index: u16, value: u8) {
        self.mem[index as usize] = value;
    }

    fn memory8(&self, index: u16) -> u8 {
        self.mem[index as usize]
    }

    fn load_bootrom(&mut self, bootrom: &[u8; 256]) {
        self.mem[..256].clone_from_slice(bootrom);
    }

    /// get 8 bit immediate at position pc + 1 + pos
    fn get_immediate8(&self, pos: u8) -> u8 {
        self.mem[(self.pc as usize) + (pos as usize) + 1]
    }

    /// get 16 bit immediate at position pc + 1 + pos
    fn get_immediate16(&self, pos: u8) -> u16 {
        // little-endian: the first byte in memory is the LSB
        ((self.get_immediate8(pos + 1) as u16) << 8) + self.get_immediate8(pos) as u16
    }

    pub fn step(&mut self) {
        let opcode = self.mem[self.pc as usize];
        let instruction = self.instructions[opcode as usize].clone();
        println!("{:#02X} {}", instruction.opcode, instruction.mnemonic);
        self.execute(instruction.clone());
        self.pc += instruction.size as u16;
        thread::sleep(Duration::from_micros((instruction.cycles / 4) as u64));
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            0x0 => {}
            0x1 => self.bc = self.get_immediate16(0),
            0x11 => self.de = self.get_immediate16(0),
            0x21 => self.hl = self.get_immediate16(0),
            0x31 => self.sp = self.get_immediate16(0),
            0x32 => {
                self.set_memory8(self.hl, self.a());
                self.hl -= 1
            }
            0x3E => self.set_a(self.get_immediate8(0)),
            0x80 => self.set_a(self.a() + self.b()),
            0xAF => self.set_a(self.a() ^ self.b()),
            0xCB => {
                let cb = self.cb_instructions[self.memory8(self.pc + 1) as usize].clone();
                println!("{:#02X} {}", cb.opcode, cb.mnemonic);
                self.execute_cb(cb)
            },
            0xE2 => self.set_memory8(self.c().into(), self.a()),
            _ => {
                unimplemented!()
            }
        }
    }

    fn execute_cb(&mut self, instruction: Instruction) {
        match instruction.opcode {
            _ => {
                unimplemented!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.a(), 0);
        assert_eq!(cpu.af, 0);

        cpu.set_a(5);
        assert_eq!(cpu.a(), 5);
        assert_eq!(cpu.af, 5 << 8);
    }

    #[test]
    fn test_b() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.b(), 0);
        assert_eq!(cpu.bc, 0);

        cpu.set_b(5);
        assert_eq!(cpu.b(), 5);
        assert_eq!(cpu.bc, 5 << 8);
    }

    #[test]
    fn test_c() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.c(), 0);
        assert_eq!(cpu.bc, 0);

        cpu.set_c(5);
        assert_eq!(cpu.c(), 5);
        assert_eq!(cpu.bc, 5);
    }

    #[test]
    fn test_h() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.h(), 0);
        assert_eq!(cpu.hl, 0);

        cpu.set_h(5);
        assert_eq!(cpu.h(), 5);
        assert_eq!(cpu.hl, 5 << 8);
    }

    #[test]
    fn test_l() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.l(), 0);
        assert_eq!(cpu.hl, 0);

        cpu.set_l(5);
        assert_eq!(cpu.l(), 5);
        assert_eq!(cpu.hl, 5);
    }

    #[test]
    fn test_d() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.d(), 0);
        assert_eq!(cpu.de, 0);

        cpu.set_d(5);
        assert_eq!(cpu.d(), 5);
        assert_eq!(cpu.de, 5 << 8);
    }

    #[test]
    fn test_e() {
        let mut cpu = LR35902::new();

        assert_eq!(cpu.e(), 0);
        assert_eq!(cpu.de, 0);

        cpu.set_e(5);
        assert_eq!(cpu.e(), 5);
        assert_eq!(cpu.de, 5);
    }

    #[test]
    fn test_immediate8() {
        let mut cpu = LR35902::new();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.get_immediate8(0), 2);
    }

    #[test]
    fn test_immediate16() {
        let mut cpu = LR35902::new();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.get_immediate16(0), 3 * 256 + 2);
    }

    #[test]
    fn test_memory() {
        let mut cpu = LR35902::new();

        cpu.set_memory8(10, 255);
        assert_eq!(cpu.memory8(10), 255);
    }
}
