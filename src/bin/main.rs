use std::fs;

use fpt::Gameboy;
use fpt::DebuggerTextInterface;

use clap::{Parser, Subcommand, Args};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct Run {
    rom: String,
    #[arg(short, long)]
    /// Flag to active debug output
    debug: Option<bool>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// debugger
    Debug{},
    Run(Run),
}

fn debug() -> Result<()> {
    let mut debugger_interface = DebuggerTextInterface::new();

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".fpt_debug_history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let cmd = String::from("return ") + &line;
                rl.add_history_entry(&line)?;
                debugger_interface.run(cmd);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".fpt_debug_history")?;
    Ok(())
}


fn run(args: Run) -> Result<()>{
    let mut gameboy = Gameboy::new();
    gameboy.set_debug(args.debug.unwrap_or(false));

    let rom = fs::read(args.rom).unwrap();
    gameboy.load_rom(&rom);

    loop {
        if args.debug.unwrap_or(false) {
         println!("pc: {:#02X}", gameboy.cpu().pc());
        }
        gameboy.step();
    }
}
fn main() -> Result<()>{
    let args = Cli::parse();

    match args.command {
        Commands::Debug{} => { debug()},
        Commands::Run(args) => {run(args)},
    }
}
