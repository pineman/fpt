use std::{thread, time::Duration};

pub mod instructions;

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: u8,
    clocks: u8,
    size: u8,
    function: fn(&mut LR35902, opcode: u8),
}

impl Instruction {
    fn new(opcode: u8, clocks: u8, size: u8, function: fn(&mut LR35902, opcode: u8)) -> Self {
        Self {
            opcode,
            clocks,
            size,
            function,
        }
    }
}

#[allow(dead_code)]
pub struct LR35902 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    pub mem: [u8; 65536],
    instructions: Vec<Instruction>,
    cb_instructions: Vec<Instruction>,
}

impl Default for LR35902 {
    fn default() -> Self {
        Self::new()
    }
}

impl LR35902 {
    pub fn new() -> Self {
        Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            mem: [0; 65536],
            instructions: Vec::new(),
            cb_instructions: Vec::new(),
        }
    }

    pub fn a(&self) -> u8 {
        return ((self.af >> 8) & 0xFF) as u8;
    }

    pub fn b(&self) -> u8 {
        return ((self.bc >> 8) & 0xFF) as u8;
    }

    pub fn c(&self) -> u8 {
        return (self.bc & 0xFF) as u8;
    }

    pub fn h(&self) -> u8 {
        return ((self.hl >> 8) & 0xFF) as u8;
    }

    pub fn set_a(&mut self, value: u8) {
        self.af = (self.af & 0xFF) | ((value as u16) << 8) as u16;
    }

    fn set_memory8(&mut self, index: u16, value: u8) {
        self.mem[index as usize] = value;
    }

    fn memory8(&self, index: u16) -> u8 {
        self.mem[index as usize]
    }

    pub fn load_bootrom(&mut self, bootrom: &[u8; 256]) {
        self.mem[..256].clone_from_slice(bootrom);
    }

    /// load 8 bit immediate at position pc + 1 + pos
    fn immediate8(&self, pos: u8) -> u8 {
        self.mem[(self.pc as usize) + (pos as usize) + 1]
    }

    /// load 16 bit immediate at position pc + 1 + pos
    pub fn immediate16(&self, pos: u8) -> u16 {
        ((self.immediate8(pos) as u16) << 8) + self.immediate8(pos + 1) as u16
    }

    pub fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn load_cb_instructions(&mut self, instructions: Vec<Instruction>) {
        self.cb_instructions = instructions;
    }

    pub fn step(&mut self) {
        let opcode = dbg!(self.mem[self.pc as usize]);

        let instruction = self.instructions[opcode as usize].clone();
        let f = instruction.function;
        f(self, instruction.opcode);
        self.pc += instruction.size as u16;
        thread::sleep(Duration::from_micros((instruction.clocks / 4) as u64));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate8() {
        let mut cpu = LR35902::new();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.immediate8(0), 2);
    }

    #[test]
    fn test_immediate16() {
        let mut cpu = LR35902::new();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.immediate16(0), 2 * 256 + 3);
    }
}
