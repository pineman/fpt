use fpt::instructions::load_instructions;
use fpt::LR35902;

fn main() {
    let mut lr35902 = LR35902::new();

    lr35902.load_bootrom(include_bytes!("../../dmg0.bin"));
    lr35902.load_instructions(load_instructions());

    loop {
        lr35902.step();
    }
}