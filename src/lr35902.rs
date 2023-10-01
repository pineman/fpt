use std::{thread, time::Duration};

mod instructions;
use instructions::{cb_instructions, instructions, Instruction};

use crate::bitwise;

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
        bitwise::get_byte16::<1>(self.af)
    }

    fn b(&self) -> u8 {
        bitwise::get_byte16::<1>(self.bc)
    }

    fn c(&self) -> u8 {
        bitwise::get_byte16::<0>(self.bc)
    }

    fn d(&self) -> u8 {
        bitwise::get_byte16::<1>(self.de)
    }

    fn e(&self) -> u8 {
        bitwise::get_byte16::<0>(self.de)
    }

    fn h(&self) -> u8 {
        bitwise::get_byte16::<1>(self.hl)
    }

    fn l(&self) -> u8 {
        bitwise::get_byte16::<0>(self.hl)
    }

    fn z_flag(&self) -> bool {
        bitwise::test_bit16::<8>(self.af)
    }
    fn n_flag(&self) -> bool {
        bitwise::test_bit16::<7>(self.af)
    }
    fn h_flag(&self) -> bool {
        bitwise::test_bit16::<6>(self.af)
    }
    fn c_flag(&self) -> bool {
        bitwise::test_bit16::<5>(self.af)
    }

    fn set_z_flag(&mut self, value: bool) {
        self.af = bitwise::set_bit16::<8>(self.af, value);
    }

    fn set_n_flag(&mut self, value: bool) {
        self.af = bitwise::set_bit16::<7>(self.af, value);
    }

    fn set_h_flag(&mut self, value: bool) {
        self.af = bitwise::set_bit16::<6>(self.af, value);
    }

    fn set_c_flag(&mut self, value: bool) {
        self.af = bitwise::set_bit16::<5>(self.af, value);
    }

    fn set_a(&mut self, value: u8) {
        self.af = bitwise::set_byte16::<1>(self.af, value);
    }

    fn set_b(&mut self, value: u8) {
        self.bc = bitwise::set_byte16::<1>(self.bc, value);
    }

    fn set_c(&mut self, value: u8) {
        self.bc = bitwise::set_byte16::<0>(self.bc, value);
    }

    fn set_d(&mut self, value: u8) {
        self.de = bitwise::set_byte16::<1>(self.de, value);
    }

    fn set_e(&mut self, value: u8) {
        self.de = bitwise::set_byte16::<0>(self.de, value);
    }

    fn set_h(&mut self, value: u8) {
        self.hl = bitwise::set_byte16::<1>(self.hl, value);
    }

    fn set_l(&mut self, value: u8) {
        self.hl = bitwise::set_byte16::<0>(self.hl, value);
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
            0x0 => {
                // NOP
                unimplemented!()
            }
            0x1 => {
                // LD BC,d16
                self.bc = self.get_immediate16(0);
            }
            0x2 => {
                // LD (BC),A
                unimplemented!()
            }
            0x3 => {
                // INC BC
                unimplemented!()
            }
            0x4 => {
                // INC B
                unimplemented!()
            }
            0x5 => {
                // DEC B
                unimplemented!()
            }
            0x6 => {
                // LD B,d8
                unimplemented!()
            }
            0x7 => {
                // RLCA
                unimplemented!()
            }
            0x8 => {
                // LD (a16),SP
                unimplemented!()
            }
            0x9 => {
                // ADD HL,BC
                unimplemented!()
            }
            0xA => {
                // LD A,(BC)
                unimplemented!()
            }
            0xB => {
                // DEC BC
                unimplemented!()
            }
            0xC => {
                // INC C
                unimplemented!()
            }
            0xD => {
                // DEC C
                unimplemented!()
            }
            0xE => {
                // LD C,d8
                unimplemented!()
            }
            0xF => {
                // RRCA
                unimplemented!()
            }
            0x10 => {
                // STOP 0
                unimplemented!()
            }
            0x11 => {
                // LD DE,d16
                self.de = self.get_immediate16(0);
            }
            0x12 => {
                // LD (DE),A
                unimplemented!()
            }
            0x13 => {
                // INC DE
                unimplemented!()
            }
            0x14 => {
                // INC D
                unimplemented!()
            }
            0x15 => {
                // DEC D
                unimplemented!()
            }
            0x16 => {
                // LD D,d8
                unimplemented!()
            }
            0x17 => {
                // RLA
                unimplemented!()
            }
            0x18 => {
                // JR r8
                unimplemented!()
            }
            0x19 => {
                // ADD HL,DE
                unimplemented!()
            }
            0x1A => {
                // LD A,(DE)
                unimplemented!()
            }
            0x1B => {
                // DEC DE
                unimplemented!()
            }
            0x1C => {
                // INC E
                unimplemented!()
            }
            0x1D => {
                // DEC E
                unimplemented!()
            }
            0x1E => {
                // LD E,d8
                unimplemented!()
            }
            0x1F => {
                // RRA
                unimplemented!()
            }
            0x20 => {
                // JR NZ,r8
                unimplemented!()
            }
            0x21 => {
                // LD HL,d16
                self.hl = self.get_immediate16(0);
            }
            0x22 => {
                // LD (HL+),A
                unimplemented!()
            }
            0x23 => {
                // INC HL
                unimplemented!()
            }
            0x24 => {
                // INC H
                unimplemented!()
            }
            0x25 => {
                // DEC H
                unimplemented!()
            }
            0x26 => {
                // LD H,d8
                unimplemented!()
            }
            0x27 => {
                // DAA
                unimplemented!()
            }
            0x28 => {
                // JR Z,r8
                unimplemented!()
            }
            0x29 => {
                // ADD HL,HL
                unimplemented!()
            }
            0x2A => {
                // LD A,(HL+)
                unimplemented!()
            }
            0x2B => {
                // DEC HL
                unimplemented!()
            }
            0x2C => {
                // INC L
                unimplemented!()
            }
            0x2D => {
                // DEC L
                unimplemented!()
            }
            0x2E => {
                // LD L,d8
                unimplemented!()
            }
            0x2F => {
                // CPL
                unimplemented!()
            }
            0x30 => {
                // JR NC,r8
                unimplemented!()
            }
            0x31 => {
                // LD SP,d16
                self.sp = self.get_immediate16(0);
            }
            0x32 => {
                // LD (HL-),A
                self.set_memory8(self.hl, self.a());
                self.hl -= 1
            }
            0x33 => {
                // INC SP
                unimplemented!()
            }
            0x34 => {
                // INC (HL)
                unimplemented!()
            }
            0x35 => {
                // DEC (HL)
                unimplemented!()
            }
            0x36 => {
                // LD (HL),d8
                unimplemented!()
            }
            0x37 => {
                // SCF
                unimplemented!()
            }
            0x38 => {
                // JR C,r8
                unimplemented!()
            }
            0x39 => {
                // ADD HL,SP
                unimplemented!()
            }
            0x3A => {
                // LD A,(HL-)
                unimplemented!()
            }
            0x3B => {
                // DEC SP
                unimplemented!()
            }
            0x3C => {
                // INC A
                unimplemented!()
            }
            0x3D => {
                // DEC A
                unimplemented!()
            }
            0x3E => {
                // LD A,d8
                self.set_a(self.get_immediate8(0));
            }
            0x3F => {
                // CCF
                unimplemented!()
            }
            0x40 => {
                // LD B,B
                unimplemented!()
            }
            0x41 => {
                // LD B,C
                unimplemented!()
            }
            0x42 => {
                // LD B,D
                unimplemented!()
            }
            0x43 => {
                // LD B,E
                unimplemented!()
            }
            0x44 => {
                // LD B,H
                unimplemented!()
            }
            0x45 => {
                // LD B,L
                unimplemented!()
            }
            0x46 => {
                // LD B,(HL)
                unimplemented!()
            }
            0x47 => {
                // LD B,A
                unimplemented!()
            }
            0x48 => {
                // LD C,B
                unimplemented!()
            }
            0x49 => {
                // LD C,C
                unimplemented!()
            }
            0x4A => {
                // LD C,D
                unimplemented!()
            }
            0x4B => {
                // LD C,E
                unimplemented!()
            }
            0x4C => {
                // LD C,H
                unimplemented!()
            }
            0x4D => {
                // LD C,L
                unimplemented!()
            }
            0x4E => {
                // LD C,(HL)
                unimplemented!()
            }
            0x4F => {
                // LD C,A
                unimplemented!()
            }
            0x50 => {
                // LD D,B
                unimplemented!()
            }
            0x51 => {
                // LD D,C
                unimplemented!()
            }
            0x52 => {
                // LD D,D
                unimplemented!()
            }
            0x53 => {
                // LD D,E
                unimplemented!()
            }
            0x54 => {
                // LD D,H
                unimplemented!()
            }
            0x55 => {
                // LD D,L
                unimplemented!()
            }
            0x56 => {
                // LD D,(HL)
                unimplemented!()
            }
            0x57 => {
                // LD D,A
                unimplemented!()
            }
            0x58 => {
                // LD E,B
                unimplemented!()
            }
            0x59 => {
                // LD E,C
                unimplemented!()
            }
            0x5A => {
                // LD E,D
                unimplemented!()
            }
            0x5B => {
                // LD E,E
                unimplemented!()
            }
            0x5C => {
                // LD E,H
                unimplemented!()
            }
            0x5D => {
                // LD E,L
                unimplemented!()
            }
            0x5E => {
                // LD E,(HL)
                unimplemented!()
            }
            0x5F => {
                // LD E,A
                unimplemented!()
            }
            0x60 => {
                // LD H,B
                unimplemented!()
            }
            0x61 => {
                // LD H,C
                unimplemented!()
            }
            0x62 => {
                // LD H,D
                unimplemented!()
            }
            0x63 => {
                // LD H,E
                unimplemented!()
            }
            0x64 => {
                // LD H,H
                unimplemented!()
            }
            0x65 => {
                // LD H,L
                unimplemented!()
            }
            0x66 => {
                // LD H,(HL)
                unimplemented!()
            }
            0x67 => {
                // LD H,A
                unimplemented!()
            }
            0x68 => {
                // LD L,B
                unimplemented!()
            }
            0x69 => {
                // LD L,C
                unimplemented!()
            }
            0x6A => {
                // LD L,D
                unimplemented!()
            }
            0x6B => {
                // LD L,E
                unimplemented!()
            }
            0x6C => {
                // LD L,H
                unimplemented!()
            }
            0x6D => {
                // LD L,L
                unimplemented!()
            }
            0x6E => {
                // LD L,(HL)
                unimplemented!()
            }
            0x6F => {
                // LD L,A
                unimplemented!()
            }
            0x70 => {
                // LD (HL),B
                unimplemented!()
            }
            0x71 => {
                // LD (HL),C
                unimplemented!()
            }
            0x72 => {
                // LD (HL),D
                unimplemented!()
            }
            0x73 => {
                // LD (HL),E
                unimplemented!()
            }
            0x74 => {
                // LD (HL),H
                unimplemented!()
            }
            0x75 => {
                // LD (HL),L
                unimplemented!()
            }
            0x76 => {
                // HALT
                unimplemented!()
            }
            0x77 => {
                // LD (HL),A
                unimplemented!()
            }
            0x78 => {
                // LD A,B
                unimplemented!()
            }
            0x79 => {
                // LD A,C
                unimplemented!()
            }
            0x7A => {
                // LD A,D
                unimplemented!()
            }
            0x7B => {
                // LD A,E
                unimplemented!()
            }
            0x7C => {
                // LD A,H
                unimplemented!()
            }
            0x7D => {
                // LD A,L
                unimplemented!()
            }
            0x7E => {
                // LD A,(HL)
                unimplemented!()
            }
            0x7F => {
                // LD A,A
                unimplemented!()
            }
            0x80 => {
                // ADD A,B
                self.set_a(self.a() + self.b());
            }
            0x81 => {
                // ADD A,C
                unimplemented!()
            }
            0x82 => {
                // ADD A,D
                unimplemented!()
            }
            0x83 => {
                // ADD A,E
                unimplemented!()
            }
            0x84 => {
                // ADD A,H
                unimplemented!()
            }
            0x85 => {
                // ADD A,L
                unimplemented!()
            }
            0x86 => {
                // ADD A,(HL)
                unimplemented!()
            }
            0x87 => {
                // ADD A,A
                unimplemented!()
            }
            0x88 => {
                // ADC A,B
                unimplemented!()
            }
            0x89 => {
                // ADC A,C
                unimplemented!()
            }
            0x8A => {
                // ADC A,D
                unimplemented!()
            }
            0x8B => {
                // ADC A,E
                unimplemented!()
            }
            0x8C => {
                // ADC A,H
                unimplemented!()
            }
            0x8D => {
                // ADC A,L
                unimplemented!()
            }
            0x8E => {
                // ADC A,(HL)
                unimplemented!()
            }
            0x8F => {
                // ADC A,A
                unimplemented!()
            }
            0x90 => {
                // SUB B
                unimplemented!()
            }
            0x91 => {
                // SUB C
                unimplemented!()
            }
            0x92 => {
                // SUB D
                unimplemented!()
            }
            0x93 => {
                // SUB E
                unimplemented!()
            }
            0x94 => {
                // SUB H
                unimplemented!()
            }
            0x95 => {
                // SUB L
                unimplemented!()
            }
            0x96 => {
                // SUB (HL)
                unimplemented!()
            }
            0x97 => {
                // SUB A
                unimplemented!()
            }
            0x98 => {
                // SBC A,B
                unimplemented!()
            }
            0x99 => {
                // SBC A,C
                unimplemented!()
            }
            0x9A => {
                // SBC A,D
                unimplemented!()
            }
            0x9B => {
                // SBC A,E
                unimplemented!()
            }
            0x9C => {
                // SBC A,H
                unimplemented!()
            }
            0x9D => {
                // SBC A,L
                unimplemented!()
            }
            0x9E => {
                // SBC A,(HL)
                unimplemented!()
            }
            0x9F => {
                // SBC A,A
                unimplemented!()
            }
            0xA0 => {
                // AND B
                unimplemented!()
            }
            0xA1 => {
                // AND C
                unimplemented!()
            }
            0xA2 => {
                // AND D
                unimplemented!()
            }
            0xA3 => {
                // AND E
                unimplemented!()
            }
            0xA4 => {
                // AND H
                unimplemented!()
            }
            0xA5 => {
                // AND L
                unimplemented!()
            }
            0xA6 => {
                // AND (HL)
                unimplemented!()
            }
            0xA7 => {
                // AND A
                unimplemented!()
            }
            0xA8 => {
                // XOR B
                unimplemented!()
            }
            0xA9 => {
                // XOR C
                unimplemented!()
            }
            0xAA => {
                // XOR D
                unimplemented!()
            }
            0xAB => {
                // XOR E
                unimplemented!()
            }
            0xAC => {
                // XOR H
                unimplemented!()
            }
            0xAD => {
                // XOR L
                unimplemented!()
            }
            0xAE => {
                // XOR (HL)
                unimplemented!()
            }
            0xAF => {
                // XOR A
                self.set_a(self.a() ^ self.b());
            }
            0xB0 => {
                // OR B
                unimplemented!()
            }
            0xB1 => {
                // OR C
                unimplemented!()
            }
            0xB2 => {
                // OR D
                unimplemented!()
            }
            0xB3 => {
                // OR E
                unimplemented!()
            }
            0xB4 => {
                // OR H
                unimplemented!()
            }
            0xB5 => {
                // OR L
                unimplemented!()
            }
            0xB6 => {
                // OR (HL)
                unimplemented!()
            }
            0xB7 => {
                // OR A
                unimplemented!()
            }
            0xB8 => {
                // CP B
                unimplemented!()
            }
            0xB9 => {
                // CP C
                unimplemented!()
            }
            0xBA => {
                // CP D
                unimplemented!()
            }
            0xBB => {
                // CP E
                unimplemented!()
            }
            0xBC => {
                // CP H
                unimplemented!()
            }
            0xBD => {
                // CP L
                unimplemented!()
            }
            0xBE => {
                // CP (HL)
                unimplemented!()
            }
            0xBF => {
                // CP A
                unimplemented!()
            }
            0xC0 => {
                // RET NZ
                unimplemented!()
            }
            0xC1 => {
                // POP BC
                unimplemented!()
            }
            0xC2 => {
                // JP NZ,a16
                unimplemented!()
            }
            0xC3 => {
                // JP a16
                unimplemented!()
            }
            0xC4 => {
                // CALL NZ,a16
                unimplemented!()
            }
            0xC5 => {
                // PUSH BC
                unimplemented!()
            }
            0xC6 => {
                // ADD A,d8
                unimplemented!()
            }
            0xC7 => {
                // RST 00H
                unimplemented!()
            }
            0xC8 => {
                // RET Z
                unimplemented!()
            }
            0xC9 => {
                // RET
                unimplemented!()
            }
            0xCA => {
                // JP Z,a16
                unimplemented!()
            }
            0xCB => {
                // PREFIX CB
                self.next_cb = true;
            }
            0xCC => {
                // CALL Z,a16
                unimplemented!()
            }
            0xCD => {
                // CALL a16
                unimplemented!()
            }
            0xCE => {
                // ADC A,d8
                unimplemented!()
            }
            0xCF => {
                // RST 08H
                unimplemented!()
            }
            0xD0 => {
                // RET NC
                unimplemented!()
            }
            0xD1 => {
                // POP DE
                unimplemented!()
            }
            0xD2 => {
                // JP NC,a16
                unimplemented!()
            }
            0xD3 => {
                // NOTHING
                unimplemented!()
            }
            0xD4 => {
                // CALL NC,a16
                unimplemented!()
            }
            0xD5 => {
                // PUSH DE
                unimplemented!()
            }
            0xD6 => {
                // SUB d8
                unimplemented!()
            }
            0xD7 => {
                // RST 10H
                unimplemented!()
            }
            0xD8 => {
                // RET C
                unimplemented!()
            }
            0xD9 => {
                // RETI
                unimplemented!()
            }
            0xDA => {
                // JP C,a16
                unimplemented!()
            }
            0xDB => {
                // NOTHING
                unimplemented!()
            }
            0xDC => {
                // CALL C,a16
                unimplemented!()
            }
            0xDD => {
                // NOTHING
                unimplemented!()
            }
            0xDE => {
                // SBC A,d8
                unimplemented!()
            }
            0xDF => {
                // RST 18H
                unimplemented!()
            }
            0xE0 => {
                // LDH (a8),A
                unimplemented!()
            }
            0xE1 => {
                // POP HL
                unimplemented!()
            }
            0xE2 => {
                // LD (C),A
                self.set_memory8(self.c().into(), self.a());
            }
            0xE3 => {
                // NOTHING
                unimplemented!()
            }
            0xE4 => {
                // NOTHING
                unimplemented!()
            }
            0xE5 => {
                // PUSH HL
                unimplemented!()
            }
            0xE6 => {
                // AND d8
                unimplemented!()
            }
            0xE7 => {
                // RST 20H
                unimplemented!()
            }
            0xE8 => {
                // ADD SP,r8
                unimplemented!()
            }
            0xE9 => {
                // JP (HL)
                unimplemented!()
            }
            0xEA => {
                // LD (a16),A
                unimplemented!()
            }
            0xEB => {
                // NOTHING
                unimplemented!()
            }
            0xEC => {
                // NOTHING
                unimplemented!()
            }
            0xED => {
                // NOTHING
                unimplemented!()
            }
            0xEE => {
                // XOR d8
                unimplemented!()
            }
            0xEF => {
                // RST 28H
                unimplemented!()
            }
            0xF0 => {
                // LDH A,(a8)
                unimplemented!()
            }
            0xF1 => {
                // POP AF
                unimplemented!()
            }
            0xF2 => {
                // LD A,(C)
                unimplemented!()
            }
            0xF3 => {
                // DI
                unimplemented!()
            }
            0xF4 => {
                // NOTHING
                unimplemented!()
            }
            0xF5 => {
                // PUSH AF
                unimplemented!()
            }
            0xF6 => {
                // OR d8
                unimplemented!()
            }
            0xF7 => {
                // RST 30H
                unimplemented!()
            }
            0xF8 => {
                // LD HL,SP+r8
                unimplemented!()
            }
            0xF9 => {
                // LD SP,HL
                unimplemented!()
            }
            0xFA => {
                // LD A,(a16)
                unimplemented!()
            }
            0xFB => {
                // EI
                unimplemented!()
            }
            0xFC => {
                // NOTHING
                unimplemented!()
            }
            0xFD => {
                // NOTHING
                unimplemented!()
            }
            0xFE => {
                // CP d8
                unimplemented!()
            }
            0xFF => {
                // RST 38H
                unimplemented!()
            }
        }
    }

    fn execute_cb(&mut self, instruction: Instruction) {
        self.next_cb = false;
        match instruction.opcode {
            0x0 => {
                // RLC B
                unimplemented!()
            }
            0x1 => {
                // RLC C
                unimplemented!()
            }
            0x2 => {
                // RLC D
                unimplemented!()
            }
            0x3 => {
                // RLC E
                unimplemented!()
            }
            0x4 => {
                // RLC H
                unimplemented!()
            }
            0x5 => {
                // RLC L
                unimplemented!()
            }
            0x6 => {
                // RLC (HL)
                unimplemented!()
            }
            0x7 => {
                // RLC A
                unimplemented!()
            }
            0x8 => {
                // RRC B
                unimplemented!()
            }
            0x9 => {
                // RRC C
                unimplemented!()
            }
            0xA => {
                // RRC D
                unimplemented!()
            }
            0xB => {
                // RRC E
                unimplemented!()
            }
            0xC => {
                // RRC H
                unimplemented!()
            }
            0xD => {
                // RRC L
                unimplemented!()
            }
            0xE => {
                // RRC (HL)
                unimplemented!()
            }
            0xF => {
                // RRC A
                unimplemented!()
            }
            0x10 => {
                // RL B
                unimplemented!()
            }
            0x11 => {
                // RL C
                unimplemented!()
            }
            0x12 => {
                // RL D
                unimplemented!()
            }
            0x13 => {
                // RL E
                unimplemented!()
            }
            0x14 => {
                // RL H
                unimplemented!()
            }
            0x15 => {
                // RL L
                unimplemented!()
            }
            0x16 => {
                // RL (HL)
                unimplemented!()
            }
            0x17 => {
                // RL A
                unimplemented!()
            }
            0x18 => {
                // RR B
                unimplemented!()
            }
            0x19 => {
                // RR C
                unimplemented!()
            }
            0x1A => {
                // RR D
                unimplemented!()
            }
            0x1B => {
                // RR E
                unimplemented!()
            }
            0x1C => {
                // RR H
                unimplemented!()
            }
            0x1D => {
                // RR L
                unimplemented!()
            }
            0x1E => {
                // RR (HL)
                unimplemented!()
            }
            0x1F => {
                // RR A
                unimplemented!()
            }
            0x20 => {
                // SLA B
                unimplemented!()
            }
            0x21 => {
                // SLA C
                unimplemented!()
            }
            0x22 => {
                // SLA D
                unimplemented!()
            }
            0x23 => {
                // SLA E
                unimplemented!()
            }
            0x24 => {
                // SLA H
                unimplemented!()
            }
            0x25 => {
                // SLA L
                unimplemented!()
            }
            0x26 => {
                // SLA (HL)
                unimplemented!()
            }
            0x27 => {
                // SLA A
                unimplemented!()
            }
            0x28 => {
                // SRA B
                unimplemented!()
            }
            0x29 => {
                // SRA C
                unimplemented!()
            }
            0x2A => {
                // SRA D
                unimplemented!()
            }
            0x2B => {
                // SRA E
                unimplemented!()
            }
            0x2C => {
                // SRA H
                unimplemented!()
            }
            0x2D => {
                // SRA L
                unimplemented!()
            }
            0x2E => {
                // SRA (HL)
                unimplemented!()
            }
            0x2F => {
                // SRA A
                unimplemented!()
            }
            0x30 => {
                // SWAP B
                unimplemented!()
            }
            0x31 => {
                // SWAP C
                unimplemented!()
            }
            0x32 => {
                // SWAP D
                unimplemented!()
            }
            0x33 => {
                // SWAP E
                unimplemented!()
            }
            0x34 => {
                // SWAP H
                unimplemented!()
            }
            0x35 => {
                // SWAP L
                unimplemented!()
            }
            0x36 => {
                // SWAP (HL)
                unimplemented!()
            }
            0x37 => {
                // SWAP A
                unimplemented!()
            }
            0x38 => {
                // SRL B
                unimplemented!()
            }
            0x39 => {
                // SRL C
                unimplemented!()
            }
            0x3A => {
                // SRL D
                unimplemented!()
            }
            0x3B => {
                // SRL E
                unimplemented!()
            }
            0x3C => {
                // SRL H
                unimplemented!()
            }
            0x3D => {
                // SRL L
                unimplemented!()
            }
            0x3E => {
                // SRL (HL)
                unimplemented!()
            }
            0x3F => {
                // SRL A
                unimplemented!()
            }
            0x40 => {
                // BIT 0,B
                unimplemented!()
            }
            0x41 => {
                // BIT 0,C
                unimplemented!()
            }
            0x42 => {
                // BIT 0,D
                unimplemented!()
            }
            0x43 => {
                // BIT 0,E
                unimplemented!()
            }
            0x44 => {
                // BIT 0,H
                unimplemented!()
            }
            0x45 => {
                // BIT 0,L
                unimplemented!()
            }
            0x46 => {
                // BIT 0,(HL)
                unimplemented!()
            }
            0x47 => {
                // BIT 0,A
                unimplemented!()
            }
            0x48 => {
                // BIT 1,B
                unimplemented!()
            }
            0x49 => {
                // BIT 1,C
                unimplemented!()
            }
            0x4A => {
                // BIT 1,D
                unimplemented!()
            }
            0x4B => {
                // BIT 1,E
                unimplemented!()
            }
            0x4C => {
                // BIT 1,H
                unimplemented!()
            }
            0x4D => {
                // BIT 1,L
                unimplemented!()
            }
            0x4E => {
                // BIT 1,(HL)
                unimplemented!()
            }
            0x4F => {
                // BIT 1,A
                unimplemented!()
            }
            0x50 => {
                // BIT 2,B
                unimplemented!()
            }
            0x51 => {
                // BIT 2,C
                unimplemented!()
            }
            0x52 => {
                // BIT 2,D
                unimplemented!()
            }
            0x53 => {
                // BIT 2,E
                unimplemented!()
            }
            0x54 => {
                // BIT 2,H
                unimplemented!()
            }
            0x55 => {
                // BIT 2,L
                unimplemented!()
            }
            0x56 => {
                // BIT 2,(HL)
                unimplemented!()
            }
            0x57 => {
                // BIT 2,A
                unimplemented!()
            }
            0x58 => {
                // BIT 3,B
                unimplemented!()
            }
            0x59 => {
                // BIT 3,C
                unimplemented!()
            }
            0x5A => {
                // BIT 3,D
                unimplemented!()
            }
            0x5B => {
                // BIT 3,E
                unimplemented!()
            }
            0x5C => {
                // BIT 3,H
                unimplemented!()
            }
            0x5D => {
                // BIT 3,L
                unimplemented!()
            }
            0x5E => {
                // BIT 3,(HL)
                unimplemented!()
            }
            0x5F => {
                // BIT 3,A
                unimplemented!()
            }
            0x60 => {
                // BIT 4,B
                unimplemented!()
            }
            0x61 => {
                // BIT 4,C
                unimplemented!()
            }
            0x62 => {
                // BIT 4,D
                unimplemented!()
            }
            0x63 => {
                // BIT 4,E
                unimplemented!()
            }
            0x64 => {
                // BIT 4,H
                unimplemented!()
            }
            0x65 => {
                // BIT 4,L
                unimplemented!()
            }
            0x66 => {
                // BIT 4,(HL)
                unimplemented!()
            }
            0x67 => {
                // BIT 4,A
                unimplemented!()
            }
            0x68 => {
                // BIT 5,B
                unimplemented!()
            }
            0x69 => {
                // BIT 5,C
                unimplemented!()
            }
            0x6A => {
                // BIT 5,D
                unimplemented!()
            }
            0x6B => {
                // BIT 5,E
                unimplemented!()
            }
            0x6C => {
                // BIT 5,H
                unimplemented!()
            }
            0x6D => {
                // BIT 5,L
                unimplemented!()
            }
            0x6E => {
                // BIT 5,(HL)
                unimplemented!()
            }
            0x6F => {
                // BIT 5,A
                unimplemented!()
            }
            0x70 => {
                // BIT 6,B
                unimplemented!()
            }
            0x71 => {
                // BIT 6,C
                unimplemented!()
            }
            0x72 => {
                // BIT 6,D
                unimplemented!()
            }
            0x73 => {
                // BIT 6,E
                unimplemented!()
            }
            0x74 => {
                // BIT 6,H
                unimplemented!()
            }
            0x75 => {
                // BIT 6,L
                unimplemented!()
            }
            0x76 => {
                // BIT 6,(HL)
                unimplemented!()
            }
            0x77 => {
                // BIT 6,A
                unimplemented!()
            }
            0x78 => {
                // BIT 7,B
                unimplemented!()
            }
            0x79 => {
                // BIT 7,C
                unimplemented!()
            }
            0x7A => {
                // BIT 7,D
                unimplemented!()
            }
            0x7B => {
                // BIT 7,E
                unimplemented!()
            }
            0x7C => {
                // BIT 7,H
                // TODO: use hl directly
                if self.h() & 0b10000000 == 0 {
                    self.set_z_flag(true);
                }
                self.set_n_flag(false);
                self.set_h_flag(false);
            }
            0x7D => {
                // BIT 7,L
                unimplemented!()
            }
            0x7E => {
                // BIT 7,(HL)
                unimplemented!()
            }
            0x7F => {
                // BIT 7,A
                unimplemented!()
            }
            0x80 => {
                // RES 0,B
                unimplemented!()
            }
            0x81 => {
                // RES 0,C
                unimplemented!()
            }
            0x82 => {
                // RES 0,D
                unimplemented!()
            }
            0x83 => {
                // RES 0,E
                unimplemented!()
            }
            0x84 => {
                // RES 0,H
                unimplemented!()
            }
            0x85 => {
                // RES 0,L
                unimplemented!()
            }
            0x86 => {
                // RES 0,(HL)
                unimplemented!()
            }
            0x87 => {
                // RES 0,A
                unimplemented!()
            }
            0x88 => {
                // RES 1,B
                unimplemented!()
            }
            0x89 => {
                // RES 1,C
                unimplemented!()
            }
            0x8A => {
                // RES 1,D
                unimplemented!()
            }
            0x8B => {
                // RES 1,E
                unimplemented!()
            }
            0x8C => {
                // RES 1,H
                unimplemented!()
            }
            0x8D => {
                // RES 1,L
                unimplemented!()
            }
            0x8E => {
                // RES 1,(HL)
                unimplemented!()
            }
            0x8F => {
                // RES 1,A
                unimplemented!()
            }
            0x90 => {
                // RES 2,B
                unimplemented!()
            }
            0x91 => {
                // RES 2,C
                unimplemented!()
            }
            0x92 => {
                // RES 2,D
                unimplemented!()
            }
            0x93 => {
                // RES 2,E
                unimplemented!()
            }
            0x94 => {
                // RES 2,H
                unimplemented!()
            }
            0x95 => {
                // RES 2,L
                unimplemented!()
            }
            0x96 => {
                // RES 2,(HL)
                unimplemented!()
            }
            0x97 => {
                // RES 2,A
                unimplemented!()
            }
            0x98 => {
                // RES 3,B
                unimplemented!()
            }
            0x99 => {
                // RES 3,C
                unimplemented!()
            }
            0x9A => {
                // RES 3,D
                unimplemented!()
            }
            0x9B => {
                // RES 3,E
                unimplemented!()
            }
            0x9C => {
                // RES 3,H
                unimplemented!()
            }
            0x9D => {
                // RES 3,L
                unimplemented!()
            }
            0x9E => {
                // RES 3,(HL)
                unimplemented!()
            }
            0x9F => {
                // RES 3,A
                unimplemented!()
            }
            0xA0 => {
                // RES 4,B
                unimplemented!()
            }
            0xA1 => {
                // RES 4,C
                unimplemented!()
            }
            0xA2 => {
                // RES 4,D
                unimplemented!()
            }
            0xA3 => {
                // RES 4,E
                unimplemented!()
            }
            0xA4 => {
                // RES 4,H
                unimplemented!()
            }
            0xA5 => {
                // RES 4,L
                unimplemented!()
            }
            0xA6 => {
                // RES 4,(HL)
                unimplemented!()
            }
            0xA7 => {
                // RES 4,A
                unimplemented!()
            }
            0xA8 => {
                // RES 5,B
                unimplemented!()
            }
            0xA9 => {
                // RES 5,C
                unimplemented!()
            }
            0xAA => {
                // RES 5,D
                unimplemented!()
            }
            0xAB => {
                // RES 5,E
                unimplemented!()
            }
            0xAC => {
                // RES 5,H
                unimplemented!()
            }
            0xAD => {
                // RES 5,L
                unimplemented!()
            }
            0xAE => {
                // RES 5,(HL)
                unimplemented!()
            }
            0xAF => {
                // RES 5,A
                unimplemented!()
            }
            0xB0 => {
                // RES 6,B
                unimplemented!()
            }
            0xB1 => {
                // RES 6,C
                unimplemented!()
            }
            0xB2 => {
                // RES 6,D
                unimplemented!()
            }
            0xB3 => {
                // RES 6,E
                unimplemented!()
            }
            0xB4 => {
                // RES 6,H
                unimplemented!()
            }
            0xB5 => {
                // RES 6,L
                unimplemented!()
            }
            0xB6 => {
                // RES 6,(HL)
                unimplemented!()
            }
            0xB7 => {
                // RES 6,A
                unimplemented!()
            }
            0xB8 => {
                // RES 7,B
                unimplemented!()
            }
            0xB9 => {
                // RES 7,C
                unimplemented!()
            }
            0xBA => {
                // RES 7,D
                unimplemented!()
            }
            0xBB => {
                // RES 7,E
                unimplemented!()
            }
            0xBC => {
                // RES 7,H
                unimplemented!()
            }
            0xBD => {
                // RES 7,L
                unimplemented!()
            }
            0xBE => {
                // RES 7,(HL)
                unimplemented!()
            }
            0xBF => {
                // RES 7,A
                unimplemented!()
            }
            0xC0 => {
                // SET 0,B
                unimplemented!()
            }
            0xC1 => {
                // SET 0,C
                unimplemented!()
            }
            0xC2 => {
                // SET 0,D
                unimplemented!()
            }
            0xC3 => {
                // SET 0,E
                unimplemented!()
            }
            0xC4 => {
                // SET 0,H
                unimplemented!()
            }
            0xC5 => {
                // SET 0,L
                unimplemented!()
            }
            0xC6 => {
                // SET 0,(HL)
                unimplemented!()
            }
            0xC7 => {
                // SET 0,A
                unimplemented!()
            }
            0xC8 => {
                // SET 1,B
                unimplemented!()
            }
            0xC9 => {
                // SET 1,C
                unimplemented!()
            }
            0xCA => {
                // SET 1,D
                unimplemented!()
            }
            0xCB => {
                // SET 1,E
                unimplemented!()
            }
            0xCC => {
                // SET 1,H
                unimplemented!()
            }
            0xCD => {
                // SET 1,L
                unimplemented!()
            }
            0xCE => {
                // SET 1,(HL)
                unimplemented!()
            }
            0xCF => {
                // SET 1,A
                unimplemented!()
            }
            0xD0 => {
                // SET 2,B
                unimplemented!()
            }
            0xD1 => {
                // SET 2,C
                unimplemented!()
            }
            0xD2 => {
                // SET 2,D
                unimplemented!()
            }
            0xD3 => {
                // SET 2,E
                unimplemented!()
            }
            0xD4 => {
                // SET 2,H
                unimplemented!()
            }
            0xD5 => {
                // SET 2,L
                unimplemented!()
            }
            0xD6 => {
                // SET 2,(HL)
                unimplemented!()
            }
            0xD7 => {
                // SET 2,A
                unimplemented!()
            }
            0xD8 => {
                // SET 3,B
                unimplemented!()
            }
            0xD9 => {
                // SET 3,C
                unimplemented!()
            }
            0xDA => {
                // SET 3,D
                unimplemented!()
            }
            0xDB => {
                // SET 3,E
                unimplemented!()
            }
            0xDC => {
                // SET 3,H
                unimplemented!()
            }
            0xDD => {
                // SET 3,L
                unimplemented!()
            }
            0xDE => {
                // SET 3,(HL)
                unimplemented!()
            }
            0xDF => {
                // SET 3,A
                unimplemented!()
            }
            0xE0 => {
                // SET 4,B
                unimplemented!()
            }
            0xE1 => {
                // SET 4,C
                unimplemented!()
            }
            0xE2 => {
                // SET 4,D
                unimplemented!()
            }
            0xE3 => {
                // SET 4,E
                unimplemented!()
            }
            0xE4 => {
                // SET 4,H
                unimplemented!()
            }
            0xE5 => {
                // SET 4,L
                unimplemented!()
            }
            0xE6 => {
                // SET 4,(HL)
                unimplemented!()
            }
            0xE7 => {
                // SET 4,A
                unimplemented!()
            }
            0xE8 => {
                // SET 5,B
                unimplemented!()
            }
            0xE9 => {
                // SET 5,C
                unimplemented!()
            }
            0xEA => {
                // SET 5,D
                unimplemented!()
            }
            0xEB => {
                // SET 5,E
                unimplemented!()
            }
            0xEC => {
                // SET 5,H
                unimplemented!()
            }
            0xED => {
                // SET 5,L
                unimplemented!()
            }
            0xEE => {
                // SET 5,(HL)
                unimplemented!()
            }
            0xEF => {
                // SET 5,A
                unimplemented!()
            }
            0xF0 => {
                // SET 6,B
                unimplemented!()
            }
            0xF1 => {
                // SET 6,C
                unimplemented!()
            }
            0xF2 => {
                // SET 6,D
                unimplemented!()
            }
            0xF3 => {
                // SET 6,E
                unimplemented!()
            }
            0xF4 => {
                // SET 6,H
                unimplemented!()
            }
            0xF5 => {
                // SET 6,L
                unimplemented!()
            }
            0xF6 => {
                // SET 6,(HL)
                unimplemented!()
            }
            0xF7 => {
                // SET 6,A
                unimplemented!()
            }
            0xF8 => {
                // SET 7,B
                unimplemented!()
            }
            0xF9 => {
                // SET 7,C
                unimplemented!()
            }
            0xFA => {
                // SET 7,D
                unimplemented!()
            }
            0xFB => {
                // SET 7,E
                unimplemented!()
            }
            0xFC => {
                // SET 7,H
                unimplemented!()
            }
            0xFD => {
                // SET 7,L
                unimplemented!()
            }
            0xFE => {
                // SET 7,(HL)
                unimplemented!()
            }
            0xFF => {
                // SET 7,A
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
