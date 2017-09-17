extern crate os_info;

fn main() {
    let info = os_info::current_platform();
    println!("OS information: {:?}", info);
}
