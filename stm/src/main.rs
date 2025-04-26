mod config;

use clap::{Command, arg, command, value_parser};
use std::path::PathBuf;
use stm_core::dpkg;

fn main() {
    better_panic::install();

    let matches = command!()
        .subcommand(
            Command::new("dpkg")
                .about("De-packages an .stm package file")
                .arg(arg!([FILE] "The .stm package file to be installed"))
                .arg(
                    arg!(-s --systemroot <PATH> "Sets a custom system root for the package to install")
                        .required(false)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(arg!(
                    --verbose "Enable verbose installing"
                )),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("dpkg", sub_matches)) => {
            let file = sub_matches
                .get_one::<String>("FILE")
                .expect("Required for 'dpkg'");
            let system_root = sub_matches.get_one::<PathBuf>("systemroot");
            let verbose = sub_matches.get_flag("verbose");

            if verbose {
                println!("file: {}", file);
                if let Some(sr) = system_root {
                    println!("system_root: {}", sr.to_string_lossy());
                }
                println!("verbose: {}", verbose);
            }

            let destination: PathBuf = match system_root {
                Some(sr) => sr.clone(),
                None => config::get_system_root_config(),
            };

            dpkg::unpack_package(file, &destination, verbose);
        }

        _ => {}
    }
}
