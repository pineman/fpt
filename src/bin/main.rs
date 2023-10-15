use std::fs;

use fpt::lr35902::LR35902;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    rom: String,
    /// Flag to active debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut lr = LR35902::new();
    lr.set_debug(args.debug);

    let rom = fs::read(args.rom).unwrap();
    lr.load_rom(rom);

    loop {
        if args.debug {
            println!("pc: {:#02X}", lr.pc());
        }
        lr.step();
    }
}
