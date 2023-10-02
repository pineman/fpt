use rstest::rstest;

use fpt::lr35902::LR35902;

#[derive(Clone)]
struct LR35902Builder {
    lr35902: LR35902,
}

//TODO: build with flags
impl LR35902Builder {
    pub fn new() -> Self {
        Self {
            lr35902: LR35902::default(),
        }
    }

    pub fn with_af(mut self, af: u16) -> LR35902Builder {
        self.lr35902.set_af(af);
        self
    }

    pub fn with_a(mut self, a: u8) -> LR35902Builder {
        self.lr35902.set_a(a);
        self
    }

    pub fn with_bc(mut self, bc: u16) -> LR35902Builder {
        self.lr35902.set_bc(bc);
        self
    }

    pub fn with_de(mut self, de: u16) -> LR35902Builder {
        self.lr35902.set_de(de);
        self
    }

    pub fn with_hl(mut self, hl: u16) -> LR35902Builder {
        self.lr35902.set_hl(hl);
        self
    }

    pub fn with_sp(mut self, sp: u16) -> LR35902Builder {
        self.lr35902.set_sp(sp);
        self
    }

    pub fn with_pc(mut self, pc: u16) -> LR35902Builder {
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
        self.lr35902.set_memory8(index, value);
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

#[test]
fn test_instr_0x001_ld_bc_d16() {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x1) // instruction LD BC,d16
        .with_memory_byte(0x0001, 2) // lsb of immediate16
        .with_memory_byte(0x0002, 1); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_bc(0x0102) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x011_ld_de_d16() {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x11) // instruction LD DE,d16
        .with_memory_byte(0x0001, 2) // lsb of immediate16
        .with_memory_byte(0x0002, 1); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_de(0x0102) // (1 << 8) + 2 == 0x0102
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
#[case(0xfe, 0x01, 0xff, 0b0000)] // no flags
#[case(0x0f, 0x01, 0x10, 0b0010)] // half carry
#[case(0xff, 0x01, 0x00, 0b1011)] // zero, half carry and carry
fn test_add(#[case] a: u16, #[case] b: u16, #[case] r: u16, #[case] f: u16) {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x80) // instruction ADD AF, BC
        .with_af(a << 8)
        .with_bc(b << 8);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_af((r << 8) + (f << 4))
        .with_bc(b << 8)
        .with_clock_cycles(4)
        .build();
    assert_eq!(sut, expected);
}
