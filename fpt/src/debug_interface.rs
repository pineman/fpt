use std::collections::VecDeque;
use std::fmt;

use num_traits::Num;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Breakpoint {
    pub pc: u16,
    pub triggered: bool,
}

impl Breakpoint {
    pub fn new(pc: u16, triggered: bool) -> Self {
        Self { pc, triggered }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Watchpoint {
    pub addr: u16,
}

impl Watchpoint {
    pub fn new(addr: u16) -> Self {
        Self { addr }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Instrpoint {
    pub opcode: u16,
    pub triggered: bool,
}

impl Instrpoint {
    pub fn new(opcode: u16, triggered: bool) -> Self {
        Self { opcode, triggered }
    }
}

#[derive(Debug)]
pub enum DebugCmd {
    Pause,
    Continue,
    Breakpoint(u16),
    Watchpoint(u16),
    Instrpoint(u16),
    Load(String),
    ListBreakpoints,
    ListWatchpoints,
    Print(u16),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DebugEvent {
    Continue,
    RegisterBreakpoint(u16),
    RegisterWatchpoint(u16),
    RegisterInstrpoint(u16),
    ListBreakpoints(Vec<Breakpoint>),
    ListWatchpoints(Vec<Watchpoint>),
    Breakpoint(u16),
    Watchpoint(u16, u16),
    Instrpoint(u16),
    Print(u8),
}

impl fmt::Display for DebugEvent {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebugEvent::Continue => writeln!(f, "continue"),
            DebugEvent::RegisterBreakpoint(pc) => {
                writeln!(f, "Registered breakpoint at pc={:#04X}", pc)
            }
            DebugEvent::RegisterWatchpoint(addr) => {
                writeln!(f, "Registered watchpoint at address {:#04X}", addr)
            }
            DebugEvent::RegisterInstrpoint(opcode) => {
                writeln!(f, "Registered instrpoint with opcode {:#04X}", opcode)
            }
            DebugEvent::ListBreakpoints(breakpoints) => {
                writeln!(f, "breakpoints:")?;
                for (i, breakpoint) in breakpoints.iter().enumerate() {
                    writeln!(f, "\t{i}: {:#06X}", breakpoint.pc)?;
                }
                Ok(())
            }
            DebugEvent::ListWatchpoints(watchpoints) => {
                writeln!(f, "watchpoints:")?;
                for (i, watchpoint) in watchpoints.iter().enumerate() {
                    writeln!(f, "\t{i}: {:#06X}", watchpoint.addr)?;
                }
                Ok(())
            }
            DebugEvent::Breakpoint(pc) => {
                writeln!(f, "Hit breakpoint at {:#06X}", pc)
            }
            DebugEvent::Watchpoint(address, value) => {
                writeln!(
                    f,
                    "Hit watchpoint at {:#06X} with value: {:#06X}",
                    address, value
                )
            }
            DebugEvent::Instrpoint(opcode) => {
                writeln!(f, "Hit instrpoint at {:#06X}", opcode)
            }
            DebugEvent::Print(value) => {
                writeln!(f, "{:#04X}", value)
            }
        }
    }
}

fn breakpoint_cmd<'a, Args>(args: Args) -> Option<DebugCmd>
where
    Args: IntoIterator<Item = &'a str>,
{
    Some(DebugCmd::Breakpoint(parse::<u16>(
        args.into_iter().next()?,
    )?))
}

fn watchpoint_cmd<'a, Args>(args: Args) -> Option<DebugCmd>
where
    Args: IntoIterator<Item = &'a str>,
{
    Some(DebugCmd::Watchpoint(parse::<u16>(
        args.into_iter().next()?,
    )?))
}

fn print_cmd<'a, Args>(args: Args) -> Option<DebugCmd>
where
    Args: IntoIterator<Item = &'a str>,
{
    Some(DebugCmd::Print(parse::<u16>(args.into_iter().next()?)?))
}

fn parse<T>(value: &str) -> Option<T>
where
    T: Num + std::str::FromStr,
{
    let value = value.trim();
    if value.starts_with("0x") {
        Some(<T>::from_str_radix(value.strip_prefix("0x").unwrap(), 16).ok()?)
    } else {
        Some(value.parse::<T>().ok()?)
    }
}

impl DebugCmd {
    pub fn from_string(cmd: &str) -> Option<DebugCmd> {
        let re = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
        let tokens = re.find_iter(cmd).map(|m| m.as_str()).collect::<Vec<&str>>();
        let mut args = tokens.iter().skip(1).copied();
        match tokens[0] {
            "c" | "continue" => Some(DebugCmd::Continue),
            "b" | "break" | "breakpoint" => breakpoint_cmd(args),
            "w" | "watch" | "watchpoint" => watchpoint_cmd(args),
            "lb" | "list_breakpoints" => Some(DebugCmd::ListBreakpoints),
            "lw" | "list_watchpoints" => Some(DebugCmd::ListWatchpoints),
            "load" => Some(DebugCmd::Load(args.next().unwrap().to_string())),
            "p" | "print" => print_cmd(args),
            _ => None,
        }
    }
}

pub trait DebugInterface {
    fn receive_command(&mut self, cmd: &DebugCmd) -> Option<DebugEvent>;
    fn paused(&self) -> bool;
    fn set_paused(&mut self, paused: bool);
    fn get_debug_events(&mut self) -> &mut VecDeque<DebugEvent>;
}
