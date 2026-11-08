use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Default, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: OsString,
    pub exe: PathBuf,
    pub user: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub threads: usize,

    // precalcolati una volta per refresh, non ad ogni frame
    pub pid_display: String,
    pub threads_display: String,
    pub cpu_display: String,
    pub memory_display: String,
    pub display_name: String,
}
