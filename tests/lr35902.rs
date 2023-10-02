use rstest::rstest;

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

    // pub fn with_reg16(self, register: &str, value: u16) -> Self {
    //     match register {
    //         "bc" => self.with_bc(value),
    //         "de" => self.with_de(value),
    //         "hl" => self.with_hl(value),
    //         _ => panic!(),
    //     }
    // }

    pub fn with_a(mut self, a: u8) -> Self {
        self.lr35902.set_a(a);
        self
    }

    pub fn with_f(mut self, f: u8) -> Self {
        self.lr35902.set_f(f);
        self
    }

    // pub fn with_af(mut self, af: u16) -> Self {
    //     self.lr35902.set_af(af);
    //     self
    // }

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

    pub fn with_clock_cycles(mut self, clock_cycles: u64) -> LR35902Builder {
        self.lr35902.set_clock_cycles(clock_cycles);
        self
    }

    //pub fn with_memory(mut self, memory: Vec<u8>) -> LR35902Builder {
    //    for (i, value) in memory.iter().enumerate() {
    //        self.lr35902.set_memory8(i as u16, *value);
    //    }

    //    self
    //}

    pub fn with_memory_byte(mut self, index: u16, value: u8) -> LR35902Builder {
        self.lr35902.set_mem8(index, value);
        self
    }

    pub fn build(self) -> LR35902 {
        self.lr35902
    }
}

#[test]
fn test_instr_0x000_nop() {
    // Given
    let builder = LR35902Builder::new().with_memory_byte(0x0000, 0x0);
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
        .with_memory_byte(0x0000, 0x1) // instruction LD BC,d16
        .with_memory_byte(0x0001, lsb) // lsb of immediate16
        .with_memory_byte(0x0002, msb); // msb of immediate16
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
#[case(2, 1, 0x0102)]
fn test_instr_0x011_ld_de_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x11) // instruction LD DE,d16
        .with_memory_byte(0x0001, lsb) // lsb of immediate16
        .with_memory_byte(0x0002, msb); // msb of immediate16
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
#[case(0x2, 0x1, 0x0102)]
fn test_instr_0x021_ld_hl_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x21) // instruction LD HL,d16
        .with_memory_byte(0x0001, lsb) // lsb of immediate16
        .with_memory_byte(0x0002, msb); // msb of immediate16
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
#[case(0x2, 0x1, 0x0102)]
#[case(0xFF, 0xFF, 0xFFFF)]
fn test_instr_0x031_ld_sp_d16(#[case] lsb: u8, #[case] msb: u8, #[case] result: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x31) // instruction LD SP,d16
        .with_memory_byte(0x0001, lsb) // lsb of immediate16
        .with_memory_byte(0x0002, msb); // msb of immediate16
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
        .with_memory_byte(0x0000, 0x32) // instruction LD (HL-), a
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
        .with_memory_byte(hl, a)
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
//#[case(0x43, "b", "e", 0x01)] //  7
//#[case(0x43, "b", "e", 0xFF)] //  8
#[case(0x44, "b", "h", 0x01)] //  9
#[case(0x44, "b", "h", 0xFF)] // 10
#[case(0x45, "b", "l", 0x01)] // 11
#[case(0x45, "b", "l", 0xFF)] // 12
#[case(0x47, "b", "a", 0x01)] // 13
#[case(0x47, "b", "a", 0xFF)] // 14
#[case(0x48, "c", "b", 0x01)] // 13
#[case(0x48, "c", "b", 0xFF)] // 14
#[case(0x49, "c", "c", 0x01)] // 15
#[case(0x49, "c", "c", 0xFF)] // 16
#[case(0x4A, "c", "d", 0x01)] // 17
#[case(0x4A, "c", "d", 0xFF)] // 18
//#[case(0x4B, "c", "e", 0x01)] // 19
//#[case(0x4B, "c", "e", 0xFF)] // 20
#[case(0x4C, "c", "h", 0x01)] // 21
#[case(0x4C, "c", "h", 0xFF)] // 22
#[case(0x4D, "c", "l", 0x01)] // 23
#[case(0x4D, "c", "l", 0xFF)] // 24
#[case(0x4F, "c", "a", 0x01)] // 25
#[case(0x4F, "c", "a", 0xFF)] // 26
#[case(0x50, "d", "b", 0x01)] // 27
#[case(0x50, "d", "b", 0xFF)]
// 28
//#[case(0x51, "d", "c", 0x01)] // 29
//#[case(0x51, "d", "c", 0xFF)] // 30
//#[case(0x52, "d", "d", 0x01)] // 31
//#[case(0x52, "d", "d", 0xFF)] // 32
//#[case(0x53, "d", "e", 0x01)] // 33
//#[case(0x53, "d", "e", 0xFF)] // 34
#[case(0x54, "d", "h", 0x01)] // 35
#[case(0x54, "d", "h", 0xFF)] // 36
#[case(0x55, "d", "l", 0x01)] // 37
#[case(0x55, "d", "l", 0xFF)] // 38
#[case(0x57, "d", "a", 0x01)] // 39
#[case(0x57, "d", "a", 0xFF)] // 40
fn test_load_8_bit_reg_to_8_bit_reg(
    #[case] opcode: u8,
    #[case] dst_reg: &str,
    #[case] src_reg: &str,
    #[case] value: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, opcode)
        .with_reg8(src_reg, value);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_clock_cycles(4)
        .with_reg8(dst_reg, value) // hl gets decremented
        .build();
    assert_eq!(sut, expected);
}

#[rstest]
#[case(0x80, "b", 0xfe, 0x01, 0xff, 0b0000)] // no flags
#[case(0x80, "b", 0x0f, 0x01, 0x10, 0b0010)] // half carry
#[case(0x80, "b", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x81, "c", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x82, "d", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x83, "e", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x84, "h", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
#[case(0x85, "l", 0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
fn test_add8(
    #[case] opcode: u8,
    #[case] src_reg: &str,
    #[case] a: u8,
    #[case] y: u8,
    #[case] r: u8,
    #[case] f: u8,
) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, opcode)
        .with_a(a)
        .with_reg8(src_reg, y);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_a(r)
        .with_f(f << 4)
        .with_reg8(src_reg, y)
        .with_clock_cycles(4)
        .build();
    assert_eq!(sut, expected);
}
