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

#[rstest]
#[case(0xFF00, 0xFF01)]
#[case(0xFFFF, 0x0000)]
fn test_instr_0x022_ld_pointer_hl_increment_from_a(#[case] hl: u16, #[case] hl_inc: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x22)
        .with_hl(hl)
        .with_a(0x1);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(hl_inc)
        .with_mem8(hl, 0x1)
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
#[case(0x100, 0xFF)]
#[case(0x0, 0xFFFF)]
fn test_instr_0x032_ld_hld_a(#[case] hl: u16, #[case] hl_after: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, 0x32) // instruction LD (HL-), a
        .with_a(0xca)
        .with_hl(hl);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(hl_after) // hl gets decremented
        .with_clock_cycles(8)
        .with_mem8(hl, 0xca)
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
        .with_hl(hl)
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
#[case(0x01)]
#[case(0xFF)]
fn test_load_register_from_immediate(
    #[values(
    (0x06, "b"),
    (0x16, "d"),
    (0x26, "h"),
    (0x0e, "c"),
    (0x1e, "e"),
    (0x2e, "l"),
    (0x3e, "a"))]
    _opcode_reg @ (opcode, reg): (u8, &str),
    #[case] d8: u8,
) {
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

    sut.mem8(0xFF00 + (address as u16));

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

    sut.mem8(0xFF00 + (address as u16));

    // Then
    let expected = builder.with_pc(1).with_clock_cycles(8).with_a(0x01).build();
    assert_eq!(sut, expected);
}

#[rstest]
// ADD A,r8
#[case(0x80, 0xfe, "b", 0x01, 0xff, 0b0000, 0b0000)]
#[case(0x80, 0x0f, "b", 0x01, 0x10, 0b0000, 0b0010)]
#[case(0x80, 0xff, "b", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x81, 0xff, "c", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x82, 0xff, "d", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x83, 0xff, "e", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x84, 0xff, "h", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x85, 0xff, "l", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x87, 0x80, "a", 0x80, 0x00, 0b0000, 0b1001)]
#[case(0x87, 0x88, "a", 0x88, 0x10, 0b0000, 0b0011)]
// XOR r8
#[case(0xA8, 0xca, "b", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xA8, 0xca, "b", 0xca, 0x00, 0b0000, 0b1000)]
#[case(0xA9, 0xca, "c", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAA, 0xca, "d", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAB, 0xca, "e", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAC, 0xca, "h", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAD, 0xca, "l", 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAF, 0xca, "a", 0xca, 0x00, 0b0000, 0b1000)]
// AND r8
#[case(0xA0, 0xca, "b", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA0, 0xfe, "b", 0x01, 0x00, 0b0000, 0b1010)]
#[case(0xA1, 0xca, "c", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA2, 0xca, "d", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA3, 0xca, "e", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA4, 0xca, "h", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA5, 0xca, "l", 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA7, 0xca, "a", 0xca, 0xca, 0b0000, 0b0010)]
// OR r8
#[case(0xB0, 0xca, "b", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB0, 0x00, "b", 0x00, 0x00, 0b0000, 0b1000)]
#[case(0xB1, 0xca, "c", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB2, 0xca, "d", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB3, 0xca, "e", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB4, 0xca, "h", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB5, 0xca, "l", 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB7, 0xca, "a", 0xca, 0xca, 0b0000, 0b0000)]
// ADC A,r8
#[case(0x88, 0xfe, "b", 0x01, 0xff, 0b0000, 0b0000)]
#[case(0x88, 0xfd, "b", 0x01, 0xff, 0b0001, 0b0000)]
#[case(0x88, 0x0f, "b", 0x01, 0x10, 0b0000, 0b0010)]
#[case(0x88, 0x0e, "b", 0x01, 0x10, 0b0001, 0b0010)]
#[case(0x88, 0xff, "b", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x88, 0xfe, "b", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x89, 0xff, "c", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x89, 0xfe, "c", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8A, 0xff, "d", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x8A, 0xfe, "d", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8B, 0xff, "e", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x8B, 0xfe, "e", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8C, 0xff, "h", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x8C, 0xfe, "h", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8D, 0xff, "l", 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x8D, 0xfe, "l", 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8F, 0x80, "a", 0x80, 0x00, 0b0000, 0b1001)]
#[case(0x8F, 0x80, "a", 0x80, 0x01, 0b0001, 0b0001)]
#[case(0x8F, 0x88, "a", 0x88, 0x10, 0b0000, 0b0011)]
// SUB A,r8
#[case::sub01(0x90, 0x3E, "b", 0x3E, 0x00, 0b0000, 0b1100)]
#[case::sub02(0x90, 0x3E, "b", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub03(0x90, 0x3E, "b", 0x40, 0xFE, 0b0000, 0b0101)]
#[case::sub04(0x90, 0x01, "b", 0xF1, 0x10, 0b0000, 0b0101)]
#[case::sub05(0x91, 0x3E, "c", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub06(0x92, 0x3E, "d", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub07(0x93, 0x3E, "e", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub08(0x94, 0x3E, "h", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub09(0x95, 0x3E, "l", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sub10(0x97, 0x3E, "a", 0x3E, 0x00, 0b0000, 0b1100)]
// SBC A,r8
#[case::sbc01(0x98, 0x3E, "b", 0x3E, 0x00, 0b0000, 0b1100)]
#[case::sbc01_c(0x98, 0x3E, "b", 0x3D, 0x00, 0b0001, 0b1100)]
#[case::sbc02(0x98, 0x3E, "b", 0x0F, 0x2F, 0b0000, 0b0110)]
#[case::sbc02_c(0x98, 0x3E, "b", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc03(0x98, 0x3E, "b", 0x40, 0xFE, 0b0000, 0b0101)]
#[case::sbc03_c(0x98, 0x3F, "b", 0x40, 0xFE, 0b0001, 0b0101)]
#[case::sbc04(0x98, 0x01, "b", 0xF1, 0x10, 0b0000, 0b0101)]
#[case::sbc04_c(0x98, 0x01, "b", 0xF0, 0x10, 0b0001, 0b0101)]
#[case::sbc05(0x99, 0x3E, "c", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc06(0x9A, 0x3E, "d", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc07(0x9B, 0x3E, "e", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc08(0x9C, 0x3E, "h", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc09(0x9D, 0x3E, "l", 0x0E, 0x2F, 0b0001, 0b0110)]
#[case::sbc10(0x9F, 0x3E, "a", 0x3E, 0xFF, 0b0001, 0b0111)]
fn test_alu8_reg_reg(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] src_reg: &str,
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(a)
        .with_reg8(src_reg, value)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_a(result)
        .with_f(flags_after << 4)
        .with_clock_cycles(4)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
// ADD A,(HL)
#[case(0x86, 0xfe, 0x0001, 0x01, 0xff, 0b0000, 0b0000)]
#[case(0x86, 0xff, 0x0001, 0x01, 0x00, 0b0000, 0b1011)]
#[case(0x86, 0xff, 0xcafe, 0x01, 0x00, 0b0000, 0b1011)]
// ADC A,(HL)
#[case(0x8E, 0xfd, 0x0001, 0x01, 0xff, 0b0001, 0b0000)]
#[case(0x8E, 0xfe, 0x0001, 0x01, 0x00, 0b0001, 0b1011)]
#[case(0x8E, 0xfe, 0xcafe, 0x01, 0x00, 0b0001, 0b1011)]
// XOR (HL)
#[case(0xAE, 0xca, 0x0001, 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xAE, 0x01, 0xcafe, 0x01, 0x00, 0b0000, 0b1000)]
// AND (HL)
#[case(0xA6, 0xca, 0x0001, 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xA6, 0xfe, 0xcafe, 0x01, 0x00, 0b0000, 0b1010)]
// OR (HL)
#[case(0xB6, 0xca, 0x0001, 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xB6, 0x00, 0xcafe, 0x00, 0x00, 0b0000, 0b1000)]
// SUB (HL)
#[case(0x96, 0x3E, 0xcafe, 0x0F, 0x2F, 0b0000, 0b0110)]
// SBC A,(HL)
#[case(0x9E, 0x3E, 0xcafe, 0x0F, 0x2E, 0b0001, 0b0110)]
fn test_alu8_reg_addr(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] hl_addr: u16,
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(a)
        .with_reg16("hl", hl_addr)
        .with_mem8(hl_addr, value)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_a(result)
        .with_f(flags_after << 4)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
// ADD A,d8
#[case(0xC6, 0xfe, 0x01, 0xff, 0b0000, 0b0000)]
#[case(0xC6, 0x0f, 0x01, 0x10, 0b0000, 0b0010)]
#[case(0xC6, 0xff, 0x01, 0x00, 0b0000, 0b1011)]
// XOR d8
#[case(0xEE, 0xca, 0xfe, 0x34, 0b0000, 0b0000)]
#[case(0xEE, 0xca, 0xca, 0x00, 0b0000, 0b1000)]
// AND d8
#[case(0xE6, 0xca, 0xfe, 0xca, 0b0000, 0b0010)]
#[case(0xE6, 0xfe, 0x01, 0x00, 0b0000, 0b1010)]
// OR d8
#[case(0xF6, 0xca, 0xfe, 0xfe, 0b0000, 0b0000)]
#[case(0xF6, 0x00, 0x00, 0x00, 0b0000, 0b1000)]
// ADC A,d8
#[case(0xCE, 0xfe, 0x01, 0xff, 0b0000, 0b0000)]
#[case(0xCE, 0xfd, 0x01, 0xff, 0b0001, 0b0000)]
#[case(0xCE, 0x0f, 0x01, 0x10, 0b0000, 0b0010)]
#[case(0xCE, 0x0e, 0x01, 0x10, 0b0001, 0b0010)]
#[case(0xCE, 0xff, 0x01, 0x00, 0b0000, 0b1011)]
#[case(0xCE, 0xfe, 0x01, 0x00, 0b0001, 0b1011)]
// SUB A,d8
#[case(0xD6, 0x3E, 0x0F, 0x2F, 0b0000, 0b0110)]
// SBC A,d8
#[case(0xDE, 0x3E, 0x0F, 0x2E, 0b0001, 0b0110)]
fn test_alu8_reg_imm(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] value: u8,
    #[case] result: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_mem8(0x0001, value)
        .with_a(a)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_a(result)
        .with_f(flags_after << 4)
        .with_clock_cycles(8)
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
#[rustfmt::skip]
// ADD SP,r8
#[case(0xE8, 0x0FFF,    1i8, 0x1000, 0b0000, 0b0010)]
#[case(0xE8, 0x0FFF, -128i8, 0x0F7F, 0b0000, 0b0000)]
#[case(0xE8, 0x0FFF,  127i8, 0x107E, 0b0000, 0b0010)]
#[case(0xE8, 0xFFFF,    1i8, 0x0000, 0b0000, 0b0011)]
#[case(0xE8, 0x0000,   -1i8, 0xFFFF, 0b0000, 0b0011)]
fn test_alu16_reg_imm(
    #[case] opcode: u8,
    #[case] sp: u16,
    #[case] value: i8,
    #[case] result: u16,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_mem8(0x0001, value as u8)
        .with_sp(sp)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_sp(result)
        .with_f(flags_after << 4)
        .with_clock_cycles(16)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case::not_zero(0x1, 0b0001, 0b0011)]
#[case::zero(0x0, 0b0000, 0b1010)]
// BIT n,REG
fn test_rsb8_reg(
    #[values(
    (0x40, "b", 0),
    (0x41, "c", 0),
    (0x42, "d", 0),
    (0x43, "e", 0),
    (0x44, "h", 0),
    (0x45, "l", 0),
    (0x47, "a", 0),
    (0x48, "b", 1),
    (0x49, "c", 1),
    (0x4A, "d", 1),
    (0x4B, "e", 1),
    (0x4C, "h", 1),
    (0x4D, "l", 1),
    (0x4F, "a", 1),
    (0x50, "b", 2),
    (0x51, "c", 2),
    (0x52, "d", 2),
    (0x53, "e", 2),
    (0x54, "h", 2),
    (0x55, "l", 2),
    (0x57, "a", 2),
    (0x58, "b", 3),
    (0x59, "c", 3),
    (0x5A, "d", 3),
    (0x5B, "e", 3),
    (0x5C, "h", 3),
    (0x5D, "l", 3),
    (0x5F, "a", 3),
    (0x60, "b", 4),
    (0x61, "c", 4),
    (0x62, "d", 4),
    (0x63, "e", 4),
    (0x64, "h", 4),
    (0x65, "l", 4),
    (0x67, "a", 4),
    (0x68, "b", 5),
    (0x69, "c", 5),
    (0x6A, "d", 5),
    (0x6B, "e", 5),
    (0x6C, "h", 5),
    (0x6D, "l", 5),
    (0x6F, "a", 5),
    (0x70, "b", 6),
    (0x71, "c", 6),
    (0x72, "d", 6),
    (0x73, "e", 6),
    (0x74, "h", 6),
    (0x75, "l", 6),
    (0x77, "a", 6),
    (0x78, "b", 7),
    (0x79, "c", 7),
    (0x7A, "d", 7),
    (0x7B, "e", 7),
    (0x7C, "h", 7),
    (0x7D, "l", 7),
    (0x7F, "a", 7))]
    _opcode_src_reg_n @ (opcode, src_reg, n): (u16, &str, u8),
    #[case] value: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem16(0x0000, (opcode << 8) + 0xCB)
        .with_reg8(src_reg, value << n)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_f(flags_after << 4)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case::not_zero(0x1, 0b0001, 0b0011)]
#[case::zero(0x0, 0b0000, 0b1010)]
// BIT n,(HL)
fn test_rsb8_addr(
    #[values(
    (0x46, 0),
    (0x4E, 1),
    (0x56, 2),
    (0x5E, 3),
    (0x66, 4),
    (0x6E, 5),
    (0x76, 6),
    (0x7E, 7))]
    _opcode_n @ (opcode, n): (u16, u8),
    #[case] value: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem16(0x0000, (opcode << 8) + 0xCB)
        .with_mem8(0x0002, value << n)
        .with_reg16("hl", 0x0002)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_f(flags_after << 4)
        .with_clock_cycles(16)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case::not_zero(0x1, 0b0101, 0b0101)]
#[case::zero(0x0, 0b1010, 0b1010)]
fn test_rsb8_reg_reg(
    #[values(
    // RES n,REG
    (0x80, "b", 0, 0),
    (0x81, "c", 0, 0),
    (0x82, "d", 0, 0),
    (0x83, "e", 0, 0),
    (0x84, "h", 0, 0),
    (0x85, "l", 0, 0),
    (0x87, "a", 0, 0),
    (0x88, "b", 1, 0),
    (0x89, "c", 1, 0),
    (0x8A, "d", 1, 0),
    (0x8B, "e", 1, 0),
    (0x8C, "h", 1, 0),
    (0x8D, "l", 1, 0),
    (0x8F, "a", 1, 0),
    (0x90, "b", 2, 0),
    (0x91, "c", 2, 0),
    (0x92, "d", 2, 0),
    (0x93, "e", 2, 0),
    (0x94, "h", 2, 0),
    (0x95, "l", 2, 0),
    (0x97, "a", 2, 0),
    (0x98, "b", 3, 0),
    (0x99, "c", 3, 0),
    (0x9A, "d", 3, 0),
    (0x9B, "e", 3, 0),
    (0x9C, "h", 3, 0),
    (0x9D, "l", 3, 0),
    (0x9F, "a", 3, 0),
    (0xA0, "b", 4, 0),
    (0xA1, "c", 4, 0),
    (0xA2, "d", 4, 0),
    (0xA3, "e", 4, 0),
    (0xA4, "h", 4, 0),
    (0xA5, "l", 4, 0),
    (0xA7, "a", 4, 0),
    (0xA8, "b", 5, 0),
    (0xA9, "c", 5, 0),
    (0xAA, "d", 5, 0),
    (0xAB, "e", 5, 0),
    (0xAC, "h", 5, 0),
    (0xAD, "l", 5, 0),
    (0xAF, "a", 5, 0),
    (0xB0, "b", 6, 0),
    (0xB1, "c", 6, 0),
    (0xB2, "d", 6, 0),
    (0xB3, "e", 6, 0),
    (0xB4, "h", 6, 0),
    (0xB5, "l", 6, 0),
    (0xB7, "a", 6, 0),
    (0xB8, "b", 7, 0),
    (0xB9, "c", 7, 0),
    (0xBA, "d", 7, 0),
    (0xBB, "e", 7, 0),
    (0xBC, "h", 7, 0),
    (0xBD, "l", 7, 0),
    (0xBF, "a", 7, 0),
    // SET n,REG
    (0xC0, "b", 0, 1),
    (0xC1, "c", 0, 1),
    (0xC2, "d", 0, 1),
    (0xC3, "e", 0, 1),
    (0xC4, "h", 0, 1),
    (0xC5, "l", 0, 1),
    (0xC7, "a", 0, 1),
    (0xC8, "b", 1, 1),
    (0xC9, "c", 1, 1),
    (0xCA, "d", 1, 1),
    (0xCB, "e", 1, 1),
    (0xCC, "h", 1, 1),
    (0xCD, "l", 1, 1),
    (0xCF, "a", 1, 1),
    (0xD0, "b", 2, 1),
    (0xD1, "c", 2, 1),
    (0xD2, "d", 2, 1),
    (0xD3, "e", 2, 1),
    (0xD4, "h", 2, 1),
    (0xD5, "l", 2, 1),
    (0xD7, "a", 2, 1),
    (0xD8, "b", 3, 1),
    (0xD9, "c", 3, 1),
    (0xDA, "d", 3, 1),
    (0xDB, "e", 3, 1),
    (0xDC, "h", 3, 1),
    (0xDD, "l", 3, 1),
    (0xDF, "a", 3, 1),
    (0xE0, "b", 4, 1),
    (0xE1, "c", 4, 1),
    (0xE2, "d", 4, 1),
    (0xE3, "e", 4, 1),
    (0xE4, "h", 4, 1),
    (0xE5, "l", 4, 1),
    (0xE7, "a", 4, 1),
    (0xE8, "b", 5, 1),
    (0xE9, "c", 5, 1),
    (0xEA, "d", 5, 1),
    (0xEB, "e", 5, 1),
    (0xEC, "h", 5, 1),
    (0xED, "l", 5, 1),
    (0xEF, "a", 5, 1),
    (0xF0, "b", 6, 1),
    (0xF1, "c", 6, 1),
    (0xF2, "d", 6, 1),
    (0xF3, "e", 6, 1),
    (0xF4, "h", 6, 1),
    (0xF5, "l", 6, 1),
    (0xF7, "a", 6, 1),
    (0xF8, "b", 7, 1),
    (0xF9, "c", 7, 1),
    (0xFA, "d", 7, 1),
    (0xFB, "e", 7, 1),
    (0xFC, "h", 7, 1),
    (0xFD, "l", 7, 1),
    (0xFF, "a", 7, 1))]
    _opcode_src_reg_n_result @ (opcode, src_reg, n, result): (u16, &str, u8, u8),
    #[case] value: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem16(0x0000, (opcode << 8) + 0xCB)
        .with_reg8(src_reg, value << n)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_reg8(src_reg, result << n)
        .with_f(flags_after << 4)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case::not_zero(0x1, 0b0101, 0b0101)]
#[case::zero(0x0, 0b1010, 0b1010)]
// RES n,(HL)
fn test_rsb8_reg_addr(
    #[values(
    (0x86, 0, 0),
    (0x8E, 1, 0),
    (0x96, 2, 0),
    (0x9E, 3, 0),
    (0xA6, 4, 0),
    (0xAE, 5, 0),
    (0xB6, 6, 0),
    (0xBE, 7, 0),
    (0xC6, 0, 1),
    (0xCE, 1, 1),
    (0xD6, 2, 1),
    (0xDE, 3, 1),
    (0xE6, 4, 1),
    (0xEE, 5, 1),
    (0xF6, 6, 1),
    (0xFE, 7, 1))]
    _opcode_n_result @ (opcode, n, result): (u16, u8, u8),
    #[case] value: u8,
    #[case] flags_before: u8,
    #[case] flags_after: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem16(0x0000, (opcode << 8) + 0xCB)
        .with_mem8(0x02, value << n)
        .with_reg16("hl", 0x0002)
        .with_f(flags_before << 4);
    let mut sut = builder.clone().build();

    // When
    sut.step();
    sut.step();

    // Then
    let expected = builder
        .with_pc(2)
        .with_mem8(0x02, result << n)
        .with_f(flags_after << 4)
        .with_clock_cycles(16)
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

    // Then
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
    // Test flags are not changed
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

    // Then
    let expected = builder
        .with_pc(1)
        .with_reg16(reg, result)
        .with_clock_cycles(8)
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0xC2, 0xFF00, "z", true, 3, 12)] // 1
#[case(0xC2, 0xFF00, "z", false, 0xFF03, 16)] // 2
#[case(0xD2, 0xFF00, "c", true, 3, 12)] // 3
#[case(0xD2, 0xFF00, "c", false, 0xFF03, 16)] // 4
#[case(0xCA, 0xFF00, "z", true, 0xFF03, 16)] // 5
#[case(0xCA, 0xFF00, "z", false, 3, 12)] // 6
#[case(0xDA, 0xFF00, "c", true, 0xFF03, 16)] // 7
#[case(0xDA, 0xFF00, "c", false, 3, 12)] // 8
#[case(0xC3, 0xFF00, "z", false, 0xFF03, 16)] // 9
#[case(0xC3, 0xFF00, "z", true, 0xFF03, 16)] // 10
#[case(0xC3, 0xFF00, "c", false, 0xFF03, 16)] // 11
#[case(0xC3, 0xFF00, "c", true, 0xFF03, 16)] // 12
#[case(0x20, 0x000F, "z", true, 2, 8)] // 13
#[case(0x20, 0x000F, "z", false, 0x0011, 12)] // 14
#[case(0x30, 0x000F, "c", true, 2, 8)] // 15
#[case(0x30, 0x000F, "c", false, 0x0011, 12)] // 16
#[case(0x28, 0x000F, "z", true, 0x0011, 12)] // 17
#[case(0x28, 0x000F, "z", false, 2, 8)] // 18
#[case(0x38, 0x000F, "c", true, 0x0011, 12)] // 19
#[case(0x38, 0x000F, "c", false, 2, 8)] // 20
#[case(0x18, 0x000F, "z", false, 0x0011, 12)] // 21
#[case(0x18, 0x000F, "z", true, 0x0011, 12)] // 22
#[case(0x18, 0x000F, "c", false, 0x0011, 12)] // 23
#[case(0x18, 0x000F, "c", true, 0x0011, 12)] // 24
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

#[rstest]
#[rustfmt::skip]
#[case(0x10, 0x10, 0b1100)]
#[case(0x10, 0x11, 0b0111)]
fn test_cp(
    #[values((0xB8, "b"), (0xB9, "c"), (0xBA, "d"), (0xBB, "e"), (0xBC, "h"), (0xBD, "l"))] _opcode_reg @ (opcode, reg): (u8, &str),
    #[case] a: u8,
    #[case] reg_value: u8,
    #[case] flags: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_mem8(0x0000, opcode)
        .with_a(a)
        .with_reg8(reg, reg_value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(0x0001)
        .with_clock_cycles(4)
        .with_f(flags << 4)
        .build();
    assert_eq!(sut, expected);
}
