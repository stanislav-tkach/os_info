extern crate os_info;

fn main() {
    let info = os_info::get();

    // Print full information:
    println!("OS information: {}", info);

    // Print information separately:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
}
