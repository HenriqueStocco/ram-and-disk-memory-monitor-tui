extern crate sysinfo;
use sysinfo::{Disks, System};

fn convert_to_giga(value: &u64) -> f64 {
    let multiplier = 1_000.00;
    let value_to_f64 = *value as f64;
    let converted_value = ((value_to_f64 / multiplier) / multiplier) / multiplier;

    return converted_value;
}

fn print_section(section_name: &str, def: fn()) {
    println!("-------- {} --------\n", section_name);

    def();

    println!("");
}

fn print_sysinfo() {
    println!("OS: {}", System::name().unwrap());
    println!("Kernel version: {}", System::kernel_version().unwrap());
    println!("OS version: {}", System::os_version().unwrap());
    println!("Host: {}", System::host_name().unwrap());
}

fn print_meminfo() {
    let mut sys = System::new_all();

    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();

    let total_memory_hr = convert_to_giga(&sys.total_memory());
    let used_memory_hr = convert_to_giga(&sys.used_memory());

    println!("Total RAM memory: {:.2} Gb", total_memory_hr);
    println!("Used RAM memory: {:.2} Gb", used_memory_hr);

    for disk in disks.list() {
        println!(
            "Disk: {}\nTotal space: {:.2} Gb",
            disk.name().to_str().unwrap(),
            convert_to_giga(&disk.total_space())
        );
    }
}

fn main() {
    print_section("Memory Info", print_meminfo);
    print_section("System Info", print_sysinfo);
}
