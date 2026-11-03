use clap::Parser;

mod cli;

fn main() -> anyhow::Result<()> {
    cli::Cli::parse();

    if let Ok(mut sys) = sysinfo::System::new_all() {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        sys.refresh_all();

        let cpus = sys.cpus().len();

        let mut i = 1;
        let mut usage_total = 0.0;

        println!("CPUs: {}", cpus);

        for cpu in sys.cpus() {
            let cpu_usage = cpu.usage();

            println!("  CPU {i} usage: {:.2}%", cpu_usage);

            usage_total += cpu_usage;

            i += 1;
        }

        let mean_usage = usage_total / cpus as f32;
        let mean_usage_formatted = format!("{:.2}%", mean_usage);

        let swap_formatted = format!("{}", sys.used_swap());

        println!("medium usage: {mean_usage_formatted}");
        println!("--------------{}", "-".repeat(mean_usage_formatted.len()));

        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", swap_formatted);

        println!("--------------------{}", "-".repeat(swap_formatted.len()));
    }

    if let Ok(disks) = sysinfo::Disks::new_with_refreshed_list() {
        let mut i = 0;

        for disk in &disks {
            let usage = disk.usage();
            println!(
                "Disk: {} ({}) [{}]",
                whichdisk::resolve(disk.mount_point())?
                    .volume_name()
                    .unwrap(),
                disk.kind(),
                disk.mount_point().display()
            );

            println!("  Filesystem: {}", disk.file_system().display());
            println!("  Usage:");
            println!(
                "  Written bytes: {} (total: {})",
                usage.written_bytes, usage.total_written_bytes
            );
            println!(
                "  Readed bytes: {} (total: {})",
                usage.read_bytes, usage.total_read_bytes
            );
            println!("  Available space: {}", disk.available_space());
            println!("  Total space: {}", disk.total_space());

            i += 1;

            if i != disks.len() {
                println!();
            }
        }
    }

    Ok(())
}
