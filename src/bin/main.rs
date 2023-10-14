use fpt::lr35902::LR35902;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag to active debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut lr = LR35902::new();
    lr.set_debug(args.debug);

    loop {
        if args.debug {
            println!("pc: {:#02X}", lr.pc());
        }
        lr.step();
    }
}
