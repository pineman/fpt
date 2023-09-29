mod instructions;

use instructions::load_instructions;

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: u8,
    clocks: u8,
    size: u8,
    function: fn(&mut LR35902, opcode: u8),
}

impl Instruction {
    fn new(
        opcode: u8,
        clocks: u8,
        size: u8,
        function: fn(&mut LR35902, opcode: u8),
    ) -> Instruction {
        Instruction {
            opcode,
            clocks,
            size,
            function,
        }
    }
}

struct LR35902 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    memory: [u8; 65536],
    instructions: Vec<Instruction>,
}

impl LR35902 {
    fn new() -> LR35902 {
        LR35902 {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            memory: [0; 65536],
            instructions: Vec::new(),
        }
    }

    fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    fn step(&mut self) {
        let opcode = dbg!(self.memory[self.pc as usize]);

        let instruction = &self.instructions[opcode as usize];
        let f = instruction.function;
        f(self, instruction.opcode);
    }
}


fn main() {
    let mut lr35902 = LR35902::new();

    lr35902.memory[..256].clone_from_slice(include_bytes!("../dmg0.bin"));

    lr35902.load_instructions(load_instructions());
    lr35902.step();
}
