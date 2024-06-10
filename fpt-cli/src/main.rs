#![feature(array_chunks)]
#![feature(iter_intersperse)]

use std::fs;

use clap::{Args, Parser, Subcommand, ValueEnum};
use debugger::DebuggerTextInterface;
use fpt::Gameboy;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub mod debugger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    gameboy_config: GameboyConfig,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct GameboyConfig {
    #[arg(short, long)]
    fake_bootrom: Option<BootromToFake>,
}

impl GameboyConfig {
    /// Build a Gameboy following this configuration. Consumes self.
    pub fn build_gameboy(self: Self) -> Gameboy {
        let mut gameboy = Gameboy::new();
        if let Some(BootromToFake::DMG0) = self.fake_bootrom {
            gameboy.simulate_dmg0_bootrom_handoff_state();
        }
        gameboy
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum BootromToFake {
    DMG0,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Debug {},
    Dump(Dump),
    Run(Run),
}

#[derive(Debug, Args)]
struct Dump {
    rom: String,
}

#[derive(Debug, Args)]
struct Run {
    rom: String,
    #[arg(short, long)]
    debug: Option<bool>,
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
                rl.add_history_entry(&line)?;
                debugger_interface.run(line);
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

fn dump(args: Dump) -> Result<()> {
    let mut gb = Gameboy::new();
    let rom = fs::read(args.rom).unwrap();
    gb.load_rom(&rom);
    loop {
        let inst = gb.cpu().decode();
        let result: Vec<String> = (1..inst.size)
            .map(|i| format!("{:#02X}", gb.cpu().mem8(gb.cpu().pc() + i as u16)))
            .collect();
        println!(
            "{:#02X}: {:?} ({:#02X}{}{})",
            gb.cpu().pc(),
            inst,
            inst.opcode,
            if result.is_empty() { "" } else { " " },
            result.join(" ")
        );
        // TODO: this is very, very stupid as it doesn't follow jumps, so it can
        // read data as code. how do decompilers even work?
        let next_pc = gb.cpu().pc() + inst.size as u16;
        gb.cpu_mut().set_pc(next_pc);
    }
}

fn run(gb_config: GameboyConfig, args: Run) -> Result<()> {
    let mut gameboy = gb_config.build_gameboy();

    let rom = fs::read(args.rom)?;
    gameboy.load_rom(&rom);
    loop {
        if args.debug.unwrap_or(false) {
            println!("{:#02X}: {:?}", gameboy.cpu().pc(), gameboy.cpu().decode());
        }
        gameboy.instruction();
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let gb_config = args.gameboy_config;

    match args.command {
        Commands::Debug {} => debug(),
        Commands::Dump(args) => dump(args),
        Commands::Run(args) => run(gb_config, args),
    }
}
