use fpt::lr35902::LR35902;

use clap::Parser;

/// Parse rom and output the disassembly
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag to enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut lr = LR35902::default();

    loop {
        if lr.pc() > 255 {
            break;
        }
        let instruction = lr.decode();

        if args.debug {
            println!("{:#02X}: {}", lr.pc(), instruction);
        }

        if instruction.size == 0 {
            panic!();
        }
        lr.set_pc(lr.pc() + instruction.size as u16);
    }
}
