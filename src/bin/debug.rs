use hlua::Lua;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use fpt::lr35902::LR35902;

struct Debugger {
    lr: LR35902,
    breakpoints: Vec<u16>,
}

#[allow(dead_code)]
impl Debugger {
    fn new() -> Self {
        Debugger {
            lr: LR35902::new(),
            breakpoints: Vec::new(),
        }
    }
    fn start(&mut self) {
        loop {
            self.lr.step();
        }
    }
}

fn main() -> Result<()> {
    let mut debugger = Debugger::new();
    let mut lua = Lua::new();

    lua.set("start", hlua::function0(move || { 
        debugger.start();
    }));

    lua.set("print", hlua::function1(move |s: String| { 
        println!("{}",s);
    }));

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".fpt_debug_history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                lua.execute::<()>(&line).unwrap();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(".fpt_debug_history");
    Ok(())

}
