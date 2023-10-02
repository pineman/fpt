use fpt::lr35902::LR35902;

#[derive(Clone)]
#[allow(dead_code)]
struct LR35902Builder {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    mem: [u8; 65536],
    //next_cb: bool,
    //instructions: Vec<Instruction>,
    clock_cycles: u64,
}

//TODO: build with flags
#[allow(dead_code)]
impl LR35902Builder {
    pub fn new() -> Self {
        Self {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            mem: [0; 65536],
            //next_cb: false,
            //instructions: vec![],
            clock_cycles: 0,
        }
    }

    pub fn with_af(mut self, af: u16) -> LR35902Builder {
        self.af = af;
        self
    }
    pub fn with_bc(mut self, bc: u16) -> LR35902Builder {
        self.bc = bc;
        self
    }

    pub fn with_pc(mut self, pc: u16) -> LR35902Builder {
        self.pc = pc;
        self
    }

    pub fn with_clock_cycles(mut self, clock_cycles: u64) -> LR35902Builder {
        self.clock_cycles = clock_cycles;
        self
    }

    pub fn with_memory(mut self, memory: Vec<u8>) -> LR35902Builder {
        for (i, value) in memory.iter().enumerate() {
            self.mem[i] = *value;
        }

        self
    }

    pub fn with_memory_byte(mut self, index: usize, value: u8) -> LR35902Builder {
        self.mem[index] = value;
        self
    }

    pub fn build(self) -> LR35902 {
        let mut lr35902 = LR35902::new();

        lr35902.set_af(self.af);
        lr35902.set_bc(self.bc);
        lr35902.set_pc(self.pc);
        lr35902.set_clock_cycles(self.clock_cycles);

        for (i, value) in self.mem.iter().enumerate() {
            lr35902.set_memory8(i.try_into().unwrap(), *value);
        }
        lr35902
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
    test_add(0xfe, 0x01, 0xff, 0b0000);
    test_add(0x0f, 0x01, 0x10, 0b0010);
    test_add(0xff, 0x01, 0x00, 0b1011);
}
