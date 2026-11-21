pub mod disks;
pub mod network;

pub const REFRESH_IN_SECS: u64 = 1;
pub const EXCLUDED_NETWORK_INTERFACES: &[&str] = &[
    "^lo$",
    "^virbr0.*",
    "^docker0$",
    "^br-.*",
    "^veth.*",
    "^vmnet.*",
    "^vboxnet.*",
];

#[derive(Debug, Default)]
pub struct Io {
    pub disks: disks::Disks,
    pub network: network::Network,
}
