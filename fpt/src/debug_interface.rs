use std::fmt;

use num_traits::Num;
use regex::Regex;

#[derive(Clone, PartialEq, Debug)]
pub struct Breakpoint {
    pub pc: u16,
    pub active: bool,
}

impl Breakpoint {
    pub fn new(pc: u16, active: bool) -> Self {
        Self { pc, active }
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

#[derive(Debug)]
pub enum DebugCmd {
    Pause,
    Continue,
    Breakpoint(u16),
    Watchpoint(u16),
    Load(String),
    ListBreakpoints,
    ListWatchpoints,
}

#[derive(Debug)]
pub enum DebugEvent {
    Continue,
    RegisterBreakpoint(u16),
    RegisterWatchpoint(u16),
    ListBreakpoints(Vec<Breakpoint>),
    ListWatchpoints(Vec<Watchpoint>),
}

impl fmt::Display for DebugEvent {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebugEvent::Continue => write!(f, "continue\n"),
            DebugEvent::RegisterBreakpoint(pc) => {
                write!(f, "Register breakpoint at pc={:#04X}\n", pc)
            }
            DebugEvent::RegisterWatchpoint(addr) => {
                write!(f, "Register watchpoint at address {:#04X}\n", addr)
            }
            DebugEvent::ListBreakpoints(breakpoints) => {
                write!(f, "breakpoints:\n")?;
                for (i, breakpoint) in breakpoints.into_iter().enumerate() {
                    write!(f, "\t{i}: {:#06X}\n", breakpoint.pc)?;
                }
                Ok(())
            }
            DebugEvent::ListWatchpoints(watchpoints) => {
                write!(f, "watchpoints:\n")?;
                for (i, watchpoint) in watchpoints.into_iter().enumerate() {
                    write!(f, "\t{i}: {:#06X}\n", watchpoint.addr)?;
                }
                Ok(())
            }
        }
    }
}

fn breakpoint_cmd<'a, Args>(mut args: Args) -> Option<DebugCmd>
where
    Args: IntoIterator<Item = &'a str>,
{
    Some(DebugCmd::Breakpoint(parse::<u16>(
        args.into_iter().next()?,
    )?))
}

fn watchpoint_cmd<'a, Args>(mut args: Args) -> Option<DebugCmd>
where
    Args: IntoIterator<Item = &'a str>,
{
    Some(DebugCmd::Watchpoint(parse::<u16>(
        args.into_iter().next()?,
    )?))
}

fn parse<'a, T>(value: &str) -> Option<T>
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
            _ => Some(DebugCmd::Continue),
        }
    }
}

pub trait DebugInterface {
    fn receive_command(&mut self, cmd: &DebugCmd) -> Option<DebugEvent>;
}
