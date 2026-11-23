pub fn get_distro_logo() -> &'static str {
    match os_info::get().os_type() {
        os_info::Type::Arch => include_str!("../../assets/arch.txt"),
        _ => "",
    }
}
