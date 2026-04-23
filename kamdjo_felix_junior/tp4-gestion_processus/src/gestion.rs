// Module for simplified OS process manager

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Ready,
    Running { cpu_id: u8 },
    Blocked { reason: String },
    Terminated { return_code: i32 },
    Zombie,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Priority {
    VeryLow,
    Low,
    Normal,
    High,
    VeryHigh,
    RealTime(u8),  // 0-99
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub priority: Priority,
    pub memory_kb: u64,
    pub parent_pid: Option<u32>,
}

pub struct ProcessManager {
    pub processes: Vec<Process>,
    pub next_pid: u32,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: vec![],
            next_pid: 1,
        }
    }

    pub fn create_process(
        &mut self,
        name: String,
        priority: Priority,
        memory_kb: u64,
        parent_pid: Option<u32>,
    ) -> u32 {
        let pid = self.next_pid;
        let p = Process {
            pid,
            name,
            state: ProcessState::Ready,
            priority,
            memory_kb,
            parent_pid,
        };
        self.processes.push(p);
        self.next_pid += 1;
        pid
    }

    #[allow(dead_code)]
    pub fn find(&self, pid: u32) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn change_state(
        &mut self,
        pid: u32,
        new_state: ProcessState,
    ) -> Result<(), String> {
        match self.processes.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.state = new_state;
                Ok(())
            }
            None => Err(format!("PID {} not found", pid)),
        }
    }

    pub fn total_memory(&self) -> u64 {
        self.processes.iter().map(|p| p.memory_kb).sum()
    }

    #[allow(dead_code)]
    pub fn processes_by_state(&self, state: &ProcessState) -> Vec<&Process> {
        self.processes
            .iter()
            .filter(|p| &p.state == state)
            .collect()
    }

    pub fn kill_process(&mut self, pid: u32) -> Result<i32, String> {
        match self.processes.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.state = ProcessState::Terminated { return_code: 0 };
                Ok(0)
            }
            None => Err(format!("PID {} not found", pid)),
        }
    }

    pub fn print_summary(&self) {
        println!("=== Process Manager ===");
        for p in &self.processes {
            println!(
                "PID: {}, Name: {}, State: {:?}, Priority: {:?}, Memory: {} KB, Parent: {:?}",
                p.pid, p.name, p.state, p.priority, p.memory_kb, p.parent_pid
            );
        }
        println!("Total memory used: {} KB", self.total_memory());
    }
}