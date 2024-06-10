use crate::debug_interface::{DebugCmd, DebugEvent, Breakpoint, Watchpoint};


#[derive(Clone, PartialEq)]
pub struct Debugger {
    breakpoints: Vec<Breakpoint>,
    watchpoints: Vec<Watchpoint>,
    pub paused: bool,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![Breakpoint::new(0, true)],
            watchpoints: Vec::new(),
            paused: false,
        }
    }

    pub fn receive_command(&mut self, cmd: &DebugCmd) -> Option<DebugEvent>{
        match cmd {
            DebugCmd::Pause => {
                self.paused = true;
                None
            }
            DebugCmd::Continue => {
                self.paused = false;
                None
            }
            DebugCmd::Breakpoint(pc) => {
                self.breakpoints.push(Breakpoint {
                    pc: *pc,
                    active: true,
                });
                Some(DebugEvent::RegisterBreakpoint(*pc))
            }
            DebugCmd::Watchpoint(addr) => {
                self.watchpoints.push(Watchpoint {
                    addr: *addr
                });
                Some(DebugEvent::RegisterWatchpoint(*addr))
            },
            DebugCmd::ListBreakpoints => {
                Some(DebugEvent::ListBreakpoints(self.breakpoints.clone()))
            },
            DebugCmd::ListWatchpoints => {
                Some(DebugEvent::ListWatchpoints(self.watchpoints.clone()))
            },
            DebugCmd::Load(_) => {
                None
            }
        }
    }

    pub fn match_breakpoint(&mut self, pc: u16) -> Option<&mut Breakpoint> {
        self.breakpoints.iter_mut().find(|b| b.pc == pc)
    }
}
