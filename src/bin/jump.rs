#![feature(use_extern_macros)]

extern crate clap;
extern crate jump;

use clap::{Arg, App, AppSettings};
use jump::Config;
use std::process;

macro_rules! exit_if_err {
    // TODO: print to stderr
    ($x:expr, $msg:expr) => (match $x {
        Ok(val) => val,
        Err(err) => {
            println!($msg, err);
            process::exit(1)
        }
    });
}

fn main() {
    let args = App::new("Jump Configure")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Allows you to easily jump around your filesystem")
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("marker")
             .help("The name of the marker")
             .index(1)
             .required(true))
        .get_matches();

    let config = exit_if_err!(Config::load(), "echo \"Failed to load config: {}\"");
    // We can unwrap this because it is marked as required
    let name = args.value_of("marker").unwrap();
    let marker_path = config.get_marker(name);
    match marker_path {
        None       => println!("echo \"Can't jump to {}\"", name),
        Some(path) => println!("cd \"{}\"", path),
    }

}
