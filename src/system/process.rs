use std::{ffi::OsString, path::PathBuf};

#[derive(Default, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: OsString,
    pub exe: PathBuf,
    pub user: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub threads: usize,
    pub startup_time: u64,

    pub pid_display: String,
    pub threads_display: String,
    pub cpu_display: String,
    pub memory_display: String,
    pub display_name: String,
}

#[derive(Clone)]
pub struct KillRequest {
    pub pid: u32,
    pub name: OsString,
    pub startup_time: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub enum KillResponse {
    NotFound(u32),
    KillFailed(u32),
}
