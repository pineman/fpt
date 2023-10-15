#![feature(iter_intersperse)]

use std::fmt;

use hlua::AnyHashableLuaValue as LuaValue;
use hlua::Lua;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use fpt::lr35902::LR35902;
use std::cell::RefCell;
use std::rc::Rc;

fn fmt_lua_value(lua_value: &LuaValue) -> String {
    match lua_value {
        LuaValue::LuaString(s) => {
            format!("{}", s)
        }
        LuaValue::LuaNil => {
            format!("")
        }
        LuaValue::LuaNumber(i) => {
            format!("{}", i)
        }
        _ => {
            panic!();
        }
    }
}

#[derive(Debug)]
enum Breakpoint {
    Breakpoint(u16),
    OnOpcode(u8),
    OnCB(u8),
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Breakpoint::Breakpoint(pc) => {
                write!(f, "breakpoint: {}", pc)
            }
            Breakpoint::OnOpcode(opcode) => {
                write!(f, "on_opcode: {}", opcode)
            }
            Breakpoint::OnCB(opcode) => {
                write!(f, "on_cb: {}", opcode)
            }
        }
    }
}

impl Breakpoint {
    fn check(&self, lr: &LR35902) -> bool {
        match self {
            Breakpoint::Breakpoint(pc) => lr.pc() == *pc,
            Breakpoint::OnOpcode(opcode) => lr.mem8(lr.pc()) == *opcode,
            Breakpoint::OnCB(opcode) => lr.mem8(lr.pc()) == *opcode && lr.get_next_cb(),
        }
    }
}

struct Debugger {
    lr: LR35902,
    breakpoints: Vec<Breakpoint>,
}

#[allow(dead_code)]
impl Debugger {
    fn new() -> Self {
        let mut lr = LR35902::new();
        lr.set_debug(true);
        Debugger {
            lr,
            breakpoints: Vec::new(),
        }
    }

    fn check(&self) -> bool {
        for breakpoint in self.breakpoints.iter() {
            if breakpoint.check(&self.lr) {
                return true;
            }
        }

        false
    }

    fn start(&mut self) {
        loop {
            if self.check() {
                self.lr.step();
                break;
            }
            self.lr.step();
        }
    }

    fn next(&mut self) {
        self.lr.step();
    }

    fn set_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.push(breakpoint);
    }

    fn list_breakpoints(&self) -> String {
        self.breakpoints
            .iter()
            .map(|breakpoint| breakpoint.to_string())
            .intersperse("\n".to_string())
            .collect::<String>()
    }

    fn pc(&mut self) -> u16 {
        self.lr.pc()
    }
}

fn main() -> Result<()> {
    let debugger = Debugger::new();
    let dbg_pointer = Rc::new(RefCell::new(debugger));
    let mut lua = Lua::new();

    let d1 = dbg_pointer.clone();
    lua.set(
        "continue",
        hlua::function0(move || -> LuaValue {
            d1.borrow_mut().start();
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "next",
        hlua::function0(move || -> LuaValue {
            d1.borrow_mut().next();
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "breakpoints",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaString(d1.borrow_mut().list_breakpoints())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "b",
        hlua::function1(move |opcode: u16| -> LuaValue {
            d1.borrow_mut()
                .set_breakpoint(Breakpoint::Breakpoint(opcode));
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "on_opcode",
        hlua::function1(move |opcode: u8| -> LuaValue {
            d1.borrow_mut().set_breakpoint(Breakpoint::OnOpcode(opcode));
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "on_cb",
        hlua::function1(move |opcode: u8| -> LuaValue {
            d1.borrow_mut().set_breakpoint(Breakpoint::OnCB(opcode));
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "pc",
        hlua::function0(move || -> LuaValue { LuaValue::LuaNumber(d1.borrow_mut().pc().into()) }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "af",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.af().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "bc",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.bc().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "de",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.de().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "hl",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.hl().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "sp",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.sp().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "mem",
        hlua::function1(move |address: u16| -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.mem8(address).into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "next_cb",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaNumber(d1.borrow_mut().lr.next_cb().into())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "clock_cycle",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaString(d1.borrow_mut().lr.clock_cycles().to_string())
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "load_rom",
        hlua::function1(move |filename: String| -> LuaValue {
            let rom = std::fs::read(filename).unwrap();
            d1.borrow_mut().lr.load_rom(rom);
            LuaValue::LuaNil
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "mem_dump",
        hlua::function0(move || -> LuaValue {
            LuaValue::LuaString(
                (0..0xFFFF)
                    .map(|i| format!("{:#02X} {:#02X}", i, d1.borrow_mut().lr.mem8(i)))
                    .intersperse("\n".to_string())
                    .collect::<String>(),
            )
        }),
    );

    let d1 = dbg_pointer.clone();
    lua.set(
        "mem_dump_ranged",
        hlua::function2(move |start:u16, end: u16| -> LuaValue {
            LuaValue::LuaString(
                (start..end)
                    .map(|i| format!("{:#02X} {:#02X}", i, d1.borrow_mut().lr.mem8(i)))
                    .intersperse("\n".to_string())
                    .collect::<String>(),
            )
        }),
    );

    lua.set(
        "print",
        hlua::function1(move |s: String| -> LuaValue {
            println!("{}", s);
            LuaValue::LuaNil
        }),
    );

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
                let result = lua.execute::<LuaValue>(&cmd);
                println!(
                    "{}",
                    fmt_lua_value(result.as_ref().expect("Failed to run function"))
                );
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
