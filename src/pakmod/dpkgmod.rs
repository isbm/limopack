use std::{
    fs::{self, OpenOptions},
    io::{Error, ErrorKind, Write},
};

use colored::Colorize;

/// This module is designed to remove a package from /var/lib/dpkg/status from being
/// mentioned there. Such operation is required to install a package and physically
/// keep its data on a media, modify its content to further reuse and allow it to be
/// updated by a standard package manager means.
///
use super::rmpak::PackMod;

#[derive(Clone)]
pub struct DpkgMod<'a> {
    packages: Vec<String>,
    status_path: String,
    debug: &'a bool,
}

impl<'a> DpkgMod<'a> {
    pub fn new(debug: &'a bool) -> Self {
        DpkgMod { packages: vec![], status_path: "/var/lib/dpkg/status".to_string(), debug }.load()
    }

    /// Remove field from a string.
    /// Fields are first keywords, following ":" colon.
    fn chop_field(&self, line: String) -> String {
        match line.split_once(':') {
            Some(data) => data.1.trim().to_string(),
            None => String::from(""),
        }
    }

    /// Load package status
    fn load(&mut self) -> Self {
        if let Ok(data) = fs::read_to_string(&self.status_path) {
            let _ = &self.packages.extend(data.split("\n\n").map(|el| el.to_string()).collect::<Vec<String>>());
        }

        self.to_owned()
    }

    /// Returns true if a current chunk corresponds to a given package name
    fn is_package(&self, name: String, data: String) -> bool {
        let dls: Vec<String> = data.split('\n').map(|x| x.to_string()).collect();
        name == self.chop_field(dls[0].to_owned())
    }
}

impl PackMod for DpkgMod<'_> {
    /// Remove package from the index. This still keeps only the state of the modpack,
    /// but does not writes anything to the disk.
    fn remove_package(&mut self, pn: String) -> Result<(), Error> {
        let mut buff: Vec<String> = vec![];
        log::info!("Looking for \"{}\" package...", pn.bright_yellow());
        let mut found = false;
        for p in &self.packages {
            if !self.is_package(pn.to_owned(), p.to_owned()) {
                buff.push(p.to_owned());
            } else {
                log::info!("Altering package manager database for \"{}\"", pn.bright_yellow());
                found = true;
            }
        }

        self.packages = Vec::new();
        self.packages.extend(buff);

        if !found {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Package \"{}\" was not found in the database", pn.bright_yellow()),
            ));
        }

        Ok(())
    }

    /// Save the current state to the disk.
    fn save(&self) -> Result<(), Error> {
        log::info!("Save changes to the dpkg database");
        if *self.debug {
            log::debug!("Backing up \"{}\" before modification", self.status_path.to_owned().bright_yellow());
        }

        let status_backup_path = format!("{}.limopack.bkp", &self.status_path);
        fs::copy(&self.status_path, &status_backup_path)?;

        if let Ok(mut fptr) = OpenOptions::new().create(true).write(true).truncate(true).open(&self.status_path) {
            let p_idx = self.packages.len() - 1;
            for (idx, pinfo) in self.packages.iter().enumerate() {
                match fptr.write_all(format!("{}{}", pinfo, if idx < p_idx { "\n\n" } else { "" }).as_bytes()) {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!(
                            "Unable to write \"{}\": \"{}\"",
                            &self.status_path.bright_yellow(),
                            err.to_string().bright_red()
                        );
                        // Badaboom. We probably screwed it all up and now need to restore the backup. Hopefully.
                        if *self.debug {
                            log::debug!("Restoring \"{}\"", self.status_path.to_owned().bright_yellow());
                        }
                        fs::copy(&status_backup_path, &self.status_path)?;

                        if *self.debug {
                            log::debug!("Removing backup at \"{}\"", &status_backup_path.to_owned().bright_yellow());
                        }

                        fs::remove_file(&status_backup_path)?;
                    }
                }
            }
        }

        Ok(())
    }
}
