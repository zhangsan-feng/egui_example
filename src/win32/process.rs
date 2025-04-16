use windows::{
    core::*,
    Win32::{
        System::{
            ProcessStatus::{EnumProcesses, EnumProcessModules, GetModuleBaseNameA},
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
        Foundation::CloseHandle,
    },
};

pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
}

pub struct ProcessInfo {
    pub processes: Vec<Process>,
}

impl ProcessInfo {
    pub fn new() -> Self {
        Self {
            processes: vec![],
        }
    }

    pub fn update(&mut self) {}



}
