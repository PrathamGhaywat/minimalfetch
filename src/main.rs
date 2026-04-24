use sysinfo::{Disks, System};
use owo_colors::OwoColorize;

fn main() {
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());

    if sysinfo::IS_SUPPORTED_SYSTEM {
        let mut sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();

        let mut space: u64 = 0;
        let mut available_space: u64 = 0;

        for disk in disks.list() {
            space += disk.total_space();
            available_space += disk.available_space();
        }

        sys.refresh_all();

        // memory (bytes → GB)
        let used_mem = sys.used_memory() as f64 / 1_073_741_824.0;
        let total_mem = sys.total_memory() as f64 / 1_073_741_824.0;

        // disk (bytes → GB)
        let used_disk = (space - available_space) as f64 / 1_073_741_824.0;
        let total_disk = space as f64 / 1_073_741_824.0;
        let free_disk = available_space as f64 / 1_073_741_824.0;

        // cpu
        let cpu_usage = sys.global_cpu_usage();
        let cpu_name = sys
            .cpus()
            .first()
            .map(|c| c.brand())
            .unwrap_or("Unknown CPU");

        println!();
        println!("{}", "  minimalfetch".bold().cyan());
        println!();

        println!("  {} {}", "os   :".blue().bold(), os_name.white());

        println!(
            "  {} {:.2} / {:.2} GB",
            "mem  :".green().bold(),
            used_mem,
            total_mem
        );

        println!(
            "  {} {:.2} / {:.2} GB {} {:.2} {}",
            "disk :".yellow().bold(),
            used_disk,
            total_disk,
            "(free:".dimmed() ,
            free_disk.dimmed(),
            "GB)".dimmed(),
        );

        println!(
            "  {} {} ({:.1}%)",
            "cpu  :".red().bold(),
            cpu_name,
            cpu_usage
        );

        println!();
    } else {
        println!("{}", "This OS isn't supported".red());
    }
}