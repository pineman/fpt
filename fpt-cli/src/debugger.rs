use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use fpt::Gameboy;
use hlua::AnyHashableLuaValue as LuaValue;
use hlua::Lua;

const ALIASES: [(&str, &str); 7] = [
    ("b", "_G['break']"),
    ("break", "_G['break']"),
    ("c", "_G['continue']"),
    ("continue", "_G['continue']"),
    ("load", "load_rom"),
    ("n", "debug_commands['next']"),
    ("next", "debug_commands['next']"),
];

fn alias_expand(cmd: String, dti: &mut DebuggerTextInterface) -> String {
    ALIASES.iter().fold(cmd, |cmd, (name, value)| {
        let name_with_space = name.to_string() + " ";
        let name_with_paren = name.to_string() + "(";
        let value_with_paren = value.to_string() + "(";

        if cmd == *name || cmd.starts_with(&name_with_space) {
            if cmd == *name {
                dti.last_repeatable_command = Some(format!("{value}()"));
                format!("{value}()")
            } else if !cmd.starts_with(&(name_with_space.clone() + "'")) {
                cmd.replacen(&name_with_space, &(value_with_paren.clone() + "'"), 1) + "')"
            } else {
                cmd.replacen(&name_with_space, &value_with_paren, 1) + ")"
            }
        } else if cmd.starts_with(&name_with_paren) {
            cmd.replace(&name_with_paren, &value_with_paren)
        } else {
            cmd
        }
    })
}

#[derive(Debug)]
pub enum Breakpoint {
    OnPc(u16),
    OnOpcode(u8),
    OnCB(u8),
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            Breakpoint::OnCB(opcode) => {
                gameboy.cpu().mem8(gameboy.cpu().pc()) == *opcode && gameboy.cpu().next_cb()
            }
        }
    }
}
pub struct Debugger {
    gameboy: Rc<RefCell<Gameboy>>,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    fn new() -> Self {
        let gameboy = Rc::new(RefCell::new(Gameboy::new()));
        Self::with_gameboy(gameboy)
    }

    pub fn with_gameboy(gameboy: Rc<RefCell<Gameboy>>) -> Self {
        Self {
            gameboy,
            breakpoints: Vec::new(),
        }
    }

    pub fn check(&self) -> bool {
        for breakpoint in &self.breakpoints {
            if breakpoint.check(&self.gameboy.borrow()) {
                return true;
            }
        }

        false
    }

    pub fn start(&mut self) {
        let mut gameboy = self.gameboy.borrow_mut();
        loop {
            println!(
                "{:#02X}: {}",
                gameboy.cpu().pc(),
                gameboy.cpu().decode().mnemonic
            );
            if self.check() {
                gameboy.instruction();
                break;
            }
            gameboy.instruction();
        }
    }

    pub fn next(&mut self) {
        let mut gameboy = self.gameboy.borrow_mut();
        println!(
            "{:#02X}: {}",
            gameboy.cpu().pc(),
            gameboy.cpu().decode().mnemonic
        );
        gameboy.instruction();
    }

    pub fn set_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.push(breakpoint);
    }

    pub fn list_breakpoints(&self) -> String {
        self.breakpoints
            .iter()
            .map(|breakpoint| breakpoint.to_string())
            .intersperse("\n".to_string())
            .collect::<String>()
    }

    pub fn pc(&self) -> u16 {
        self.gameboy.borrow().cpu().pc()
    }
}

pub struct DebuggerTextInterface<'a> {
    lua: Lua<'a>,
    last_repeatable_command: Option<String>,
}

impl DebuggerTextInterface<'_> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let debugger = Debugger::new();
        let mut lua = Lua::new();
        lua.openlibs();

        {
            let mut aliases_table = lua.empty_array("_aliases");
            for (name, value) in ALIASES {
                aliases_table.set(name, value);
            }
        }

        lua.execute::<()>(
            r#"
            function _help()
                available_functions = {}
                for name, _fn in pairs(debug_commands) do
                  table.insert(available_functions, name)
                end
                table.sort(available_functions)

                s = "Available debugging functions"
                s = "\n" .. s .. "\n" .. ("-"):rep(#s) .. "\n"
                for _i, name in ipairs(available_functions) do
                  s = s .. "  - " .. name .. "\n"
                end

                s = s .. "\nAlias    Expansion\n-------- -----------\n"
                for alias, expansion in pairs(_aliases) do
                  s = s .. string.format("%-8s %s\n", alias, expansion)
                end
                return s
            end

            setmetatable(_G, {
              __index = function(_, key)
                if debug_commands[key] then
                  return debug_commands[key]
                else
                  return _help()
                end
              end
            })
            "#,
        )
        .unwrap();

        {
            let mut debug_commands = lua.empty_array("debug_commands");

            let dbg_pointer = Rc::new(RefCell::new(debugger));
            let d1 = dbg_pointer.clone();

            debug_commands.set(
                "continue",
                hlua::function0(move || -> LuaValue {
                    d1.borrow_mut().start();
                    LuaValue::LuaNil
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "next",
                hlua::function0(move || -> LuaValue {
                    d1.borrow_mut().next();
                    LuaValue::LuaNil
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "breakpoints",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaString(d1.borrow_mut().list_breakpoints())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "break",
                hlua::function1(move |pc: u16| -> LuaValue {
                    d1.borrow_mut().set_breakpoint(Breakpoint::OnPc(pc));
                    LuaValue::LuaString(format!("set breakpoint on pc: {}", pc))
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "break_on_opcode",
                hlua::function1(move |opcode: u8| -> LuaValue {
                    d1.borrow_mut().set_breakpoint(Breakpoint::OnOpcode(opcode));
                    LuaValue::LuaString(format!("set breakpoint on opcode: {}", opcode))
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "break_on_cb",
                hlua::function1(move |opcode: u8| -> LuaValue {
                    d1.borrow_mut().set_breakpoint(Breakpoint::OnCB(opcode));
                    LuaValue::LuaString(format!("set breakpoint on cb: {}", opcode))
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "pc",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().pc().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "af",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().af().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "bc",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().bc().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "de",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().de().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "hl",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().hl().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "sp",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().sp().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "mem",
                hlua::function1(move |address: u16| -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().mem8(address).into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "next_cb",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaNumber(d1.borrow().gameboy.borrow().cpu().next_cb().into())
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "clock_cycle",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaString(
                        d1.borrow()
                            .gameboy
                            .borrow()
                            .cpu()
                            .clock_cycles()
                            .to_string(),
                    )
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "load_rom",
                hlua::function1(move |filename: String| -> LuaValue {
                    let rom = std::fs::read(filename).unwrap();
                    d1.borrow().gameboy.borrow_mut().load_rom(&rom);
                    let game_name = String::from_utf8(
                        d1.borrow()
                            .gameboy
                            .borrow()
                            .bus()
                            .memory_mut()
                            .slice(0x134..0x143)
                            .to_vec(),
                    )
                    .unwrap_or("???".to_string());
                    LuaValue::LuaString(format!("Loaded [{game_name}]"))
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "mem_dump",
                hlua::function0(move || -> LuaValue {
                    LuaValue::LuaString(
                        (0..0xFFFF)
                            .map(|i| {
                                format!(
                                    "{:#02X} {:#02X}",
                                    i,
                                    d1.borrow().gameboy.borrow().cpu().mem8(i)
                                )
                            })
                            .intersperse("\n".to_string())
                            .collect::<String>(),
                    )
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "mem_dump_ranged",
                hlua::function2(move |start: u16, end: u16| -> LuaValue {
                    LuaValue::LuaString(
                        (start..end)
                            .map(|i| {
                                format!(
                                    "{:#02X} {:#02X}",
                                    i,
                                    d1.borrow().gameboy.borrow().cpu().mem8(i)
                                )
                            })
                            .intersperse("\n".to_string())
                            .collect::<String>(),
                    )
                }),
            );

            let d1 = dbg_pointer.clone();
            debug_commands.set(
                "screenshot",
                hlua::function1(move |filename: String| -> LuaValue {
                    // Assumes the user wants a .pgm file
                    let mut file = File::create(&filename)
                        .unwrap_or_else(|_| panic!("Couldn't open file \"{filename}\""));

                    // TODO: code dedup
                    // Write the header for a 160x144 PGM image with 4 shades of gray
                    write!(file, "P2\n# Game Boy screenshot: {filename}\n160 140\n3\n")
                        .expect("Couldn't write PGM header");

                    // Our Game Boy's framebuffer seems to have a direct correspondence to this!
                    let d1 = d1.borrow();
                    let gameboy = d1.gameboy.borrow();
                    let frame = gameboy.get_frame();

                    for line in frame.array_chunks::<160>() {
                        let pgm_line = line
                            .iter()
                            .map(|p| (b'3' - *p) as char) // ASCII from '0' to '3'
                            .intersperse(' ')
                            .collect::<String>()
                            + "\n";
                        file.write_all(pgm_line.as_bytes())
                            .expect("Couldn't write PGM line");
                    }

                    // Report success
                    LuaValue::LuaString(format!("Screenshot written to {filename}\n"))
                }),
            );
        }

        Self {
            lua,
            last_repeatable_command: None,
        }
    }

    pub fn run(&mut self, mut cmd: String) {
        if cmd.is_empty() {
            match self.last_repeatable_command {
                Some(ref repeatable_command) => cmd.clone_from(repeatable_command),
                None => return,
            }
        }

        let expanded_cmd = alias_expand(cmd.clone(), self);
        let expanded_cmd = format!("print({expanded_cmd})");
        // eprintln!("[VERBOSE]    input command: {}", &cmd);
        // eprintln!("[VERBOSE] expanded command: {}", &expanded_cmd);

        let result = self.lua.execute::<LuaValue>(&expanded_cmd);

        if let Err(err) = result {
            eprintln!("{}", err);
        }
    }
}
