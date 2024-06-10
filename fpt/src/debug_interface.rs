use regex::Regex;


#[derive(Clone, PartialEq, Debug)]
pub struct Breakpoint {
    pub pc: u16,
    pub active: bool,
}

impl Breakpoint {
    pub fn new(pc: u16, active: bool) -> Self {
        Self {
            pc,
            active
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Watchpoint {
    pub addr: u16,
}

impl Watchpoint {
    pub fn new(addr: u16) -> Self {
        Self {
            addr,
        }
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
    RegisterBreakpoint(u16),
    RegisterWatchpoint(u16),
    ListBreakpoints(Vec<Breakpoint>),
    ListWatchpoints(Vec<Watchpoint>),
}

fn breakpoint_cmd<'a, Args>(mut args: Args) -> DebugCmd 
where
    Args: IntoIterator<Item = &'a str>
{
    DebugCmd::Breakpoint(args.into_iter().next().unwrap().parse::<u16>().unwrap())
}


fn watchpoint_cmd<'a, Args>(mut args: Args) -> DebugCmd 
where
    Args: IntoIterator<Item = &'a str>
{
    DebugCmd::Watchpoint(args.into_iter().next().unwrap().parse::<u16>().unwrap())
}

impl DebugCmd {
    pub fn from_string(cmd: &str) -> DebugCmd {

        let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();

        let tokens = re.find_iter(cmd).map(|m| m.as_str()).collect::<Vec<&str>>();
        let mut args = tokens.iter().skip(1).copied();
        match tokens[0] {
            "c" | "continue" => DebugCmd::Continue,
            "b" | "break" | "breakpoint" => breakpoint_cmd(args),
            "w" | "watch" | "watchpoint" => watchpoint_cmd(args),
            "lb" | "list_breakpoints" => DebugCmd::ListBreakpoints,
            "lw" | "list_watchpoints" => DebugCmd::ListWatchpoints,
            "load"                       => DebugCmd::Load(args.next().unwrap().to_string()),
            _ => DebugCmd::Continue
        }
    }
}

pub trait DebugInterface {
    fn receive_command(&mut self, cmd: &DebugCmd) -> Option<DebugEvent>;
}
