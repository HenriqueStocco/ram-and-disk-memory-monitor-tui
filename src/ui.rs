//pub fn print_x() {}
// pub fn print_y() {}

pub fn print_section_layout(sec_name: &str, func: fn()) {
    println!("-------- {} --------\n", sec_name);

    func();

    println!("");
}
