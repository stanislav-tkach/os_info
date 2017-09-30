extern crate os_info;

fn main() {
    let info = os_info::get();
    println!("OS information: {}", info);
}
