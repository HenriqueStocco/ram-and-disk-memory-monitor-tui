use crate::utils;
use sysinfo::{Disks, System};

pub fn print_sysinfo() {
    println!("OS: {}", System::name().unwrap());
    println!("Kernel version: {}", System::kernel_version().unwrap());
    println!("OS version: {}", System::os_version().unwrap());
    println!("Host: {}", System::host_name().unwrap());
}

pub fn print_meminfo() {
    let mut sys = System::new_all();

    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();

    let total_memory_hr = utils::convert_to_gb_f64(&sys.total_memory());
    let used_memory_hr = utils::convert_to_gb_f64(&sys.used_memory());

    println!("Total RAM memory: {:.2} Gb", total_memory_hr);
    println!("Used RAM memory: {:.2} Gb", used_memory_hr);

    for disk in disks.list() {
        println!(
            "Disk: {}\nTotal space: {:.2} Gb",
            disk.name().to_str().unwrap(),
            utils::convert_to_gb_f64(&disk.total_space())
        );
    }
}
