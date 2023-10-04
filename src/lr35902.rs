use std::fmt;
use std::{thread, time::Duration};

pub mod instructions;
use instructions::{Instruction, InstructionKind, INSTRUCTIONS};

use crate::bitwise as bw;

#[derive(PartialEq, Clone)]
pub struct LR35902 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    pub mem: [u8; 65536],
    next_cb: bool,
    clock_cycles: u64,
}

impl Default for LR35902 {
    fn default() -> Self {
        Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            mem: [0; 65536],
            next_cb: false,
            clock_cycles: 0,
        }
    }
}

impl fmt::Debug for LR35902 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LR35902 {{ af: {:#06X}, bc: {:#06X}, de: {:#06X}, hl: {:#06X}, sp: {:#06X}, pc: {:#06X}, clock_cycles: {} }} ", self.af, self.bc, self.de, self.hl, self.sp, self.pc, self.clock_cycles)
    }
}

impl LR35902 {
    pub fn new() -> Self {
        let mut m = Self::default();
        m.load_bootrom(include_bytes!("../dmg0.bin"));
        m
    }

    pub fn f(&self) -> u8 {
        bw::get_byte16::<0>(self.af)
    }

    pub fn set_f(&mut self, value: u8) {
        self.af = bw::set_byte16::<0>(self.af, value);
    }

    pub fn a(&self) -> u8 {
        bw::get_byte16::<1>(self.af)
    }

    pub fn set_a(&mut self, value: u8) {
        self.af = bw::set_byte16::<1>(self.af, value);
    }

    pub fn af(&self) -> u16 {
        self.af
    }

    pub fn set_af(&mut self, af: u16) {
        self.af = af;
    }

    pub fn b(&self) -> u8 {
        bw::get_byte16::<1>(self.bc)
    }

    pub fn set_b(&mut self, value: u8) {
        self.bc = bw::set_byte16::<1>(self.bc, value);
    }

    pub fn c(&self) -> u8 {
        bw::get_byte16::<0>(self.bc)
    }

    pub fn set_c(&mut self, value: u8) {
        self.bc = bw::set_byte16::<0>(self.bc, value);
    }

    pub fn bc(&self) -> u16 {
        self.bc
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.bc = bc;
    }

    pub fn d(&self) -> u8 {
        bw::get_byte16::<1>(self.de)
    }

    pub fn set_d(&mut self, value: u8) {
        self.de = bw::set_byte16::<1>(self.de, value);
    }

    pub fn e(&self) -> u8 {
        bw::get_byte16::<0>(self.de)
    }

    pub fn set_e(&mut self, value: u8) {
        self.de = bw::set_byte16::<0>(self.de, value);
    }

    pub fn de(&self) -> u16 {
        self.de
    }

    pub fn set_de(&mut self, de: u16) {
        self.de = de;
    }

    pub fn h(&self) -> u8 {
        bw::get_byte16::<1>(self.hl)
    }

    pub fn set_h(&mut self, value: u8) {
        self.hl = bw::set_byte16::<1>(self.hl, value);
    }

    pub fn l(&self) -> u8 {
        bw::get_byte16::<0>(self.hl)
    }

    pub fn set_l(&mut self, value: u8) {
        self.hl = bw::set_byte16::<0>(self.hl, value);
    }

    pub fn hl(&self) -> u16 {
        self.hl
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.hl = hl;
    }

    pub fn z_flag(&self) -> bool {
        bw::test_bit16::<8>(self.af)
    }

    pub fn set_z_flag(&mut self, value: bool) {
        self.af = bw::set_bit16::<7>(self.af, value);
    }

    pub fn n_flag(&self) -> bool {
        bw::test_bit16::<7>(self.af)
    }

    pub fn set_n_flag(&mut self, value: bool) {
        self.af = bw::set_bit16::<6>(self.af, value);
    }

    pub fn h_flag(&self) -> bool {
        bw::test_bit16::<6>(self.af)
    }

    pub fn set_h_flag(&mut self, value: bool) {
        self.af = bw::set_bit16::<5>(self.af, value);
    }

    pub fn c_flag(&self) -> bool {
        bw::test_bit16::<5>(self.af)
    }

    pub fn set_c_flag(&mut self, value: bool) {
        self.af = bw::set_bit16::<4>(self.af, value);
    }

    pub fn clock_cycles(&self) -> u64 {
        self.clock_cycles
    }

    pub fn set_clock_cycles(&mut self, clock_cycles: u64) {
        self.clock_cycles = clock_cycles;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn mem8(&self, index: u16) -> u8 {
        self.mem[index as usize]
    }

    pub fn mem16(&self, index: u16) -> u16 {
        bw::word16(self.mem8(index + 1), self.mem8(index))
    }

    pub fn set_mem8(&mut self, index: u16, value: u8) {
        self.mem[index as usize] = value;
    }

    pub fn set_mem16(&mut self, index: u16, value: u16) {
        self.set_mem8(index + 1, bw::get_byte16::<1>(value));
        self.set_mem8(index, bw::get_byte16::<0>(value));
    }

    /// get 8 bit immediate at position pc + 1 + pos
    fn get_d8(&self, pos: u8) -> u8 {
        self.mem8(self.pc + pos as u16 + 1)
    }

    /// get 16 bit immediate at position pc + 1 + pos
    fn get_d16(&self, pos: u8) -> u16 {
        // little-endian: the first byte in memory is the LSB
        ((self.get_d8(pos + 1) as u16) << 8) + self.get_d8(pos) as u16
    }

    fn load_bootrom(&mut self, bootrom: &[u8; 256]) {
        self.mem[..256].clone_from_slice(bootrom);
    }

    /// Run one cycle
    pub fn step(&mut self) {
        let mut opcode = self.mem[self.pc as usize] as u16;
        if self.next_cb {
            opcode += 0x100;
            self.next_cb = false;
        }
        let instruction = INSTRUCTIONS[opcode as usize];
        println!("{:#02X} {}", instruction.opcode, instruction.mnemonic);
        self.execute(instruction);
        if instruction.kind != InstructionKind::Jump {
            self.pc += instruction.size as u16;
        }
        thread::sleep(Duration::from_micros((instruction.cycles / 4) as u64));
        self.clock_cycles += instruction.cycles as u64;
        // TODO: measure time and panic if cycle time exceeded
    }

    fn half_carry8(&self, x: u8, y: u8) -> bool {
        ((x & 0x0f) + (y & 0x0f)) > 0x0f
    }

    fn half_carry16(&self, x: u16, y: u16) -> bool {
        self.half_carry8((x >> 8) as u8, (y >> 8) as u8)
    }

    fn add8(&mut self, x: u8, y: u8) -> u8 {
        let (result, overflow) = x.overflowing_add(y);
        self.set_z_flag(result == 0);
        self.set_n_flag(false);
        self.set_h_flag(self.half_carry8(x, y));
        self.set_c_flag(overflow);
        result
    }

    fn add16(&mut self, x: u16, y: u16) -> u16 {
        let (result, overflow) = x.overflowing_add(y);
        self.set_z_flag(result == 0);
        self.set_n_flag(false);
        self.set_h_flag(self.half_carry16(x, y));
        self.set_c_flag(overflow);
        result
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            0x0 => {
                // NOP
            }
            0x1 => {
                // LD BC,d16
                self.bc = self.get_d16(0);
            }
            0x2 => {
                // LD (BC),A
                self.set_mem8(self.bc(), self.a());
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
                self.set_b(self.get_d8(0));
            }
            0x7 => {
                // RLCA
                unimplemented!()
            }
            0x8 => {
                // LD (a16),SP
                self.set_mem16(dbg!(self.get_d16(0)), self.sp());
            }
            0x9 => {
                // ADD HL,BC
                let result = self.add16(self.hl(), self.bc());
                self.set_hl(result);
            }
            0xA => {
                // LD A,(BC)
                self.set_a(self.mem8(self.bc()));
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
                self.set_c(self.get_d8(0));
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
                self.de = self.get_d16(0);
            }
            0x12 => {
                // LD (DE),A
                self.set_mem8(self.de(), self.a());
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
                self.set_d(self.get_d8(0));
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
                self.set_a(self.mem8(self.de()));
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
                self.set_e(self.get_d8(0));
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
                self.set_hl(self.get_d16(0));
            }
            0x22 => {
                // LD (HL+),A
                self.set_mem8(self.hl(), self.a());
                self.set_hl(self.hl() + 1);
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
                self.set_h(self.get_d8(0));
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
                self.set_a(self.mem8(self.hl()));
                self.set_hl(self.hl() + 1);
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
                self.set_l(self.get_d8(0));
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
                self.set_sp(self.get_d16(0));
            }
            0x32 => {
                // LD (HL-),A
                self.set_mem8(self.hl, self.a());
                self.set_hl(self.hl() - 1)
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
                self.set_mem8(self.hl(), self.get_d8(0));
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
                self.set_a(self.mem8(self.hl()));
                self.set_hl(self.hl - 1);
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
                self.set_a(self.get_d8(0));
            }
            0x3F => {
                // CCF
                unimplemented!()
            }
            0x40 => {
                // LD B,B
                self.set_b(self.b());
            }
            0x41 => {
                // LD B,C
                self.set_b(self.c());
            }
            0x42 => {
                // LD B,D
                self.set_b(self.d());
            }
            0x43 => {
                // LD B,E
                self.set_b(self.e());
            }
            0x44 => {
                // LD B,H
                self.set_b(self.h());
            }
            0x45 => {
                // LD B,L
                self.set_b(self.l());
            }
            0x46 => {
                // LD B,(HL)
                self.set_b(self.mem8(self.hl()));
            }
            0x47 => {
                // LD B,A
                self.set_b(self.a());
            }
            0x48 => {
                // LD C,B
                self.set_c(self.b());
            }
            0x49 => {
                // LD C,C
                self.set_c(self.c());
            }
            0x4A => {
                // LD C,D
                self.set_c(self.d());
            }
            0x4B => {
                // LD C,E
                self.set_c(self.e());
            }
            0x4C => {
                // LD C,H
                self.set_c(self.h());
            }
            0x4D => {
                // LD C,L
                self.set_c(self.l());
            }
            0x4E => {
                // LD C,(HL)
                self.set_c(self.mem8(self.hl()));
            }
            0x4F => {
                // LD C,A
                self.set_c(self.a());
            }
            0x50 => {
                // LD D,B
                self.set_d(self.b());
            }
            0x51 => {
                // LD D,C
                self.set_d(self.c());
            }
            0x52 => {
                // LD D,D
                self.set_d(self.d());
            }
            0x53 => {
                // LD D,E
                self.set_d(self.e());
            }
            0x54 => {
                // LD D,H
                self.set_d(self.h());
            }
            0x55 => {
                // LD D,L
                self.set_d(self.l());
            }
            0x56 => {
                // LD D,(HL)
                self.set_d(self.mem8(self.hl()));
            }
            0x57 => {
                // LD D,A
                self.set_d(self.a());
            }
            0x58 => {
                // LD E,B
                self.set_e(self.b());
            }
            0x59 => {
                // LD E,C
                self.set_e(self.c());
            }
            0x5A => {
                // LD E,D
                self.set_e(self.d());
            }
            0x5B => {
                // LD E,E
                self.set_e(self.e());
            }
            0x5C => {
                // LD E,H
                self.set_e(self.h());
            }
            0x5D => {
                // LD E,L
                self.set_e(self.l());
            }
            0x5E => {
                // LD E,(HL)
                self.set_e(self.mem8(self.hl()));
            }
            0x5F => {
                // LD E,A
                self.set_e(self.a());
            }
            0x60 => {
                // LD H,B
                self.set_h(self.b());
            }
            0x61 => {
                // LD H,C
                self.set_h(self.c());
            }
            0x62 => {
                // LD H,D
                self.set_h(self.d());
            }
            0x63 => {
                // LD H,E
                self.set_h(self.e());
            }
            0x64 => {
                // LD H,H
                self.set_h(self.h());
            }
            0x65 => {
                // LD H,L
                self.set_h(self.l());
            }
            0x66 => {
                // LD H,(HL)
                self.set_h(self.mem8(self.hl()));
            }
            0x67 => {
                // LD H,A
                self.set_h(self.a());
            }
            0x68 => {
                // LD L,B
                self.set_l(self.b());
            }
            0x69 => {
                // LD L,C
                self.set_l(self.c());
            }
            0x6A => {
                // LD L,D
                self.set_l(self.d());
            }
            0x6B => {
                // LD L,E
                self.set_l(self.e());
            }
            0x6C => {
                // LD L,H
                self.set_l(self.h());
            }
            0x6D => {
                // LD L,L
                self.set_l(self.l());
            }
            0x6E => {
                // LD L,(HL)
                self.set_l(self.mem8(self.hl()));
            }
            0x6F => {
                // LD L,A
                self.set_l(self.a());
            }
            0x70 => {
                // LD (HL),B
                self.set_mem8(self.hl(), self.b());
            }
            0x71 => {
                // LD (HL),C
                self.set_mem8(self.hl(), self.c());
            }
            0x72 => {
                // LD (HL),D
                self.set_mem8(self.hl(), self.d());
            }
            0x73 => {
                // LD (HL),E
                self.set_mem8(self.hl(), self.e());
            }
            0x74 => {
                // LD (HL),H
                self.set_mem8(self.hl(), self.h());
            }
            0x75 => {
                // LD (HL),L
                self.set_mem8(self.hl(), self.l());
            }
            0x76 => {
                // HALT
                unimplemented!()
            }
            0x77 => {
                // LD (HL),A
                self.set_mem8(self.hl(), self.a());
            }
            0x78 => {
                // LD A,B
                self.set_a(self.b());
            }
            0x79 => {
                // LD A,C
                self.set_a(self.c());
            }
            0x7A => {
                // LD A,D
                self.set_a(self.d());
            }
            0x7B => {
                // LD A,E
                self.set_a(self.e());
            }
            0x7C => {
                // LD A,H
                self.set_a(self.h());
            }
            0x7D => {
                // LD A,L
                self.set_a(self.l());
            }
            0x7E => {
                // LD A,(HL)
                self.set_a(self.mem8(self.hl()));
            }
            0x7F => {
                // LD A,A
                self.set_a(self.a());
            }
            0x80 => {
                // ADD A,B
                let result = self.add8(self.a(), self.b());
                self.set_a(result);
            }
            0x81 => {
                // ADD A,C
                let result = self.add8(self.a(), self.c());
                self.set_a(result);
            }
            0x82 => {
                // ADD A,D
                let result = self.add8(self.a(), self.d());
                self.set_a(result);
            }
            0x83 => {
                // ADD A,E
                let result = self.add8(self.a(), self.e());
                self.set_a(result);
            }
            0x84 => {
                // ADD A,H
                let result = self.add8(self.a(), self.h());
                self.set_a(result);
            }
            0x85 => {
                // ADD A,L
                let result = self.add8(self.a(), self.l());
                self.set_a(result);
            }
            0x86 => {
                // ADD A,(HL)
                let result = self.add8(self.a(), self.mem[self.hl() as usize]);
                self.set_a(result);
            }
            0x87 => {
                // ADD A,A
                let result = self.add8(self.a(), self.a());
                self.set_a(result);
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
                let result = self.a() ^ self.b();
                self.set_z_flag(result == 0);
                self.set_n_flag(false);
                self.set_h_flag(false);
                self.set_c_flag(false);
                self.set_a(result);
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
                let result = self.add8(self.a(), self.get_d8(0));
                self.set_a(result);
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
                // Not implemented
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
                // Not implemented
                unimplemented!()
            }
            0xDC => {
                // CALL C,a16
                unimplemented!()
            }
            0xDD => {
                // Not implemented
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
                self.set_mem8(0xFF00 | self.get_d8(0) as u16, self.a());
            }
            0xE1 => {
                // POP HL
                unimplemented!()
            }
            0xE2 => {
                // LD (C),A
                self.set_mem8(0xFF00 + self.c() as u16, self.a());
            }
            0xE3 => {
                // Not implemented
                unimplemented!()
            }
            0xE4 => {
                // Not implemented
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
                self.set_mem8(self.get_d16(0), self.a());
            }
            0xEB => {
                // Not implemented
                unimplemented!()
            }
            0xEC => {
                // Not implemented
                unimplemented!()
            }
            0xED => {
                // Not implemented
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
                self.set_a(self.mem8(0xFF00 | self.get_d8(0) as u16));
            }
            0xF1 => {
                // POP AF
                unimplemented!()
            }
            0xF2 => {
                // LD A,(C)
                self.set_a(self.mem8(0xFF00 | self.c() as u16));
            }
            0xF3 => {
                // DI
                unimplemented!()
            }
            0xF4 => {
                // Not implemented
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
                let result = self.add16(self.sp(), self.get_d8(0) as u16);
                self.set_hl(dbg!(self.mem16(dbg!(result))));
                self.set_z_flag(false);
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
                // Not implemented
                unimplemented!()
            }
            0xFD => {
                // Not implemented
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
            0x100 => {
                // RLC B
                unimplemented!()
            }
            0x101 => {
                // RLC C
                unimplemented!()
            }
            0x102 => {
                // RLC D
                unimplemented!()
            }
            0x103 => {
                // RLC E
                unimplemented!()
            }
            0x104 => {
                // RLC H
                unimplemented!()
            }
            0x105 => {
                // RLC L
                unimplemented!()
            }
            0x106 => {
                // RLC (HL)
                unimplemented!()
            }
            0x107 => {
                // RLC A
                unimplemented!()
            }
            0x108 => {
                // RRC B
                unimplemented!()
            }
            0x109 => {
                // RRC C
                unimplemented!()
            }
            0x10A => {
                // RRC D
                unimplemented!()
            }
            0x10B => {
                // RRC E
                unimplemented!()
            }
            0x10C => {
                // RRC H
                unimplemented!()
            }
            0x10D => {
                // RRC L
                unimplemented!()
            }
            0x10E => {
                // RRC (HL)
                unimplemented!()
            }
            0x10F => {
                // RRC A
                unimplemented!()
            }
            0x110 => {
                // RL B
                unimplemented!()
            }
            0x111 => {
                // RL C
                unimplemented!()
            }
            0x112 => {
                // RL D
                unimplemented!()
            }
            0x113 => {
                // RL E
                unimplemented!()
            }
            0x114 => {
                // RL H
                unimplemented!()
            }
            0x115 => {
                // RL L
                unimplemented!()
            }
            0x116 => {
                // RL (HL)
                unimplemented!()
            }
            0x117 => {
                // RL A
                unimplemented!()
            }
            0x118 => {
                // RR B
                unimplemented!()
            }
            0x119 => {
                // RR C
                unimplemented!()
            }
            0x11A => {
                // RR D
                unimplemented!()
            }
            0x11B => {
                // RR E
                unimplemented!()
            }
            0x11C => {
                // RR H
                unimplemented!()
            }
            0x11D => {
                // RR L
                unimplemented!()
            }
            0x11E => {
                // RR (HL)
                unimplemented!()
            }
            0x11F => {
                // RR A
                unimplemented!()
            }
            0x120 => {
                // SLA B
                unimplemented!()
            }
            0x121 => {
                // SLA C
                unimplemented!()
            }
            0x122 => {
                // SLA D
                unimplemented!()
            }
            0x123 => {
                // SLA E
                unimplemented!()
            }
            0x124 => {
                // SLA H
                unimplemented!()
            }
            0x125 => {
                // SLA L
                unimplemented!()
            }
            0x126 => {
                // SLA (HL)
                unimplemented!()
            }
            0x127 => {
                // SLA A
                unimplemented!()
            }
            0x128 => {
                // SRA B
                unimplemented!()
            }
            0x129 => {
                // SRA C
                unimplemented!()
            }
            0x12A => {
                // SRA D
                unimplemented!()
            }
            0x12B => {
                // SRA E
                unimplemented!()
            }
            0x12C => {
                // SRA H
                unimplemented!()
            }
            0x12D => {
                // SRA L
                unimplemented!()
            }
            0x12E => {
                // SRA (HL)
                unimplemented!()
            }
            0x12F => {
                // SRA A
                unimplemented!()
            }
            0x130 => {
                // SWAP B
                unimplemented!()
            }
            0x131 => {
                // SWAP C
                unimplemented!()
            }
            0x132 => {
                // SWAP D
                unimplemented!()
            }
            0x133 => {
                // SWAP E
                unimplemented!()
            }
            0x134 => {
                // SWAP H
                unimplemented!()
            }
            0x135 => {
                // SWAP L
                unimplemented!()
            }
            0x136 => {
                // SWAP (HL)
                unimplemented!()
            }
            0x137 => {
                // SWAP A
                unimplemented!()
            }
            0x138 => {
                // SRL B
                unimplemented!()
            }
            0x139 => {
                // SRL C
                unimplemented!()
            }
            0x13A => {
                // SRL D
                unimplemented!()
            }
            0x13B => {
                // SRL E
                unimplemented!()
            }
            0x13C => {
                // SRL H
                unimplemented!()
            }
            0x13D => {
                // SRL L
                unimplemented!()
            }
            0x13E => {
                // SRL (HL)
                unimplemented!()
            }
            0x13F => {
                // SRL A
                unimplemented!()
            }
            0x140 => {
                // BIT 0,B
                unimplemented!()
            }
            0x141 => {
                // BIT 0,C
                unimplemented!()
            }
            0x142 => {
                // BIT 0,D
                unimplemented!()
            }
            0x143 => {
                // BIT 0,E
                unimplemented!()
            }
            0x144 => {
                // BIT 0,H
                unimplemented!()
            }
            0x145 => {
                // BIT 0,L
                unimplemented!()
            }
            0x146 => {
                // BIT 0,(HL)
                unimplemented!()
            }
            0x147 => {
                // BIT 0,A
                unimplemented!()
            }
            0x148 => {
                // BIT 1,B
                unimplemented!()
            }
            0x149 => {
                // BIT 1,C
                unimplemented!()
            }
            0x14A => {
                // BIT 1,D
                unimplemented!()
            }
            0x14B => {
                // BIT 1,E
                unimplemented!()
            }
            0x14C => {
                // BIT 1,H
                unimplemented!()
            }
            0x14D => {
                // BIT 1,L
                unimplemented!()
            }
            0x14E => {
                // BIT 1,(HL)
                unimplemented!()
            }
            0x14F => {
                // BIT 1,A
                unimplemented!()
            }
            0x150 => {
                // BIT 2,B
                unimplemented!()
            }
            0x151 => {
                // BIT 2,C
                unimplemented!()
            }
            0x152 => {
                // BIT 2,D
                unimplemented!()
            }
            0x153 => {
                // BIT 2,E
                unimplemented!()
            }
            0x154 => {
                // BIT 2,H
                unimplemented!()
            }
            0x155 => {
                // BIT 2,L
                unimplemented!()
            }
            0x156 => {
                // BIT 2,(HL)
                unimplemented!()
            }
            0x157 => {
                // BIT 2,A
                unimplemented!()
            }
            0x158 => {
                // BIT 3,B
                unimplemented!()
            }
            0x159 => {
                // BIT 3,C
                unimplemented!()
            }
            0x15A => {
                // BIT 3,D
                unimplemented!()
            }
            0x15B => {
                // BIT 3,E
                unimplemented!()
            }
            0x15C => {
                // BIT 3,H
                unimplemented!()
            }
            0x15D => {
                // BIT 3,L
                unimplemented!()
            }
            0x15E => {
                // BIT 3,(HL)
                unimplemented!()
            }
            0x15F => {
                // BIT 3,A
                unimplemented!()
            }
            0x160 => {
                // BIT 4,B
                unimplemented!()
            }
            0x161 => {
                // BIT 4,C
                unimplemented!()
            }
            0x162 => {
                // BIT 4,D
                unimplemented!()
            }
            0x163 => {
                // BIT 4,E
                unimplemented!()
            }
            0x164 => {
                // BIT 4,H
                unimplemented!()
            }
            0x165 => {
                // BIT 4,L
                unimplemented!()
            }
            0x166 => {
                // BIT 4,(HL)
                unimplemented!()
            }
            0x167 => {
                // BIT 4,A
                unimplemented!()
            }
            0x168 => {
                // BIT 5,B
                unimplemented!()
            }
            0x169 => {
                // BIT 5,C
                unimplemented!()
            }
            0x16A => {
                // BIT 5,D
                unimplemented!()
            }
            0x16B => {
                // BIT 5,E
                unimplemented!()
            }
            0x16C => {
                // BIT 5,H
                unimplemented!()
            }
            0x16D => {
                // BIT 5,L
                unimplemented!()
            }
            0x16E => {
                // BIT 5,(HL)
                unimplemented!()
            }
            0x16F => {
                // BIT 5,A
                unimplemented!()
            }
            0x170 => {
                // BIT 6,B
                unimplemented!()
            }
            0x171 => {
                // BIT 6,C
                unimplemented!()
            }
            0x172 => {
                // BIT 6,D
                unimplemented!()
            }
            0x173 => {
                // BIT 6,E
                unimplemented!()
            }
            0x174 => {
                // BIT 6,H
                unimplemented!()
            }
            0x175 => {
                // BIT 6,L
                unimplemented!()
            }
            0x176 => {
                // BIT 6,(HL)
                unimplemented!()
            }
            0x177 => {
                // BIT 6,A
                unimplemented!()
            }
            0x178 => {
                // BIT 7,B
                unimplemented!()
            }
            0x179 => {
                // BIT 7,C
                unimplemented!()
            }
            0x17A => {
                // BIT 7,D
                unimplemented!()
            }
            0x17B => {
                // BIT 7,E
                unimplemented!()
            }
            0x17C => {
                // BIT 7,H
                if !bw::test_bit16::<8>(self.hl) {
                    self.set_z_flag(true);
                }
                self.set_n_flag(false);
                self.set_h_flag(true);
            }
            0x17D => {
                // BIT 7,L
                unimplemented!()
            }
            0x17E => {
                // BIT 7,(HL)
                unimplemented!()
            }
            0x17F => {
                // BIT 7,A
                unimplemented!()
            }
            0x180 => {
                // RES 0,B
                unimplemented!()
            }
            0x181 => {
                // RES 0,C
                unimplemented!()
            }
            0x182 => {
                // RES 0,D
                unimplemented!()
            }
            0x183 => {
                // RES 0,E
                unimplemented!()
            }
            0x184 => {
                // RES 0,H
                unimplemented!()
            }
            0x185 => {
                // RES 0,L
                unimplemented!()
            }
            0x186 => {
                // RES 0,(HL)
                unimplemented!()
            }
            0x187 => {
                // RES 0,A
                unimplemented!()
            }
            0x188 => {
                // RES 1,B
                unimplemented!()
            }
            0x189 => {
                // RES 1,C
                unimplemented!()
            }
            0x18A => {
                // RES 1,D
                unimplemented!()
            }
            0x18B => {
                // RES 1,E
                unimplemented!()
            }
            0x18C => {
                // RES 1,H
                unimplemented!()
            }
            0x18D => {
                // RES 1,L
                unimplemented!()
            }
            0x18E => {
                // RES 1,(HL)
                unimplemented!()
            }
            0x18F => {
                // RES 1,A
                unimplemented!()
            }
            0x190 => {
                // RES 2,B
                unimplemented!()
            }
            0x191 => {
                // RES 2,C
                unimplemented!()
            }
            0x192 => {
                // RES 2,D
                unimplemented!()
            }
            0x193 => {
                // RES 2,E
                unimplemented!()
            }
            0x194 => {
                // RES 2,H
                unimplemented!()
            }
            0x195 => {
                // RES 2,L
                unimplemented!()
            }
            0x196 => {
                // RES 2,(HL)
                unimplemented!()
            }
            0x197 => {
                // RES 2,A
                unimplemented!()
            }
            0x198 => {
                // RES 3,B
                unimplemented!()
            }
            0x199 => {
                // RES 3,C
                unimplemented!()
            }
            0x19A => {
                // RES 3,D
                unimplemented!()
            }
            0x19B => {
                // RES 3,E
                unimplemented!()
            }
            0x19C => {
                // RES 3,H
                unimplemented!()
            }
            0x19D => {
                // RES 3,L
                unimplemented!()
            }
            0x19E => {
                // RES 3,(HL)
                unimplemented!()
            }
            0x19F => {
                // RES 3,A
                unimplemented!()
            }
            0x1A0 => {
                // RES 4,B
                unimplemented!()
            }
            0x1A1 => {
                // RES 4,C
                unimplemented!()
            }
            0x1A2 => {
                // RES 4,D
                unimplemented!()
            }
            0x1A3 => {
                // RES 4,E
                unimplemented!()
            }
            0x1A4 => {
                // RES 4,H
                unimplemented!()
            }
            0x1A5 => {
                // RES 4,L
                unimplemented!()
            }
            0x1A6 => {
                // RES 4,(HL)
                unimplemented!()
            }
            0x1A7 => {
                // RES 4,A
                unimplemented!()
            }
            0x1A8 => {
                // RES 5,B
                unimplemented!()
            }
            0x1A9 => {
                // RES 5,C
                unimplemented!()
            }
            0x1AA => {
                // RES 5,D
                unimplemented!()
            }
            0x1AB => {
                // RES 5,E
                unimplemented!()
            }
            0x1AC => {
                // RES 5,H
                unimplemented!()
            }
            0x1AD => {
                // RES 5,L
                unimplemented!()
            }
            0x1AE => {
                // RES 5,(HL)
                unimplemented!()
            }
            0x1AF => {
                // RES 5,A
                unimplemented!()
            }
            0x1B0 => {
                // RES 6,B
                unimplemented!()
            }
            0x1B1 => {
                // RES 6,C
                unimplemented!()
            }
            0x1B2 => {
                // RES 6,D
                unimplemented!()
            }
            0x1B3 => {
                // RES 6,E
                unimplemented!()
            }
            0x1B4 => {
                // RES 6,H
                unimplemented!()
            }
            0x1B5 => {
                // RES 6,L
                unimplemented!()
            }
            0x1B6 => {
                // RES 6,(HL)
                unimplemented!()
            }
            0x1B7 => {
                // RES 6,A
                unimplemented!()
            }
            0x1B8 => {
                // RES 7,B
                unimplemented!()
            }
            0x1B9 => {
                // RES 7,C
                unimplemented!()
            }
            0x1BA => {
                // RES 7,D
                unimplemented!()
            }
            0x1BB => {
                // RES 7,E
                unimplemented!()
            }
            0x1BC => {
                // RES 7,H
                unimplemented!()
            }
            0x1BD => {
                // RES 7,L
                unimplemented!()
            }
            0x1BE => {
                // RES 7,(HL)
                unimplemented!()
            }
            0x1BF => {
                // RES 7,A
                unimplemented!()
            }
            0x1C0 => {
                // SET 0,B
                unimplemented!()
            }
            0x1C1 => {
                // SET 0,C
                unimplemented!()
            }
            0x1C2 => {
                // SET 0,D
                unimplemented!()
            }
            0x1C3 => {
                // SET 0,E
                unimplemented!()
            }
            0x1C4 => {
                // SET 0,H
                unimplemented!()
            }
            0x1C5 => {
                // SET 0,L
                unimplemented!()
            }
            0x1C6 => {
                // SET 0,(HL)
                unimplemented!()
            }
            0x1C7 => {
                // SET 0,A
                unimplemented!()
            }
            0x1C8 => {
                // SET 1,B
                unimplemented!()
            }
            0x1C9 => {
                // SET 1,C
                unimplemented!()
            }
            0x1CA => {
                // SET 1,D
                unimplemented!()
            }
            0x1CB => {
                // SET 1,E
                unimplemented!()
            }
            0x1CC => {
                // SET 1,H
                unimplemented!()
            }
            0x1CD => {
                // SET 1,L
                unimplemented!()
            }
            0x1CE => {
                // SET 1,(HL)
                unimplemented!()
            }
            0x1CF => {
                // SET 1,A
                unimplemented!()
            }
            0x1D0 => {
                // SET 2,B
                unimplemented!()
            }
            0x1D1 => {
                // SET 2,C
                unimplemented!()
            }
            0x1D2 => {
                // SET 2,D
                unimplemented!()
            }
            0x1D3 => {
                // SET 2,E
                unimplemented!()
            }
            0x1D4 => {
                // SET 2,H
                unimplemented!()
            }
            0x1D5 => {
                // SET 2,L
                unimplemented!()
            }
            0x1D6 => {
                // SET 2,(HL)
                unimplemented!()
            }
            0x1D7 => {
                // SET 2,A
                unimplemented!()
            }
            0x1D8 => {
                // SET 3,B
                unimplemented!()
            }
            0x1D9 => {
                // SET 3,C
                unimplemented!()
            }
            0x1DA => {
                // SET 3,D
                unimplemented!()
            }
            0x1DB => {
                // SET 3,E
                unimplemented!()
            }
            0x1DC => {
                // SET 3,H
                unimplemented!()
            }
            0x1DD => {
                // SET 3,L
                unimplemented!()
            }
            0x1DE => {
                // SET 3,(HL)
                unimplemented!()
            }
            0x1DF => {
                // SET 3,A
                unimplemented!()
            }
            0x1E0 => {
                // SET 4,B
                unimplemented!()
            }
            0x1E1 => {
                // SET 4,C
                unimplemented!()
            }
            0x1E2 => {
                // SET 4,D
                unimplemented!()
            }
            0x1E3 => {
                // SET 4,E
                unimplemented!()
            }
            0x1E4 => {
                // SET 4,H
                unimplemented!()
            }
            0x1E5 => {
                // SET 4,L
                unimplemented!()
            }
            0x1E6 => {
                // SET 4,(HL)
                unimplemented!()
            }
            0x1E7 => {
                // SET 4,A
                unimplemented!()
            }
            0x1E8 => {
                // SET 5,B
                unimplemented!()
            }
            0x1E9 => {
                // SET 5,C
                unimplemented!()
            }
            0x1EA => {
                // SET 5,D
                unimplemented!()
            }
            0x1EB => {
                // SET 5,E
                unimplemented!()
            }
            0x1EC => {
                // SET 5,H
                unimplemented!()
            }
            0x1ED => {
                // SET 5,L
                unimplemented!()
            }
            0x1EE => {
                // SET 5,(HL)
                unimplemented!()
            }
            0x1EF => {
                // SET 5,A
                unimplemented!()
            }
            0x1F0 => {
                // SET 6,B
                unimplemented!()
            }
            0x1F1 => {
                // SET 6,C
                unimplemented!()
            }
            0x1F2 => {
                // SET 6,D
                unimplemented!()
            }
            0x1F3 => {
                // SET 6,E
                unimplemented!()
            }
            0x1F4 => {
                // SET 6,H
                unimplemented!()
            }
            0x1F5 => {
                // SET 6,L
                unimplemented!()
            }
            0x1F6 => {
                // SET 6,(HL)
                unimplemented!()
            }
            0x1F7 => {
                // SET 6,A
                unimplemented!()
            }
            0x1F8 => {
                // SET 7,B
                unimplemented!()
            }
            0x1F9 => {
                // SET 7,C
                unimplemented!()
            }
            0x1FA => {
                // SET 7,D
                unimplemented!()
            }
            0x1FB => {
                // SET 7,E
                unimplemented!()
            }
            0x1FC => {
                // SET 7,H
                unimplemented!()
            }
            0x1FD => {
                // SET 7,L
                unimplemented!()
            }
            0x1FE => {
                // SET 7,(HL)
                unimplemented!()
            }
            0x1FF => {
                // SET 7,A
                unimplemented!()
            }
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
        let mut cpu = LR35902::default();

        assert_eq!(cpu.a(), 0);
        assert_eq!(cpu.af, 0);

        cpu.set_a(5);
        assert_eq!(cpu.a(), 5);
        assert_eq!(cpu.af, 5 << 8);
    }

    #[test]
    fn test_b() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.b(), 0);
        assert_eq!(cpu.bc, 0);

        cpu.set_b(5);
        assert_eq!(cpu.b(), 5);
        assert_eq!(cpu.bc, 5 << 8);
    }

    #[test]
    fn test_c() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.c(), 0);
        assert_eq!(cpu.bc, 0);

        cpu.set_c(5);
        assert_eq!(cpu.c(), 5);
        assert_eq!(cpu.bc, 5);
    }

    #[test]
    fn test_h() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.h(), 0);
        assert_eq!(cpu.hl, 0);

        cpu.set_h(5);
        assert_eq!(cpu.h(), 5);
        assert_eq!(cpu.hl, 5 << 8);
    }

    #[test]
    fn test_l() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.l(), 0);
        assert_eq!(cpu.hl, 0);

        cpu.set_l(5);
        assert_eq!(cpu.l(), 5);
        assert_eq!(cpu.hl, 5);
    }

    #[test]
    fn test_d() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.d(), 0);
        assert_eq!(cpu.de, 0);

        cpu.set_d(5);
        assert_eq!(cpu.d(), 5);
        assert_eq!(cpu.de, 5 << 8);
    }

    #[test]
    fn test_e() {
        let mut cpu = LR35902::default();

        assert_eq!(cpu.e(), 0);
        assert_eq!(cpu.de, 0);

        cpu.set_e(5);
        assert_eq!(cpu.e(), 5);
        assert_eq!(cpu.de, 5);
    }

    #[test]
    fn test_immediate8() {
        let mut cpu = LR35902::default();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.get_d8(0), 2);
    }

    #[test]
    fn test_immediate16() {
        let mut cpu = LR35902::default();
        let mut bootrom = [0; 256];
        bootrom[0] = 1;
        bootrom[1] = 2;
        bootrom[2] = 3;
        cpu.load_bootrom(&bootrom);

        assert_eq!(cpu.get_d16(0), 3 * 256 + 2);
    }

    #[test]
    fn test_memory() {
        let mut cpu = LR35902::default();

        cpu.set_mem8(10, 255);
        assert_eq!(cpu.mem8(10), 255);
    }
}
