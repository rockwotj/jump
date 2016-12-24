#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::path::PathBuf;
use std::io;
use std::error::Error;
use std::io::{ErrorKind, Result};
use std::fs::File;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    workspace: Option<PathBuf>,
    markers: HashMap<String, PathBuf>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let path = try!(Config::get_config_file());
        if path.exists() {
            let file = try!(File::open(path));         
            serde_json::from_reader(file)
                        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err.description()))
        } else {
            Ok(Config {
                workspace: None,
                markers: HashMap::new()
            })
        }
    }

    pub fn set_workspace(&mut self) -> Result<()> {
        env::current_dir().map(|cwd| {
            self.workspace = Some(cwd);
        })
    }

    pub fn get_marker(self, name: &str) -> Option<PathBuf> {
        let marker = self.markers.get(&name.to_string()).map(|pb| pb.clone());
        let workspace = self.workspace.map(|ws| ws.join(name));
        marker.or(workspace)
    }

    pub fn set_marker(&mut self, name: String) -> Result<()> {
        env::current_dir().map(|cwd| {
            self.markers.insert(name, cwd);
        })
    }

    pub fn unset_marker(&mut self, name: String) -> Result<()> {
       match self.markers.remove(&name) {
            Some(_) => Ok(()),
            None => Err(io::Error::new(ErrorKind::NotFound, "No marker exists for this name")),
       }
    }

    pub fn save(&self) -> Result<()> {
        let path = try!(Config::get_config_file());
        let mut file = try!(File::create(path));         
        serde_json::to_writer_pretty(&mut file, self)
            .map_err(|err| io::Error::new(ErrorKind::InvalidData, err.description()))
    }

    fn get_config_file() -> Result<PathBuf> {
        env::home_dir()
            .map(|path|  path.join(".jumprc"))
            .ok_or(io::Error::new(ErrorKind::NotFound, "Cannot locate home directory"))
    }
}
