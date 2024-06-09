use crate::debug_interface::DebugCmd;

#[derive(Clone, PartialEq)]
pub struct Breakpoint {
    pub pc: u16,
    pub active: bool,
}
#[derive(Clone, PartialEq)]
pub struct Watchpoint {
    pub addr: u16,
}

#[derive(Clone, PartialEq)]
pub struct Debugger {
    breakpoints: Vec<Breakpoint>,
    watchpoints: Vec<Watchpoint>,
    pub paused: bool,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
            paused: false,
        }
    }

    pub fn receive_command(&mut self, cmd: &DebugCmd) {
        match cmd {
            DebugCmd::Pause => {
                self.paused = true;
            }
            DebugCmd::Continue => {
                self.paused = false;
            }
            DebugCmd::Breakpoint(pc) => {
                self.breakpoints.push(Breakpoint {
                    pc: *pc,
                    active: true,
                });
            }
            DebugCmd::Watchpoint(addr) => {
                self.watchpoints.push(Watchpoint {
                    addr: *addr
                });
            },
        }
    }

    pub fn match_breakpoint(&mut self, pc: u16) -> Option<&mut Breakpoint> {
        self.breakpoints.iter_mut().find(|b| b.pc == pc)
    }
}
