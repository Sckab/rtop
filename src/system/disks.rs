use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Default)]
pub struct Disk {
    pub name: OsString,
    pub mount: PathBuf,
    pub read: u64,
    pub write: u64,
    pub available: u64,
    pub total: u64,
}

#[derive(Debug, Default)]
pub struct Disks {
    pub disks: Vec<Disk>,
}
