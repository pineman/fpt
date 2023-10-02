use fpt::lr35902::LR35902;

#[derive(Clone)]
#[allow(dead_code)]
struct LR35902Builder {
    lr35902: LR35902,
}

//TODO: build with flags
#[allow(dead_code)]
impl LR35902Builder {
    pub fn new() -> Self {
        Self {
            lr35902: LR35902::new(),
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

    pub fn with_memory(mut self, memory: Vec<u8>) -> LR35902Builder {
        for (i, value) in memory.iter().enumerate() {
            self.lr35902.set_memory8(i as u16, *value);
        }

        self
    }

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
    let builder = LR35902Builder::new().with_memory_byte(0, 0);
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

#[test]
fn test_instr_0x021_ld_hl_d16() {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x21) // instruction LD HL,d16
        .with_memory_byte(0x0001, 2) // lsb of immediate16
        .with_memory_byte(0x0002, 1); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_hl(0x0102) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x031_ld_sp_d16() {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x31) // instruction LD HL,d16
        .with_memory_byte(0x0001, 2) // lsb of immediate16
        .with_memory_byte(0x0002, 1); // msb of immediate16
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(3)
        .with_sp(0x0102) // (1 << 8) + 2 == 0x0102
        .with_clock_cycles(12)
        .build();
    assert_eq!(sut, expected);
}

#[test]
fn test_instr_0x032_ld_hld_a() {
    // Given
    let builder = LR35902Builder::new()
        .with_memory_byte(0x0000, 0x32) // instruction LD HL,d16
        .with_a(0x10)
        .with_hl(0x100);
    let mut sut = builder.clone().build();

    // When
    sut.step();

    // Then
    let expected = builder
        .with_pc(1)
        .with_hl(0xFF) // hl gets decremented
        .with_clock_cycles(8)
        .with_memory_byte(0x100, 0x10)
        .build();
    assert_eq!(sut, expected);
}

fn test_add(a: u16, b: u16, r: u16, f: u16) {
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

#[test]
fn test_instr_0x080_add_a_b() {
    test_add(0xfe, 0x01, 0xff, 0b0000); // no flags
    test_add(0x0f, 0x01, 0x10, 0b0010); // half carry
    test_add(0xff, 0x01, 0x00, 0b1011); // zero, half carry and carry
}
