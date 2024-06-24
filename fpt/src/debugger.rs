use std::collections::VecDeque;

use crate::debug_interface::{Breakpoint, DebugCmd, DebugEvent, Watchpoint};

#[derive(Clone, PartialEq)]
pub struct Debugger {
    breakpoints: Vec<Breakpoint>,
    watchpoints: Vec<Watchpoint>,
    pub paused: bool,
    dbg_events: VecDeque<DebugEvent>,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
            paused: false,
            dbg_events: VecDeque::new(),
        }
    }

    pub fn receive_command(&mut self, cmd: &DebugCmd) -> Option<DebugEvent> {
        match cmd {
            DebugCmd::Pause => {
                self.paused = true;
                None
            }
            DebugCmd::Continue => {
                self.paused = false;
                Some(DebugEvent::Continue)
            }
            DebugCmd::Breakpoint(pc) => {
                self.breakpoints.push(Breakpoint {
                    pc: *pc,
                    triggered: false,
                });
                Some(DebugEvent::RegisterBreakpoint(*pc))
            }
            DebugCmd::Watchpoint(addr) => {
                self.watchpoints.push(Watchpoint { addr: *addr });
                Some(DebugEvent::RegisterWatchpoint(*addr))
            }
            DebugCmd::ListBreakpoints => {
                Some(DebugEvent::ListBreakpoints(self.breakpoints.clone()))
            }
            DebugCmd::ListWatchpoints => {
                Some(DebugEvent::ListWatchpoints(self.watchpoints.clone()))
            }
            DebugCmd::Load(_) => None,
        }
    }

    pub fn match_breakpoint(&mut self, pc: u16) -> bool {
        let breakpoint = self.breakpoints.iter_mut().find(|b| b.pc == pc);

        let mut pc = 0;
        if breakpoint.is_some() {
            let breakpoint = breakpoint.unwrap();
            pc = breakpoint.pc;
            if breakpoint.triggered {
                breakpoint.triggered = false;
            } else {
                breakpoint.triggered = true;
                self.paused = true;
            }
        }

        if self.paused {
            self.dbg_events.push_back(DebugEvent::Breakpoint(pc));
        }

        self.paused
    }

    pub fn debug_events(&mut self) -> &mut VecDeque<DebugEvent> {
        &mut self.dbg_events
    }
}
