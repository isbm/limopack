use std::{
    fs,
    io::{Error, ErrorKind},
};

use colored::Colorize;

/// This module is designed to remove a package from /var/lib/dpkg/status from being
/// mentioned there. Such operation is required to install a package and physically
/// keep its data on a media, modify its content to further reuse and allow it to be
/// updated by a standard package manager means.
///
use super::rmpak::PackMod;

#[derive(Clone)]
pub struct DpkgMod {
    packages: Vec<String>,
}

impl DpkgMod {
    pub fn new() -> Self {
        DpkgMod { packages: vec![] }.load()
    }

    /// Remove field from a string.
    /// Fields are first keywords, following ":" colon.
    fn chop_field(&self, line: String) -> String {
        match line.split_once(":") {
            Some(data) => {
                return data.1.trim().to_string();
            }
            None => {
                return String::from("");
            }
        }
    }

    /// Load package status
    fn load(&mut self) -> Self {
        match fs::read_to_string("/var/lib/dpkg/status") {
            Ok(data) => {
                let _ = &self.packages.extend(data.split("\n\n").map(|el| el.to_string()).collect::<Vec<String>>());
            }
            Err(_) => (),
        }

        self.to_owned()
    }

    /// Returns true if a current chunk corresponds to a given package name
    fn is_package(&self, name: String, data: String) -> bool {
        let dls: Vec<String> = data.split("\n").map(|x| x.to_string()).collect();
        name == self.chop_field(dls[0].to_owned())
    }
}

impl PackMod for DpkgMod {
    fn remove_package(&self, pn: String) -> Result<(), Error> {
        log::info!("Looking for \"{}\" package in dpkg database", pn.bright_yellow());
        for p in &self.packages {
            if self.is_package(pn.to_owned(), p.to_owned()) {
                log::info!("Altering dpkg database for package \"{}\"", pn.bright_yellow());
                return Ok(());
            }
        }

        Err(Error::new(ErrorKind::NotFound, format!("Package \"{}\" was not found in the database", pn.bright_yellow())))
    }
}
