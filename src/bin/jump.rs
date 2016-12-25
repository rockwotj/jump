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
             .index(1))
        .get_matches();

    let config = exit_if_err!(Config::load(), "Failed to load config: {}");
    let name = args.value_of("marker").unwrap_or("");
    let marker = config.get_marker(name)
                       .and_then(|path| path.to_str()
                                            .map(|s| s.to_string()));
    match marker {
        None       => {
            println!("Can't jump to '{}'", name);
            process::exit(1)
        },
        Some(path) => println!("{}", path),
    }

}
