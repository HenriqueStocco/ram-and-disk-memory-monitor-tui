use sysinfo::{Disks, System};

fn convert_to_giga(value: &u64) -> f64 {
    let multiplier = 1_000.00;
    let value_to_f64 = *value as f64;
    let converted_value = ((value_to_f64 / multiplier) / multiplier) / multiplier;

    return converted_value;
}

fn main() {
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();

    sys.refresh_all();

    let total_memory_hr = convert_to_giga(&sys.total_memory());
    let used_memory_hr = convert_to_giga(&sys.used_memory());

    println!("Total RAM memory: {:.2} Gb", total_memory_hr);
    println!("Used RAM memory: {:.2} Gb", used_memory_hr);
    for disk in disks.list() {
        println!(
            "Disk: {:?}, Total space: {:.2?} Gb",
            disk.name(),
            convert_to_giga(&disk.total_space())
        );
    }
}
