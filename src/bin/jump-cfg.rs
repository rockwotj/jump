#![feature(use_extern_macros)]

extern crate clap;
extern crate jump;

use clap::{Arg, App, AppSettings, SubCommand};
use jump::Config;
use std::process;

macro_rules! exit_if_err {
    ($x:expr, $msg:expr) => (match $x {
        Ok(val) => val,
        Err(err) => {
            println!($msg, err);
            process::exit(1)
        }
    });
}

fn main() {
    let app = App::new("Jump Configure")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Allows you to easily jump around your filesystem")
        .settings(&[AppSettings::SubcommandRequiredElseHelp,
                    AppSettings::GlobalVersion,
                    AppSettings::UnifiedHelpMessage])
        .subcommand(SubCommand::with_name("workspace")
                    .about("Sets the workspace to look for projects in to $CWD"))
        .subcommand(SubCommand::with_name("display")
                    .about("Prints your current configuration"))
        .subcommand(SubCommand::with_name("set")
                    .about("Sets a jump marker for the current location")
                    .arg(Arg::with_name("name")
                         .help("The name of the marker")
                         .index(1)
                         .required(true)))
        .subcommand(SubCommand::with_name("unset")
                    .about("Removes a named jump marker")
                    .arg(Arg::with_name("name")
                         .help("The name of the marker")
                         .index(1)
                         .required(true)))
        .get_matches();

    let mut config = exit_if_err!(Config::load(), "Failed to load config: {}");
    match app.subcommand() {
        ("display", _)   => {
            // TODO(rockwood): Properly pretty print the config
            println!("{:?}", config);
        },
        ("workspace", _) => {
            exit_if_err!(config.set_workspace(), "Failed to set workspace: {}");
        },
        ("set", Some(cmd)) => {
            // We can unwrap this because it is marked as required
            let name = cmd.value_of("name").unwrap().to_string();
            exit_if_err!(config.set_marker(name), "Failed to set marker: {}");
        },
        ("unset", Some(cmd)) => {
            // We can unwrap this because it is marked as required
            let name = cmd.value_of("name").unwrap().to_string();
            exit_if_err!(config.unset_marker(name), "Failed to unset marker: {}");
        },
        _                => unreachable!(),
    }
    exit_if_err!(config.save(), "Failed to save workspace: {}");
}
