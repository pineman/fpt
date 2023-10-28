
use std::fmt;

use hlua::AnyHashableLuaValue as LuaValue;
use hlua::Lua;

use crate::Gameboy;
use std::cell::RefCell;
use std::rc::Rc;

fn fmt_lua_value(lua_value: &LuaValue) -> String {
    match lua_value {
        LuaValue::LuaString(s) => s.to_string(),
        LuaValue::LuaNil => String::new(),
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
    OnPc(u16),
    OnOpcode(u8),
    OnCB(u8),
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Breakpoint::OnPc(pc) => {
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
    fn check(&self, gameboy: &Gameboy) -> bool {
        match self {
            Breakpoint::OnPc(pc) => gameboy.cpu().pc() == *pc,
            Breakpoint::OnOpcode(opcode) => gameboy.cpu().mem8(gameboy.cpu().pc()) == *opcode,
            Breakpoint::OnCB(opcode) => gameboy.cpu().mem8(gameboy.cpu().pc()) == *opcode && gameboy.cpu().get_next_cb(),
        }
    }
}
struct Debugger {
    gameboy: Gameboy,
    breakpoints: Vec<Breakpoint>,
}

#[allow(dead_code)]
impl Debugger {
    fn new() -> Debugger {
        let mut gameboy = Gameboy::new();
        gameboy.set_debug(true);

        let mut debugger = Debugger {
            gameboy,
            breakpoints: Vec::new(),
        };



        debugger
    }

    fn check(&self) -> bool {
        for breakpoint in self.breakpoints.iter() {
            if breakpoint.check(&self.gameboy) {
                return true;
            }
        }

        false
    }

    fn start(&mut self) {
        loop {
            if self.check() {
                self.gameboy.step();
                break;
            }
            self.gameboy.step();
        }
    }

    fn next(&mut self) {
        self.gameboy.step();
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
        self.gameboy.cpu().pc()
    }

    //fn step(&mut self, cmd: String) {
    //    let result = self.lua.execute::<LuaValue>(&cmd);
    //    println!(
    //        "{}",
    //        fmt_lua_value(result.as_ref().expect("Failed to run function"))
    //    );
    //}
}

pub struct DebuggerTextInterface<'a> {
    lua: Lua<'a>,
}

impl DebuggerTextInterface<'_> {
    pub fn new() -> Self {
        let debugger = Debugger::new();
        let mut lua = Lua::new();

        let dbg_pointer = Rc::new(RefCell::new(debugger));
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
                d1.borrow_mut().set_breakpoint(Breakpoint::OnPc(opcode));
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
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().af().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "bc",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().bc().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "de",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().de().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "hl",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().hl().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "sp",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().sp().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "mem",
            hlua::function1(move |address: u16| -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().mem8(address).into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "next_cb",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaNumber(d1.borrow_mut().gameboy.cpu().next_cb().into())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "clock_cycle",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaString(d1.borrow_mut().gameboy.cpu().clock_cycles().to_string())
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "load_rom",
            hlua::function1(move |filename: String| -> LuaValue {
                let rom = std::fs::read(filename).unwrap();
                d1.borrow_mut().gameboy.load_rom(&rom);
                LuaValue::LuaNil
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "mem_dump",
            hlua::function0(move || -> LuaValue {
                LuaValue::LuaString(
                    (0..0xFFFF)
                        .map(|i| format!("{:#02X} {:#02X}", i, d1.borrow_mut().gameboy.cpu().mem8(i)))
                        .intersperse("\n".to_string())
                        .collect::<String>(),
                )
            }),
        );

        let d1 = dbg_pointer.clone();
        lua.set(
            "mem_dump_ranged",
            hlua::function2(move |start: u16, end: u16| -> LuaValue {
                LuaValue::LuaString(
                    (start..end)
                        .map(|i| format!("{:#02X} {:#02X}", i, d1.borrow_mut().gameboy.cpu().mem8(i)))
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

        Self {
            lua,
        }
    }

    pub fn run(&mut self, cmd: String) {
        let value = self.lua.execute::<LuaValue>(&cmd);
        println!("{}", fmt_lua_value(&value.unwrap()));

    }
}
