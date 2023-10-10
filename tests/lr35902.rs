use rstest::*;

use fpt::lr35902::LR35902;

#[derive(Clone)]
struct LR35902Builder {
    lr35902: LR35902,
}

impl LR35902Builder {
    pub fn new() -> Self {
        Self {
            lr35902: LR35902::default(),
        }
    }

    pub fn with_reg8(self, register: &str, value: u8) -> Self {
        match register {
            "b" => self.with_b(value),
            "c" => self.with_c(value),
            "d" => self.with_d(value),
            "e" => self.with_e(value),
            "h" => self.with_h(value),
            "l" => self.with_l(value),
            "a" => self.with_a(value),
            _ => panic!(),
        }
    }

    pub fn with_reg16(self, register: &str, value: u16) -> Self {
        match register {
            "bc" => self.with_bc(value),
            "de" => self.with_de(value),
            "hl" => self.with_hl(value),
            "af" => self.with_af(value),
            "sp" => self.with_sp(value),
            _ => panic!(),
        }
    }

    pub fn with_flag(self, flag: &str, value: bool) -> Self {
        match flag {
            "z" => self.with_z_flag(value),
            "c" => self.with_c_flag(value),
            "n" => self.with_n_flag(value),
            "h" => self.with_h_flag(value),
            _ => panic!(),
        }
    }

    pub fn with_a(mut self, a: u8) -> Self {
        self.lr35902.set_a(a);
        self
    }

    pub fn with_f(mut self, f: u8) -> Self {
        self.lr35902.set_f(f);
        self
    }

    pub fn with_af(mut self, af: u16) -> Self {
        self.lr35902.set_af(af);
        self
    }

    pub fn with_b(mut self, b: u8) -> Self {
        self.lr35902.set_b(b);
        self
    }

    pub fn with_c(mut self, c: u8) -> Self {
        self.lr35902.set_c(c);
        self
    }

    pub fn with_bc(mut self, bc: u16) -> Self {
        self.lr35902.set_bc(bc);
        self
    }

    pub fn with_d(mut self, d: u8) -> Self {
        self.lr35902.set_d(d);
        self
    }

    pub fn with_e(mut self, e: u8) -> Self {
        self.lr35902.set_e(e);
        self
    }

    pub fn with_de(mut self, de: u16) -> Self {
        self.lr35902.set_de(de);
        self
    }

    pub fn with_h(mut self, h: u8) -> Self {
        self.lr35902.set_h(h);
        self
    }

    pub fn with_l(mut self, l: u8) -> Self {
        self.lr35902.set_l(l);
        self
    }

    pub fn with_hl(mut self, hl: u16) -> Self {
        self.lr35902.set_hl(hl);
        self
    }

    pub fn with_sp(mut self, sp: u16) -> Self {
        self.lr35902.set_sp(sp);
        self
    }

    pub fn with_pc(mut self, pc: u16) -> Self {
        self.lr35902.set_pc(pc);
        self
    }

    pub fn with_z_flag(mut self, z: bool) -> Self {
        self.lr35902.set_z_flag(z);
        self
    }

    pub fn with_c_flag(mut self, c: bool) -> Self {
        self.lr35902.set_c_flag(c);
        self
    }

    pub fn with_n_flag(mut self, n: bool) -> Self {
        self.lr35902.set_n_flag(n);
        self
    }

    pub fn with_h_flag(mut self, h: bool) -> Self {
        self.lr35902.set_h_flag(h);
        self
    }

    pub fn with_clock_cycles(mut self, clock_cycles: u64) -> LR35902Builder {
        self.lr35902.set_clock_cycles(clock_cycles);
        self
    }

    pub fn with_mem8(mut self, index: u16, value: u8) -> LR35902Builder {
        self.lr35902.set_mem8(index, value);
        self
    }

    pub fn with_mem16(mut self, index: u16, value: u16) -> LR35902Builder {
        self.lr35902.set_mem16(index, value);
        self
    }

    pub fn build(self) -> LR35902 {
        self.lr35902
    }
}

#[test]
fn test_instr_0x000_nop() {
    // Given
    let builder = LR35902Builder::new().with_mem8(0x0000, 0x0);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder.with_pc(1).with_clock_cycles(4).build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(2, 1, 0x0102)]
fn test_instr_0x001_ld_bc_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x1) // instruction LD BC,d16
        .with_mem8(0x0001, lsb) // lsb of immediate16
        .with_mem8(0x0002, msb); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_bc(result) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x02, "bc")]
#[case(0x12, "de")]
fn test_instr_ld_pointer_from_a(#[case] opcode: u8, #[case] register: &str) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(0x01)
        .with_reg16(register, 0xFF00);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_mem16(0xFF00, 0x01)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x008_ld_pointer_immediate16_from_sp() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x8)
        .with_mem16(0x0001, 0xFF00)
        .with_sp(0x01);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_mem16(0xFF00, 0x01)
        .with_clock_cycles(20)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(2, 1, 0x0102)]
fn test_instr_0x011_ld_de_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x11) // instruction LD DE,d16
        .with_mem8(0x0001, lsb) // lsb of immediate16
        .with_mem8(0x0002, msb); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_de(result) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0xa, "bc", 0xFF00, 0x01)]
#[case(0x1a, "de", 0xFF00, 0x01)]
fn test_instr_ld_register_a_from_pointer(
    #[case] opcode: u8,
    #[case] register: &str,
    #[case] address: u16,
    #[case] value: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode) // instruction LD (HL-), a
        .with_mem8(address, value)
        .with_reg16(register, address);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(8)
        .with_a(value)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x2, 0x1, 0x0102)]
fn test_instr_0x021_ld_hl_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x21) // instruction LD HL,d16
        .with_mem8(0x0001, lsb) // lsb of immediate16
        .with_mem8(0x0002, msb); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_hl(result) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x022_ld_pointer_hl_increment_from_a() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x22)
        .with_hl(0xFF00)
        .with_a(0x1);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(0xFF01)
        .with_mem8(0xFF00, 0x1)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x2a_ld_register_a_from_hli() {
    // Given
    let hl = 0xFF00;
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x2a) // instruction LD (HL-), a
        .with_mem8(hl, 0x01)
        .with_hl(hl);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(hl + 1) // hl gets decremented
        .with_clock_cycles(8)
        .with_a(0x01)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x2, 0x1, 0x0102)]
#[case(0xFF, 0xFF, 0xFFFF)]
fn test_instr_0x031_ld_sp_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x31) // instruction LD SP,d16
        .with_mem8(0x0001, lsb) // lsb of immediate16
        .with_mem8(0x0002, msb); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_sp(result) // (msb << 8) + lsb == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x10, 0x100)]
#[case(0xFF, 0x1)]
fn test_instr_0x032_ld_hld_a(#[case] a: u8, #[case] hl: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x32) // instruction LD (HL-), a
        .with_a(a)
        .with_hl(hl);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(hl - 1) // hl gets decremented
        .with_clock_cycles(8)
        .with_mem8(hl, a)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x3a_ld_register_a_from_hld() {
    // Given

    let hl = 0xFF00;
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x3a) // instruction LD (HL-), a
        .with_mem8(hl, 0x01)
        .with_hl(hl);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(hl - 1) // hl gets decremented
        .with_clock_cycles(8)
        .with_a(0x01)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x40, "b", "b", 0x01)] //  1
#[case(0x40, "b", "b", 0xFF)] //  2
#[case(0x41, "b", "c", 0x01)] //  3
#[case(0x41, "b", "c", 0xFF)] //  4
#[case(0x42, "b", "d", 0x01)] //  5
#[case(0x42, "b", "d", 0xFF)] //  6
#[case(0x43, "b", "e", 0x01)] //  7
#[case(0x43, "b", "e", 0xFF)] //  8
#[case(0x44, "b", "h", 0x01)] //  9
#[case(0x44, "b", "h", 0xFF)] // 10
#[case(0x45, "b", "l", 0x01)] // 11
#[case(0x45, "b", "l", 0xFF)] // 12
#[case(0x47, "b", "a", 0x01)] // 13
#[case(0x47, "b", "a", 0xFF)] // 14
#[case(0x48, "c", "b", 0x01)] // 15
#[case(0x48, "c", "b", 0xFF)] // 16
#[case(0x49, "c", "c", 0x01)] // 17
#[case(0x49, "c", "c", 0xFF)] // 18
#[case(0x4A, "c", "d", 0x01)] // 19
#[case(0x4A, "c", "d", 0xFF)] // 20
#[case(0x4B, "c", "e", 0x01)] // 21
#[case(0x4B, "c", "e", 0xFF)] // 22
#[case(0x4C, "c", "h", 0x01)] // 23
#[case(0x4C, "c", "h", 0xFF)] // 24
#[case(0x4D, "c", "l", 0x01)] // 25
#[case(0x4D, "c", "l", 0xFF)] // 26
#[case(0x4F, "c", "a", 0x01)] // 27
#[case(0x4F, "c", "a", 0xFF)] // 28
#[case(0x50, "d", "b", 0x01)] // 29
#[case(0x50, "d", "b", 0xFF)] // 30
#[case(0x51, "d", "c", 0x01)] // 31
#[case(0x51, "d", "c", 0xFF)] // 32
#[case(0x52, "d", "d", 0x01)] // 33
#[case(0x52, "d", "d", 0xFF)] // 34
#[case(0x53, "d", "e", 0x01)] // 35
#[case(0x53, "d", "e", 0xFF)] // 36
#[case(0x54, "d", "h", 0x01)] // 37
#[case(0x54, "d", "h", 0xFF)] // 38
#[case(0x55, "d", "l", 0x01)] // 39
#[case(0x55, "d", "l", 0xFF)] // 40
#[case(0x57, "d", "a", 0x01)] // 41
#[case(0x57, "d", "a", 0xFF)] // 42
#[case(0x58, "e", "b", 0x01)] // 43
#[case(0x58, "e", "b", 0xFF)] // 44
#[case(0x59, "e", "c", 0x01)] // 45
#[case(0x59, "e", "c", 0xFF)] // 46
#[case(0x5a, "e", "d", 0x01)] // 47
#[case(0x5a, "e", "d", 0xFF)] // 48
#[case(0x5b, "e", "e", 0x01)] // 49
#[case(0x5b, "e", "e", 0xFF)] // 50
#[case(0x5c, "e", "h", 0x01)] // 51
#[case(0x5c, "e", "h", 0xFF)] // 52
#[case(0x5d, "e", "l", 0x01)] // 53
#[case(0x5d, "e", "l", 0xFF)] // 54
#[case(0x5f, "e", "a", 0x01)] // 55
#[case(0x5f, "e", "a", 0xFF)] // 56
#[case(0x60, "h", "b", 0x01)] // 57
#[case(0x60, "h", "b", 0xFF)] // 58
#[case(0x61, "h", "c", 0x01)] // 59
#[case(0x61, "h", "c", 0xFF)] // 60
#[case(0x62, "h", "d", 0x01)] // 61
#[case(0x62, "h", "d", 0xFF)] // 62
#[case(0x63, "h", "e", 0x01)] // 63
#[case(0x63, "h", "e", 0xFF)] // 64
#[case(0x64, "h", "h", 0x01)] // 65
#[case(0x64, "h", "h", 0xFF)] // 66
#[case(0x65, "h", "l", 0x01)] // 67
#[case(0x65, "h", "l", 0xFF)] // 68
#[case(0x67, "h", "a", 0x01)] // 69
#[case(0x67, "h", "a", 0xFF)] // 70
#[case(0x68, "l", "b", 0x01)] // 71
#[case(0x68, "l", "b", 0xFF)] // 72
#[case(0x69, "l", "c", 0x01)] // 73
#[case(0x69, "l", "c", 0xFF)] // 74
#[case(0x6a, "l", "d", 0x01)] // 75
#[case(0x6a, "l", "d", 0xFF)] // 76
#[case(0x6b, "l", "e", 0x01)] // 77
#[case(0x6b, "l", "e", 0xFF)] // 78
#[case(0x6c, "l", "h", 0x01)] // 79
#[case(0x6c, "l", "h", 0xFF)] // 80
#[case(0x6d, "l", "l", 0x01)] // 81
#[case(0x6d, "l", "l", 0xFF)] // 82
#[case(0x6f, "l", "a", 0x01)] // 83
#[case(0x6f, "l", "a", 0xFF)] // 84
#[case(0x78, "a", "b", 0x01)] // 85
#[case(0x78, "a", "b", 0xFF)] // 86
#[case(0x79, "a", "c", 0x01)] // 87
#[case(0x79, "a", "c", 0xFF)] // 88
#[case(0x7a, "a", "d", 0x01)] // 89
#[case(0x7a, "a", "d", 0xFF)] // 90
#[case(0x7b, "a", "e", 0x01)] // 91
#[case(0x7b, "a", "e", 0xFF)] // 92
#[case(0x7c, "a", "h", 0x01)] // 93
#[case(0x7c, "a", "h", 0xFF)] // 94
#[case(0x7d, "a", "l", 0x01)] // 95
#[case(0x7d, "a", "l", 0xFF)] // 96
#[case(0x7f, "a", "a", 0x01)] // 97
#[case(0x7f, "a", "a", 0xFF)] // 98
fn test_load_8_bit_reg_to_8_bit_reg(
    #[case] opcode: u8,
    #[case] dst_reg: &str,
    #[case] src_reg: &str,
    #[case] value: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_reg8(src_reg, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(4)
        .with_reg8(dst_reg, value)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x46, "b", 0x0100, 0x01)] //  1
#[case(0x46, "b", 0x0100, 0xFF)] //  2
#[case(0x4E, "c", 0x0100, 0x01)] //  3
#[case(0x4E, "c", 0x0100, 0xFF)] //  4
#[case(0x56, "d", 0x0100, 0x01)] //  5
#[case(0x56, "d", 0x0100, 0xFF)] //  6
#[case(0x5e, "e", 0x0100, 0x01)] //  7
#[case(0x5e, "e", 0x0100, 0xFF)] //  8
#[case(0x66, "h", 0x0100, 0x01)] //  9
#[case(0x66, "h", 0x0100, 0xFF)] // 10
#[case(0x6E, "l", 0x0100, 0x01)] // 11
#[case(0x6E, "l", 0x0100, 0xFF)] // 12
#[case(0x7E, "a", 0x0100, 0x01)] // 13
#[case(0x7E, "a", 0x0100, 0xFF)] // 14
fn test_load_8_bit_reg_from_hl_pointer(
    #[case] opcode: u8,
    #[case] dst_reg: &str,
    #[case] hl: u16,
    #[case] value: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_mem8(hl, value)
        .with_hl(hl);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(8)
        .with_reg8(dst_reg, value)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x70, "b", 0x0100, 0x01)] //  1
#[case(0x70, "b", 0x0100, 0xFF)] //  2
#[case(0x71, "c", 0x0100, 0x01)] //  3
#[case(0x71, "c", 0x0100, 0xFF)] //  4
#[case(0x72, "d", 0x0100, 0x01)] //  5
#[case(0x72, "d", 0x0100, 0xFF)] //  6
#[case(0x73, "e", 0x0100, 0x01)] //  7
#[case(0x73, "e", 0x0100, 0xFF)] //  8
#[case(0x74, "h", 0x0100, 0x01)] //  9
#[case(0x74, "h", 0x0100, 0xFF)] // 10
#[case(0x75, "l", 0x0100, 0x01)] // 11
#[case(0x75, "l", 0x0100, 0xFF)] // 12
#[case(0x77, "a", 0x0100, 0x01)] // 13
#[case(0x77, "a", 0x0100, 0xFF)] // 14
fn test_load_hl_pointer_from_8_bit_reg(
    #[case] opcode: u8,
    #[case] src_reg: &str,
    #[case] hl: u16,
    #[case] value: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_hl(dbg!(hl))
        .with_reg8(src_reg, value);

    let mut sut = builder.clone().build();
    let hl = sut.hl();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(8)
        .with_mem8(hl, value)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x06, "b", 0x01)] // 1
#[case(0x16, "d", 0x01)] // 2
#[case(0x26, "h", 0x01)] // 3
#[case(0x0e, "c", 0x01)] // 4
#[case(0x1e, "e", 0x01)] // 5
#[case(0x2e, "l", 0x01)] // 6
#[case(0x3e, "a", 0x01)] // 7
fn test_load_register_from_immediate(#[case] opcode: u8, #[case] reg: &str, #[case] d8: u8) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_mem8(0x0001, d8);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_clock_cycles(8)
        .with_reg8(reg, d8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x01, 0x0100)]
#[case(0xFF, 0x0100)]
fn test_instr_0x36_ld_hl_d8(#[case] d8: u8, #[case] hl: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x36)
        .with_mem8(0x0001, d8)
        .with_hl(hl);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_clock_cycles(12)
        .with_mem8(hl, d8)
        .build();
    assert_eq!(sut, expected);
}

// TODO e8 is signed
#[rstest]
fn test_instr_0xf8_ld_hl_sp_plus_e8() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xf8)
        .with_mem8(0x0001, 0x05) // e8 = 5
        .with_sp(0x1000);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_clock_cycles(12)
        .with_hl(0x1005)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0xe0_ld_immediate8_pointer_from_register_a() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xe0)
        .with_mem8(0x0001, 0xFF)
        .with_a(0x01);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_clock_cycles(12)
        .with_mem8(0xFFFF, 0x01)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0xe2_ld_pointer_c_from_register_a() {
    // Given
    let address = 0xFF;

    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xe2)
        .with_c(address)
        .with_a(0x01);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    dbg!(sut.mem8(0xFF00 + (address as u16)));

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(8)
        .with_mem8(0xFF00 + (address as u16), 0x01)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0xea_ld_immediate16_pointer_from_register_a() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xea)
        .with_mem16(0x0001, 0xFFFF)
        .with_a(0x01);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_clock_cycles(16)
        .with_mem8(0xFFFF, 0x01)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0xf0_ld_register_a_from_immediate_pointer() {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xf0)
        .with_mem8(0x0001, 0xFF)
        .with_mem8(0xFFFF, 0x01);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_clock_cycles(12)
        .with_a(0x01)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0xf2_ld_from_register_a_from_c_pointer() {
    // Given
    let address = 0xFF;

    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0xf2)
        .with_mem8(0xFFFF, 0x01)
        .with_c(address);

    let mut sut = builder.clone().build();

    // When
    sut.step();

    dbg!(sut.mem8(0xFF00 + (address as u16)));

    // Then
    let expected = builder.with_pc(1).with_clock_cycles(8).with_a(0x01).build();
    assert_eq!(sut, expected);
}

// TODO: break test_add8 (and test_xor8) into three:
// ADD A,<reg not A>
// ADD A,A
#[rstest]
// ADD A,(HL)
#[case(0x86, 0xfe, 0x0001, 0x01, 0xff, 0b0000)] // no flags
#[case(0x86, 0xff, 0x0001, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x86, 0xff, 0xcafe, 0x01, 0x00, 0b1011)] // zero, half carry and carry
// XOR A,(HL)
#[case(0xAE, 0xca, 0x0001, 0xfe, 0x34, 0b0000)]
#[case(0xAE, 0x01, 0xcafe, 0x01, 0x00, 0b1000)]
fn test_alu_reg_addr(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] hl_addr: u16,
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(a)
        .with_reg16("hl", hl_addr)
        .with_mem8(hl_addr, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_a(result)
        .with_f(flags << 4)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
// ADD A,r8
#[case(0x80, 0xfe, "b", 0x01, 0xff, 0b0000)] // no flags
#[case(0x80, 0x0f, "b", 0x01, 0x10, 0b0010)] // half carry
#[case(0x80, 0xff, "b", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x81, 0xff, "c", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x82, 0xff, "d", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x83, 0xff, "e", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x84, 0xff, "h", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x85, 0xff, "l", 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x87, 0x80, "a", 0x80, 0x00, 0b1001)] // zero, half carry and carry
#[case(0x87, 0x88, "a", 0x88, 0x10, 0b0011)] // zero, half carry and carry
// XOR A,r8
#[case(0xA8, 0xca, "b", 0xfe, 0x34, 0b0000)]
#[case(0xA8, 0xca, "b", 0xca, 0x00, 0b1000)]
#[case(0xA9, 0xca, "c", 0xfe, 0x34, 0b0000)]
#[case(0xAA, 0xca, "d", 0xfe, 0x34, 0b0000)]
#[case(0xAB, 0xca, "e", 0xfe, 0x34, 0b0000)]
#[case(0xAC, 0xca, "h", 0xfe, 0x34, 0b0000)]
#[case(0xAD, 0xca, "l", 0xfe, 0x34, 0b0000)]
#[case(0xAF, 0xca, "a", 0xca, 0x00, 0b1000)]
fn test_alu8_reg_reg(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] src_reg: &str,
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(a)
        .with_reg8(src_reg, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_a(result)
        .with_f(flags << 4)
        .with_clock_cycles(4)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
// ADD HL,r16
#[case(0x09, 0xffff, "bc", 0x0001, 0x0, 0b0000, 0b0011)]
#[case(0x09, 0xffff, "bc", 0x0001, 0x0, 0b1000, 0b1011)]
#[case(0x19, 0xffff, "de", 0x0001, 0x0, 0b0000, 0b0011)]
#[case(0x19, 0xffff, "de", 0x0001, 0x0, 0b1000, 0b1011)]
#[case(0x29, 0x8000, "hl", 0x8000, 0x0, 0b0000, 0b0001)]
#[case(0x29, 0x8000, "hl", 0x8000, 0x0, 0b1000, 0b1001)]
#[case(0x39, 0xffff, "sp", 0x0001, 0x0, 0b0000, 0b0011)]
#[case(0x39, 0xffff, "sp", 0x0001, 0x0, 0b1000, 0b1011)]
fn test_alu16_reg_reg(
    #[case] opcode: u8,
    #[case] hl: u16,
    #[case] src_reg: &str,
    #[case] value: u16,
    #[case] result: u16,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_f(flags_before << 4)
        .with_hl(hl)
        .with_reg16(src_reg, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(result)
        .with_f(flags_after << 4)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case::base_case(0x00, 0x01, 0b0000, 0b0000)]
#[case::overwrite(0x41, 0x42, 0b1111, 0b0001)]
#[case::half_carry(0x0F, 0x10, 0b0010, 0b0010)]
#[case::zero_flag(0xFF, 0x00, 0b0000, 0b1010)] // and no carry, unlike ADD 1
fn test_inc_8_bit_reg(
    #[values((0x04, "b"), (0x0C, "c"),
             (0x14, "d"), (0x1C, "e"),
             (0x24, "h"), (0x2C, "l"),
             (0x3C, "a"))]
    _opcode_reg @ (opcode, reg): (u8, &str),
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_reg8(reg, value)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    let expected = builder
        .with_pc(1)
        .with_reg8(reg, result)
        .with_f(flags_after << 4)
        .with_clock_cycles(4)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x0000, 0x0001)]
#[case(0x00FF, 0x0100)]
#[case(0xFFFF, 0x0000)]
fn test_inc_16_bit_reg(
    #[values((0x03, "bc"),
             (0x13, "de"),
             (0x23, "hl"),
             (0x33, "sp"))]
    _opcode_reg @ (opcode, reg): (u8, &str),
    #[case] value: u16,
    #[case] result: u16,
    #[values(0b0000, 0b1111)] flags: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_reg16(reg, value)
        .with_f(flags << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    let expected = builder
        .with_pc(1)
        .with_reg16(reg, result)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}
#[rstest]
#[case(0xC2, 0xFF00, "z", true, 3, 12)]
#[case(0xC2, 0xFF00, "z", false, 0xFF00, 16)]
#[case(0xD2, 0xFF00, "c", true, 3, 12)]
#[case(0xD2, 0xFF00, "c", false, 0xFF00, 16)]
#[case(0xCA, 0xFF00, "z", true, 0xFF00, 16)]
#[case(0xCA, 0xFF00, "z", false, 3, 12)]
#[case(0xDA, 0xFF00, "c", true, 0xFF00, 16)]
#[case(0xDA, 0xFF00, "c", false, 3, 12)]
#[case(0xC3, 0xFF00, "z", false, 0xFF00, 16)]
#[case(0xC3, 0xFF00, "z", true, 0xFF00, 16)]
#[case(0xC3, 0xFF00, "c", false, 0xFF00, 16)]
#[case(0xC3, 0xFF00, "c", true, 0xFF00, 16)]
#[case(0x20, 0x00FF, "z", true, 2, 8)]
#[case(0x20, 0x00FF, "z", false, 0x00FF, 12)]
#[case(0x30, 0x00FF, "c", true, 2, 8)]
#[case(0x30, 0x00FF, "c", false, 0x00FF, 12)]
#[case(0x28, 0x00FF, "z", true, 0x00FF, 12)]
#[case(0x28, 0x00FF, "z", false, 2, 8)]
#[case(0x38, 0x00FF, "c", true, 0x00FF, 12)]
#[case(0x38, 0x00FF, "c", false, 2, 8)]
#[case(0x18, 0x00FF, "z", false, 0x00FF, 12)]
#[case(0x18, 0x00FF, "z", true, 0x00FF, 12)]
#[case(0x18, 0x00FF, "c", false, 0x00FF, 12)]
#[case(0x18, 0x00FF, "c", true, 0x00FF, 12)]
fn test_jump(
    #[case] opcode: u8,
    #[case] address: u16,
    #[case] flag: &str,
    #[case] value: bool,
    #[case] pc: u16,
    #[case] clocks: u64,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_mem16(0x0001, address)
        .with_flag(flag, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder.with_pc(pc).with_clock_cycles(clocks).build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0xC5, "bc", 0xFFFF, 0xFF00)]
#[case(0xC5, "bc", 0x0001, 0xFF00)]
#[case(0xD5, "de", 0xFFFF, 0xFF00)]
#[case(0xD5, "de", 0x0001, 0xFF00)]
#[case(0xE5, "hl", 0xFFFF, 0xFF00)]
#[case(0xE5, "hl", 0x0001, 0xFF00)]
#[case(0xF5, "af", 0xFFFF, 0xFF00)]
#[case(0xF5, "af", 0x0001, 0xFF00)]
fn test_push(#[case] opcode: u8, #[case] register: &str, #[case] value: u16, #[case] sp: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_sp(sp)
        .with_reg16(register, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(0x0001)
        .with_clock_cycles(16)
        .with_mem16(sp - 2, value)
        .with_sp(sp - 2)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0xC1, "bc", 0xFFFF, 0xFF00)]
#[case(0xC1, "bc", 0x0001, 0xFF00)]
#[case(0xD1, "de", 0xFFFF, 0xFF00)]
#[case(0xD1, "de", 0x0001, 0xFF00)]
#[case(0xE1, "hl", 0xFFFF, 0xFF00)]
#[case(0xE1, "hl", 0x0001, 0xFF00)]
#[case(0xF1, "af", 0xFFFF, 0xFF00)]
#[case(0xF1, "af", 0x0001, 0xFF00)]
fn test_pop(#[case] opcode: u8, #[case] register: &str, #[case] value: u16, #[case] sp: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_sp(sp)
        .with_mem16(sp, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(0x0001)
        .with_clock_cycles(12)
        .with_sp(sp + 2)
        .with_reg16(register, value)
        .build();
    assert_eq!(sut, expected);
}
