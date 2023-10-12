use fpt::lr35902::LR35902;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut lr = LR35902::new();
    lr.set_debug(args.debug);

    loop {
        if lr.pc() > 255 {
            break;
        }
        if args.debug {
            println!("pc: {}", lr.pc());
        }
        let instruction = lr.decode();
        println!(
            "{:#02X}: {:#02X} {}",
            lr.pc(),
            instruction.opcode,
            instruction.mnemonic
        );

        if instruction.size == 0 {
            panic!();
        }
        lr.set_pc(lr.pc() + instruction.size as u16);
    }
}
