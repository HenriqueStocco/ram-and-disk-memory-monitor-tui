extern crate sysinfo;

mod sys;
mod ui;
mod utils;

fn main() {
    ui::print_section_layout("Memory Info", sys::print_meminfo);
    ui::print_section_layout("System Info", sys::print_sysinfo);
}
