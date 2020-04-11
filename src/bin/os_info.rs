use log::warn;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// Show all information.
    #[structopt(long)]
    all: bool,
    /// Show OS type.
    #[structopt(short = "t", long = "type")]
    type_: bool,
    /// Show OS version.
    #[structopt(short, long)]
    version: bool,
    /// Show OS bitness.
    #[structopt(short, long)]
    bitness: bool,
}

fn main() {
    env_logger::init();

    let options = Options::from_args();
    let info = os_info::get();

    if options.all || !(options.type_ || options.version || options.bitness) {
        if options.type_ || options.version || options.bitness {
            warn!("--all supersedes all other options");
        }

        println!(
            "OS information:\nType: {}\nVersion: {}\nBitness: {}",
            info.os_type(),
            info.version(),
            info.bitness()
        );
    } else {
        if options.type_ {
            println!("OS type: {}", info.os_type());
        }

        if options.version {
            println!("OS version: {}", info.version());
        }

        if options.bitness {
            println!("OS bitness: {}", info.bitness());
        }
    }
}
