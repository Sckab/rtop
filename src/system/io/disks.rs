use std::{ffi::OsString, path::PathBuf};

use crate::system::sorting::DisksSort;

#[derive(Debug, Default)]
pub struct Disk {
    pub name: OsString,
    pub mount: PathBuf,
    pub read: u64,
    pub write: u64,
    pub available: u64,
    pub total: u64,
}

#[derive(Default)]
pub struct Disks {
    pub disks: Vec<Disk>,
    pub sort: DisksSort,
}
