pub fn convert_to_gb_f64(value: &u64) -> f64 {
    let multiplier = 1_000.00;
    let value_to_f64 = *value as f64;
    let converted_value = ((value_to_f64 / multiplier) / multiplier) / multiplier;

    return converted_value;
}

pub fn print_section_layout(sec_name: &str, func: fn()) {
    println!("-------- {} --------\n", sec_name);

    func();

    println!("");
}
