use os_info::Type;

/// Names accepted by `rtop fetch --logo <LOGO>`.
pub const LOGO_NAMES: &[&str] = &[
    "almalinux",
    "alpine",
    "arch",
    "centos",
    "debian",
    "endeavouros",
    "fedora",
    "freebsd",
    "gentoo",
    "kali",
    "linux",
    "macos",
    "manjaro",
    "mint",
    "nixos",
    "opensuse",
    "popos",
    "redhat",
    "rocky",
    "tux",
    "ubuntu",
    "void",
    "windows",
];

/// Returns the logo for an explicit `--logo` name.
pub fn get_logo(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "almalinux" => Some(include_str!("../../assets/almalinux.txt")),
        "alpine" => Some(include_str!("../../assets/alpine.txt")),
        "arch" => Some(include_str!("../../assets/arch.txt")),
        "centos" => Some(include_str!("../../assets/centos.txt")),
        "debian" => Some(include_str!("../../assets/debian.txt")),
        "endeavouros" => Some(include_str!("../../assets/endeavouros.txt")),
        "fedora" => Some(include_str!("../../assets/fedora.txt")),
        "freebsd" => Some(include_str!("../../assets/freebsd.txt")),
        "gentoo" => Some(include_str!("../../assets/gentoo.txt")),
        "kali" => Some(include_str!("../../assets/kali.txt")),
        "linux" | "tux" => Some(include_str!("../../assets/tux.txt")),
        "macos" => Some(include_str!("../../assets/macos.txt")),
        "manjaro" => Some(include_str!("../../assets/manjaro.txt")),
        "mint" => Some(include_str!("../../assets/mint.txt")),
        "nixos" => Some(include_str!("../../assets/nixos.txt")),
        "opensuse" => Some(include_str!("../../assets/opensuse.txt")),
        "popos" => Some(include_str!("../../assets/popos.txt")),
        "redhat" => Some(include_str!("../../assets/redhat.txt")),
        "rocky" => Some(include_str!("../../assets/rocky.txt")),
        "ubuntu" => Some(include_str!("../../assets/ubuntu.txt")),
        "void" => Some(include_str!("../../assets/void.txt")),
        "windows" => Some(include_str!("../../assets/windows.txt")),
        _ => None,
    }
}

pub fn get_distro_logo() -> &'static str {
    match os_info::get().os_type() {
        Type::Arch | Type::Artix | Type::CachyOS | Type::Garuda => {
            include_str!("../../assets/arch.txt")
        }
        Type::Ubuntu | Type::Elementary | Type::Zorin | Type::KDENeon | Type::Raspbian => {
            include_str!("../../assets/ubuntu.txt")
        }
        Type::Debian => include_str!("../../assets/debian.txt"),
        Type::Fedora | Type::Nobara | Type::Bazzite | Type::Bluefin | Type::Ultramarine => {
            include_str!("../../assets/fedora.txt")
        }
        Type::Mint => include_str!("../../assets/mint.txt"),
        Type::openSUSE | Type::SUSE => include_str!("../../assets/opensuse.txt"),
        Type::Manjaro => include_str!("../../assets/manjaro.txt"),
        Type::Gentoo => include_str!("../../assets/gentoo.txt"),
        Type::Alpine => include_str!("../../assets/alpine.txt"),
        Type::NixOS => include_str!("../../assets/nixos.txt"),
        Type::Void => include_str!("../../assets/void.txt"),
        Type::Kali => include_str!("../../assets/kali.txt"),
        Type::Pop => include_str!("../../assets/popos.txt"),
        Type::EndeavourOS => include_str!("../../assets/endeavouros.txt"),
        Type::RockyLinux => include_str!("../../assets/rocky.txt"),
        Type::AlmaLinux => include_str!("../../assets/almalinux.txt"),
        Type::CentOS => include_str!("../../assets/centos.txt"),
        Type::Redhat | Type::RedHatEnterprise | Type::OracleLinux | Type::Amazon => {
            include_str!("../../assets/redhat.txt")
        }
        Type::FreeBSD
        | Type::OpenBSD
        | Type::NetBSD
        | Type::DragonFly
        | Type::HardenedBSD
        | Type::MidnightBSD => include_str!("../../assets/freebsd.txt"),
        Type::Macos => include_str!("../../assets/macos.txt"),
        Type::Windows => include_str!("../../assets/windows.txt"),
        Type::Linux | Type::Unknown => include_str!("../../assets/tux.txt"),
        _ => include_str!("../../assets/tux.txt"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_logo_name_resolves_to_an_asset() {
        for name in LOGO_NAMES {
            let logo = get_logo(name).unwrap_or_else(|| panic!("missing logo for {name}"));

            assert!(logo.contains('\n'), "{name} has no color header");
        }
    }

    #[test]
    fn logo_names_are_case_insensitive() {
        assert_eq!(get_logo("WINDOWS"), get_logo("windows"));
        assert_eq!(get_logo("Linux"), get_logo("tux"));
    }

    #[test]
    fn unknown_logo_name_is_rejected() {
        assert_eq!(get_logo("nope"), None);
    }

    #[test]
    fn logos_do_not_repeat_their_base_color_at_the_start() {
        for name in LOGO_NAMES {
            let logo = get_logo(name).unwrap();
            let (base, art) = logo.split_once('\n').unwrap();
            let first_line = art.lines().next().unwrap().trim_start();
            let redundant_marker = format!("&({{{}}})", base.trim()).to_ascii_lowercase();

            assert!(
                !first_line
                    .to_ascii_lowercase()
                    .starts_with(&redundant_marker),
                "{name} repeats its base color at the start of the art"
            );
        }
    }
}
