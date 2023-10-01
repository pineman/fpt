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
    next_cb: bool,
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
            next_cb: false,
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

    fn z_flag(&self) -> bool {
        self.af & 0b10000000 == 0b10000000
    }
    fn n_flag(&self) -> bool {
        self.af & 0b01000000 == 0b01000000
    }
    fn h_flag(&self) -> bool {
        self.af & 0b00100000 == 0b00100000
    }
    fn c_flag(&self) -> bool {
        self.af & 0b00010000 == 0b00010000
    }

    fn set_z_flag(&mut self, value: bool) {
        if value {
            self.af = self.af | 0b10000000;
        } else {
            self.af = self.af & 0b01111111;
        }
    }

    fn set_n_flag(&mut self, value: bool) {
        if value {
            self.af = self.af | 0b01000000;
        } else {
            self.af = self.af & 0b10111111;
        }
    }

    fn set_h_flag(&mut self, value: bool) {
        if value {
            self.af = self.af | 0b00100000;
        } else {
            self.af = self.af & 0b11011111;
        }
    }

    fn set_c_flag(&mut self, value: bool) {
        if value {
            self.af = self.af | 0b00010000;
        } else {
            self.af = self.af & 0b11101111;
        }
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
        let instruction = self.get_instruction();
        println!("{:#02X} {}", instruction.opcode, instruction.mnemonic);
        self.run_instruction(instruction.clone());
        self.inc_pc(instruction.clone());
        thread::sleep(Duration::from_micros((instruction.cycles / 4) as u64));
        // TODO: measure time and panic if cycle time exceeded
    }

    fn inc_pc(&mut self, instruction: Instruction) {
        if self.next_cb {
            self.pc += 1;
        } else {
            self.pc += instruction.size as u16;
        }
    }

    fn get_instruction(&mut self) -> Instruction {
        let opcode = self.mem[self.pc as usize];
        if self.next_cb {
            self.cb_instructions[opcode as usize].clone()
        } else {
            self.instructions[opcode as usize].clone()
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        // TODO: this function could return the pc offset for jumps
        if self.next_cb {
            self.execute_cb(instruction.clone())
        } else {
            self.execute(instruction.clone())
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            0x0 => unimplemented!(),
            0x1 => self.bc = self.get_immediate16(0),
            0x2 => unimplemented!(),
            0x3 => unimplemented!(),
            0x4 => unimplemented!(),
            0x5 => unimplemented!(),
            0x6 => unimplemented!(),
            0x7 => unimplemented!(),
            0x8 => unimplemented!(),
            0x9 => unimplemented!(),
            0xA => unimplemented!(),
            0xB => unimplemented!(),
            0xC => unimplemented!(),
            0xD => unimplemented!(),
            0xE => unimplemented!(),
            0xF => unimplemented!(),
            0x10 => unimplemented!(),
            0x11 => self.de = self.get_immediate16(0),
            0x12 => unimplemented!(),
            0x13 => unimplemented!(),
            0x14 => unimplemented!(),
            0x15 => unimplemented!(),
            0x16 => unimplemented!(),
            0x17 => unimplemented!(),
            0x18 => unimplemented!(),
            0x19 => unimplemented!(),
            0x1A => unimplemented!(),
            0x1B => unimplemented!(),
            0x1C => unimplemented!(),
            0x1D => unimplemented!(),
            0x1E => unimplemented!(),
            0x1F => unimplemented!(),
            0x20 => unimplemented!(),
            0x21 => self.hl = self.get_immediate16(0),
            0x22 => unimplemented!(),
            0x23 => unimplemented!(),
            0x24 => unimplemented!(),
            0x25 => unimplemented!(),
            0x26 => unimplemented!(),
            0x27 => unimplemented!(),
            0x28 => unimplemented!(),
            0x29 => unimplemented!(),
            0x2A => unimplemented!(),
            0x2B => unimplemented!(),
            0x2C => unimplemented!(),
            0x2D => unimplemented!(),
            0x2E => unimplemented!(),
            0x2F => unimplemented!(),
            0x30 => unimplemented!(),
            0x31 => self.sp = self.get_immediate16(0),
            0x32 => {
                self.set_memory8(self.hl, self.a());
                self.hl -= 1
            }
            0x33 => unimplemented!(),
            0x34 => unimplemented!(),
            0x35 => unimplemented!(),
            0x36 => unimplemented!(),
            0x37 => unimplemented!(),
            0x38 => unimplemented!(),
            0x39 => unimplemented!(),
            0x3A => unimplemented!(),
            0x3B => unimplemented!(),
            0x3C => unimplemented!(),
            0x3D => unimplemented!(),
            0x3E => self.set_a(self.get_immediate8(0)),
            0x3F => unimplemented!(),
            0x40 => unimplemented!(),
            0x41 => unimplemented!(),
            0x42 => unimplemented!(),
            0x43 => unimplemented!(),
            0x44 => unimplemented!(),
            0x45 => unimplemented!(),
            0x46 => unimplemented!(),
            0x47 => unimplemented!(),
            0x48 => unimplemented!(),
            0x49 => unimplemented!(),
            0x4A => unimplemented!(),
            0x4B => unimplemented!(),
            0x4C => unimplemented!(),
            0x4D => unimplemented!(),
            0x4E => unimplemented!(),
            0x4F => unimplemented!(),
            0x50 => unimplemented!(),
            0x51 => unimplemented!(),
            0x52 => unimplemented!(),
            0x53 => unimplemented!(),
            0x54 => unimplemented!(),
            0x55 => unimplemented!(),
            0x56 => unimplemented!(),
            0x57 => unimplemented!(),
            0x58 => unimplemented!(),
            0x59 => unimplemented!(),
            0x5A => unimplemented!(),
            0x5B => unimplemented!(),
            0x5C => unimplemented!(),
            0x5D => unimplemented!(),
            0x5E => unimplemented!(),
            0x5F => unimplemented!(),
            0x60 => unimplemented!(),
            0x61 => unimplemented!(),
            0x62 => unimplemented!(),
            0x63 => unimplemented!(),
            0x64 => unimplemented!(),
            0x65 => unimplemented!(),
            0x66 => unimplemented!(),
            0x67 => unimplemented!(),
            0x68 => unimplemented!(),
            0x69 => unimplemented!(),
            0x6A => unimplemented!(),
            0x6B => unimplemented!(),
            0x6C => unimplemented!(),
            0x6D => unimplemented!(),
            0x6E => unimplemented!(),
            0x6F => unimplemented!(),
            0x70 => unimplemented!(),
            0x71 => unimplemented!(),
            0x72 => unimplemented!(),
            0x73 => unimplemented!(),
            0x74 => unimplemented!(),
            0x75 => unimplemented!(),
            0x76 => unimplemented!(),
            0x77 => unimplemented!(),
            0x78 => unimplemented!(),
            0x79 => unimplemented!(),
            0x7A => unimplemented!(),
            0x7B => unimplemented!(),
            0x7C => unimplemented!(),
            0x7D => unimplemented!(),
            0x7E => unimplemented!(),
            0x7F => unimplemented!(),
            0x80 => self.set_a(self.a() + self.b()),
            0x81 => unimplemented!(),
            0x82 => unimplemented!(),
            0x83 => unimplemented!(),
            0x84 => unimplemented!(),
            0x85 => unimplemented!(),
            0x86 => unimplemented!(),
            0x87 => unimplemented!(),
            0x88 => unimplemented!(),
            0x89 => unimplemented!(),
            0x8A => unimplemented!(),
            0x8B => unimplemented!(),
            0x8C => unimplemented!(),
            0x8D => unimplemented!(),
            0x8E => unimplemented!(),
            0x8F => unimplemented!(),
            0x90 => unimplemented!(),
            0x91 => unimplemented!(),
            0x92 => unimplemented!(),
            0x93 => unimplemented!(),
            0x94 => unimplemented!(),
            0x95 => unimplemented!(),
            0x96 => unimplemented!(),
            0x97 => unimplemented!(),
            0x98 => unimplemented!(),
            0x99 => unimplemented!(),
            0x9A => unimplemented!(),
            0x9B => unimplemented!(),
            0x9C => unimplemented!(),
            0x9D => unimplemented!(),
            0x9E => unimplemented!(),
            0x9F => unimplemented!(),
            0xA0 => unimplemented!(),
            0xA1 => unimplemented!(),
            0xA2 => unimplemented!(),
            0xA3 => unimplemented!(),
            0xA4 => unimplemented!(),
            0xA5 => unimplemented!(),
            0xA6 => unimplemented!(),
            0xA7 => unimplemented!(),
            0xA8 => unimplemented!(),
            0xA9 => unimplemented!(),
            0xAA => unimplemented!(),
            0xAB => unimplemented!(),
            0xAC => unimplemented!(),
            0xAD => unimplemented!(),
            0xAE => unimplemented!(),
            0xAF => self.set_a(self.a() ^ self.b()),
            0xB0 => unimplemented!(),
            0xB1 => unimplemented!(),
            0xB2 => unimplemented!(),
            0xB3 => unimplemented!(),
            0xB4 => unimplemented!(),
            0xB5 => unimplemented!(),
            0xB6 => unimplemented!(),
            0xB7 => unimplemented!(),
            0xB8 => unimplemented!(),
            0xB9 => unimplemented!(),
            0xBA => unimplemented!(),
            0xBB => unimplemented!(),
            0xBC => unimplemented!(),
            0xBD => unimplemented!(),
            0xBE => unimplemented!(),
            0xBF => unimplemented!(),
            0xC0 => unimplemented!(),
            0xC1 => unimplemented!(),
            0xC2 => unimplemented!(),
            0xC3 => unimplemented!(),
            0xC4 => unimplemented!(),
            0xC5 => unimplemented!(),
            0xC6 => unimplemented!(),
            0xC7 => unimplemented!(),
            0xC8 => unimplemented!(),
            0xC9 => unimplemented!(),
            0xCA => unimplemented!(),
            0xCB => self.next_cb = true,
            0xCC => unimplemented!(),
            0xCD => unimplemented!(),
            0xCE => unimplemented!(),
            0xCF => unimplemented!(),
            0xD0 => unimplemented!(),
            0xD1 => unimplemented!(),
            0xD2 => unimplemented!(),
            0xD3 => unimplemented!(),
            0xD4 => unimplemented!(),
            0xD5 => unimplemented!(),
            0xD6 => unimplemented!(),
            0xD7 => unimplemented!(),
            0xD8 => unimplemented!(),
            0xD9 => unimplemented!(),
            0xDA => unimplemented!(),
            0xDB => unimplemented!(),
            0xDC => unimplemented!(),
            0xDD => unimplemented!(),
            0xDE => unimplemented!(),
            0xDF => unimplemented!(),
            0xE0 => unimplemented!(),
            0xE1 => unimplemented!(),
            0xE2 => self.set_memory8(self.c().into(), self.a()),
            0xE3 => unimplemented!(),
            0xE4 => unimplemented!(),
            0xE5 => unimplemented!(),
            0xE6 => unimplemented!(),
            0xE7 => unimplemented!(),
            0xE8 => unimplemented!(),
            0xE9 => unimplemented!(),
            0xEA => unimplemented!(),
            0xEB => unimplemented!(),
            0xEC => unimplemented!(),
            0xED => unimplemented!(),
            0xEE => unimplemented!(),
            0xEF => unimplemented!(),
            0xF0 => unimplemented!(),
            0xF1 => unimplemented!(),
            0xF2 => unimplemented!(),
            0xF3 => unimplemented!(),
            0xF4 => unimplemented!(),
            0xF5 => unimplemented!(),
            0xF6 => unimplemented!(),
            0xF7 => unimplemented!(),
            0xF8 => unimplemented!(),
            0xF9 => unimplemented!(),
            0xFA => unimplemented!(),
            0xFB => unimplemented!(),
            0xFC => unimplemented!(),
            0xFD => unimplemented!(),
            0xFE => unimplemented!(),
            0xFF => unimplemented!(),
        }
    }

    fn execute_cb(&mut self, instruction: Instruction) {
        self.next_cb = false;
        match instruction.opcode {
            0x0 => unimplemented!(),
            0x1 => unimplemented!(),
            0x2 => unimplemented!(),
            0x3 => unimplemented!(),
            0x4 => unimplemented!(),
            0x5 => unimplemented!(),
            0x6 => unimplemented!(),
            0x7 => unimplemented!(),
            0x8 => unimplemented!(),
            0x9 => unimplemented!(),
            0xA => unimplemented!(),
            0xB => unimplemented!(),
            0xC => unimplemented!(),
            0xD => unimplemented!(),
            0xE => unimplemented!(),
            0xF => unimplemented!(),
            0x10 => unimplemented!(),
            0x11 => unimplemented!(),
            0x12 => unimplemented!(),
            0x13 => unimplemented!(),
            0x14 => unimplemented!(),
            0x15 => unimplemented!(),
            0x16 => unimplemented!(),
            0x17 => unimplemented!(),
            0x18 => unimplemented!(),
            0x19 => unimplemented!(),
            0x1A => unimplemented!(),
            0x1B => unimplemented!(),
            0x1C => unimplemented!(),
            0x1D => unimplemented!(),
            0x1E => unimplemented!(),
            0x1F => unimplemented!(),
            0x20 => unimplemented!(),
            0x21 => unimplemented!(),
            0x22 => unimplemented!(),
            0x23 => unimplemented!(),
            0x24 => unimplemented!(),
            0x25 => unimplemented!(),
            0x26 => unimplemented!(),
            0x27 => unimplemented!(),
            0x28 => unimplemented!(),
            0x29 => unimplemented!(),
            0x2A => unimplemented!(),
            0x2B => unimplemented!(),
            0x2C => unimplemented!(),
            0x2D => unimplemented!(),
            0x2E => unimplemented!(),
            0x2F => unimplemented!(),
            0x30 => unimplemented!(),
            0x31 => unimplemented!(),
            0x32 => unimplemented!(),
            0x33 => unimplemented!(),
            0x34 => unimplemented!(),
            0x35 => unimplemented!(),
            0x36 => unimplemented!(),
            0x37 => unimplemented!(),
            0x38 => unimplemented!(),
            0x39 => unimplemented!(),
            0x3A => unimplemented!(),
            0x3B => unimplemented!(),
            0x3C => unimplemented!(),
            0x3D => unimplemented!(),
            0x3E => unimplemented!(),
            0x3F => unimplemented!(),
            0x40 => unimplemented!(),
            0x41 => unimplemented!(),
            0x42 => unimplemented!(),
            0x43 => unimplemented!(),
            0x44 => unimplemented!(),
            0x45 => unimplemented!(),
            0x46 => unimplemented!(),
            0x47 => unimplemented!(),
            0x48 => unimplemented!(),
            0x49 => unimplemented!(),
            0x4A => unimplemented!(),
            0x4B => unimplemented!(),
            0x4C => unimplemented!(),
            0x4D => unimplemented!(),
            0x4E => unimplemented!(),
            0x4F => unimplemented!(),
            0x50 => unimplemented!(),
            0x51 => unimplemented!(),
            0x52 => unimplemented!(),
            0x53 => unimplemented!(),
            0x54 => unimplemented!(),
            0x55 => unimplemented!(),
            0x56 => unimplemented!(),
            0x57 => unimplemented!(),
            0x58 => unimplemented!(),
            0x59 => unimplemented!(),
            0x5A => unimplemented!(),
            0x5B => unimplemented!(),
            0x5C => unimplemented!(),
            0x5D => unimplemented!(),
            0x5E => unimplemented!(),
            0x5F => unimplemented!(),
            0x60 => unimplemented!(),
            0x61 => unimplemented!(),
            0x62 => unimplemented!(),
            0x63 => unimplemented!(),
            0x64 => unimplemented!(),
            0x65 => unimplemented!(),
            0x66 => unimplemented!(),
            0x67 => unimplemented!(),
            0x68 => unimplemented!(),
            0x69 => unimplemented!(),
            0x6A => unimplemented!(),
            0x6B => unimplemented!(),
            0x6C => unimplemented!(),
            0x6D => unimplemented!(),
            0x6E => unimplemented!(),
            0x6F => unimplemented!(),
            0x70 => unimplemented!(),
            0x71 => unimplemented!(),
            0x72 => unimplemented!(),
            0x73 => unimplemented!(),
            0x74 => unimplemented!(),
            0x75 => unimplemented!(),
            0x76 => unimplemented!(),
            0x77 => unimplemented!(),
            0x78 => unimplemented!(),
            0x79 => unimplemented!(),
            0x7A => unimplemented!(),
            0x7B => unimplemented!(),
            // TODO: use hl directly
            0x7C => {
                if self.h() & 0b10000000 == 0 {
                    self.set_z_flag(true);
                }
                self.set_n_flag(false);
                self.set_h_flag(false);
            }
            0x7D => unimplemented!(),
            0x7E => unimplemented!(),
            0x7F => unimplemented!(),
            0x80 => unimplemented!(),
            0x81 => unimplemented!(),
            0x82 => unimplemented!(),
            0x83 => unimplemented!(),
            0x84 => unimplemented!(),
            0x85 => unimplemented!(),
            0x86 => unimplemented!(),
            0x87 => unimplemented!(),
            0x88 => unimplemented!(),
            0x89 => unimplemented!(),
            0x8A => unimplemented!(),
            0x8B => unimplemented!(),
            0x8C => unimplemented!(),
            0x8D => unimplemented!(),
            0x8E => unimplemented!(),
            0x8F => unimplemented!(),
            0x90 => unimplemented!(),
            0x91 => unimplemented!(),
            0x92 => unimplemented!(),
            0x93 => unimplemented!(),
            0x94 => unimplemented!(),
            0x95 => unimplemented!(),
            0x96 => unimplemented!(),
            0x97 => unimplemented!(),
            0x98 => unimplemented!(),
            0x99 => unimplemented!(),
            0x9A => unimplemented!(),
            0x9B => unimplemented!(),
            0x9C => unimplemented!(),
            0x9D => unimplemented!(),
            0x9E => unimplemented!(),
            0x9F => unimplemented!(),
            0xA0 => unimplemented!(),
            0xA1 => unimplemented!(),
            0xA2 => unimplemented!(),
            0xA3 => unimplemented!(),
            0xA4 => unimplemented!(),
            0xA5 => unimplemented!(),
            0xA6 => unimplemented!(),
            0xA7 => unimplemented!(),
            0xA8 => unimplemented!(),
            0xA9 => unimplemented!(),
            0xAA => unimplemented!(),
            0xAB => unimplemented!(),
            0xAC => unimplemented!(),
            0xAD => unimplemented!(),
            0xAE => unimplemented!(),
            0xAF => unimplemented!(),
            0xB0 => unimplemented!(),
            0xB1 => unimplemented!(),
            0xB2 => unimplemented!(),
            0xB3 => unimplemented!(),
            0xB4 => unimplemented!(),
            0xB5 => unimplemented!(),
            0xB6 => unimplemented!(),
            0xB7 => unimplemented!(),
            0xB8 => unimplemented!(),
            0xB9 => unimplemented!(),
            0xBA => unimplemented!(),
            0xBB => unimplemented!(),
            0xBC => unimplemented!(),
            0xBD => unimplemented!(),
            0xBE => unimplemented!(),
            0xBF => unimplemented!(),
            0xC0 => unimplemented!(),
            0xC1 => unimplemented!(),
            0xC2 => unimplemented!(),
            0xC3 => unimplemented!(),
            0xC4 => unimplemented!(),
            0xC5 => unimplemented!(),
            0xC6 => unimplemented!(),
            0xC7 => unimplemented!(),
            0xC8 => unimplemented!(),
            0xC9 => unimplemented!(),
            0xCA => unimplemented!(),
            0xCB => unimplemented!(),
            0xCC => unimplemented!(),
            0xCD => unimplemented!(),
            0xCE => unimplemented!(),
            0xCF => unimplemented!(),
            0xD0 => unimplemented!(),
            0xD1 => unimplemented!(),
            0xD2 => unimplemented!(),
            0xD3 => unimplemented!(),
            0xD4 => unimplemented!(),
            0xD5 => unimplemented!(),
            0xD6 => unimplemented!(),
            0xD7 => unimplemented!(),
            0xD8 => unimplemented!(),
            0xD9 => unimplemented!(),
            0xDA => unimplemented!(),
            0xDB => unimplemented!(),
            0xDC => unimplemented!(),
            0xDD => unimplemented!(),
            0xDE => unimplemented!(),
            0xDF => unimplemented!(),
            0xE0 => unimplemented!(),
            0xE1 => unimplemented!(),
            0xE2 => unimplemented!(),
            0xE3 => unimplemented!(),
            0xE4 => unimplemented!(),
            0xE5 => unimplemented!(),
            0xE6 => unimplemented!(),
            0xE7 => unimplemented!(),
            0xE8 => unimplemented!(),
            0xE9 => unimplemented!(),
            0xEA => unimplemented!(),
            0xEB => unimplemented!(),
            0xEC => unimplemented!(),
            0xED => unimplemented!(),
            0xEE => unimplemented!(),
            0xEF => unimplemented!(),
            0xF0 => unimplemented!(),
            0xF1 => unimplemented!(),
            0xF2 => unimplemented!(),
            0xF3 => unimplemented!(),
            0xF4 => unimplemented!(),
            0xF5 => unimplemented!(),
            0xF6 => unimplemented!(),
            0xF7 => unimplemented!(),
            0xF8 => unimplemented!(),
            0xF9 => unimplemented!(),
            0xFA => unimplemented!(),
            0xFB => unimplemented!(),
            0xFC => unimplemented!(),
            0xFD => unimplemented!(),
            0xFE => unimplemented!(),
            0xFF => unimplemented!(),
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
