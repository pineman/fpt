#[derive(Debug)]
pub enum DebugCmd {
    Pause,
    Continue,
    Breakpoint(u16),
    Watchpoint(u16),
}

impl DebugCmd {
    pub fn from_string(cmd: &str) -> DebugCmd {
        DebugCmd::Continue
    }
}

enum DebugEvent {
    Breakpoint(u16),
    Watchpoint(u16, u16),
}

pub trait DebugInterface {
    fn receive_command(&mut self, cmd: &DebugCmd);
}
