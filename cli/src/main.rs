//! `os_info_cli`
//!
//! Provides command line interfaces for getting information about the current operating system,
//! such as type, version, edition and bitness.

#![deny(missing_docs, unsafe_code)]

use clap::Parser;
use log::warn;

#[derive(Parser)]
#[clap(about, version)]
struct Options {
    /// Show all OS information.
    #[clap(long)]
    all: bool,
    /// Show OS type.
    #[clap(short = 't', long = "type")]
    type_: bool,
    /// Show OS version.
    #[clap(short = 'v', long)]
    os_version: bool,
    /// Show OS bitness.
    #[clap(short, long)]
    bitness: bool,
}

fn main() {
    env_logger::init();

    let options = Options::parse();
    let info = os_info::get();

    if options.all || !(options.type_ || options.os_version || options.bitness) {
        if options.type_ || options.os_version || options.bitness {
            warn!("--all supersedes all other options");
        }

        println!(
            "OS information:\nType: {}\nVersion: {}\nBitness: {}\nFamily: {}",
            info.os_type(),
            info.version(),
            info.bitness(),
            info.family(),
        );
    } else {
        if options.type_ {
            println!("OS type: {}", info.os_type());
        }

        if options.os_version {
            println!("OS version: {}", info.version());
        }

        if options.bitness {
            println!("OS bitness: {}", info.bitness());
        }
    }
}
