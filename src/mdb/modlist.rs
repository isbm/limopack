use crate::mtree::kerman::kman;
use crate::mtree::kerman::kman::KernelInfo;
use colored::Colorize;
use exitcode::{self};
use log::warn;
use std::io::{BufRead, Error, Write};
use std::path::PathBuf;
use std::{collections::HashMap, fs::File, io::ErrorKind, path::Path, process};
use std::{fmt::format, io};

/// Module tracker
/// Used modules are stored a plain-text file in /lib/modules/<version>/modules.active
/// and each module is tracked in a garbage-collector style: if there is a software
/// component that is using it, then a module has one reference. If there are more
/// software components, then the reference increases. Once software component is
/// uninstalled, then the reference should decrease by one. If a module has no more
/// references, then the module is subject to be removed from the media.
///
/// Standard modules are different, they are marked as such and they are never touched
/// after provisioning.
///
/// IMPORTANT: This file does not contain module dependencies. Adding and removal is
///            meant to only operate on modules that are main, and their dependencies
///            will just "follow". Removal of a module is as simple as excluding it
///            from this file, so the dependency resolver will remove what's left.
///
/// Format of the /lib/modules/<version>/modules.active file as follows:
///
///     <relative/module/path>:<marker>
///
/// Markers:
///
///     <int> - Number of references (software components) that require that module
///     S     - Static permanent module
///
/// Example:
///
///     kernel/drivers/net/tap.ko:S
///     kernel/drivers/acpi/acpi_pad.ko:1
///

static MOD_STOR: &str = "modules.active";

pub struct ModList<'a> {
    // Map to path to a module which referring to a number.
    // The number is referring to negative, zero and positive values:
    //   - negative value (-1) is "S" (static module)
    //   - zero value makes a module to be a subject for garbage collection
    //   - any positive value is a counter for the references
    modlist: HashMap<String, i16>,
    kinfo: &'a KernelInfo<'a>,
}

impl<'a> ModList<'a> {
    /// Constructor
    pub fn new(kinfo: &'a KernelInfo) -> Result<Self, std::io::Error> {
        let mut modlist = ModList { modlist: HashMap::default(), kinfo };

        let loaded = modlist.load();
        if loaded.is_err() {
            Err(loaded.err().unwrap())
        } else {
            Ok(modlist)
        }
    }

    // Get storage path
    fn get_storage_path(&self) -> PathBuf {
        Path::new(kman::MOD_D).join(&self.kinfo.version).join(MOD_STOR)
    }

    /// Read used modules from the storage
    fn load(&mut self) -> Result<(), std::io::Error> {
        let st_pth = self.get_storage_path();
        if !st_pth.exists() {
            log::warn!("No module storage index found. Skipping...");
            return Ok(());
        }

        let rfp: Result<File, std::io::Error> = File::open(st_pth);
        if rfp.is_err() {
            return Err(rfp.err().unwrap());
        }

        for mut data in io::BufReader::new(rfp.unwrap()).lines().flatten() {
            data = data.trim().to_string();
            if data.starts_with('#') || data.is_empty() || !data.contains(':') {
                continue;
            }
            let state_kw: Vec<String> = data.split(':').map(|x| x.to_string()).collect();
            if state_kw.len() != 2 {
                log::warn!("Suspicious entry found: {}. Skipping...", data);
                continue;
            }

            let state_ptr: i16 = if state_kw[1] == "S" { -1 } else { state_kw[1].to_string().parse::<i16>().unwrap() };

            self.modlist.insert(state_kw[0].to_owned(), state_ptr);
        }

        Ok(())
    }

    /// Write data of used modules to the storage
    fn write(&self) -> Result<(), std::io::Error> {
        let sp = self.get_storage_path();
        log::info!("Writing to {}", format!("{:?}", sp.as_path()).bright_yellow());
        let f_res = File::create(sp);
        if f_res.is_err() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("Error while saving data about used modules: {}", f_res.err().unwrap()),
            ));
        }

        let mut f_ptr = f_res.unwrap();

        for (modname, modstate) in &self.modlist {
            let mut s: String;
            let rw: Result<(), io::Error> = f_ptr.write_all(
                format!("{}:{}\n", modname, if modstate < &0 { "S".to_string() } else { modstate.to_string() }).as_bytes(),
            );

            if rw.is_err() {
                return Err(rw.err().unwrap());
            }
        }

        Ok(())
    }

    /// Add a main module (no dependencies to in). This increases the counter, but doesn't write anything to a disk.
    pub fn add(&mut self, name: String, is_static: bool) {
        let optval = self.modlist.get(&name);
        if optval.is_none() {
            // new entry
            log::info!("Adding {}module \"{}\"", if is_static { "static " } else { "" }, name.bright_yellow());
            self.modlist.insert(name, if is_static { -1 } else { 1 });
        } else {
            let refcount = optval.unwrap();
            if *refcount > 0 {
                log::info!("Updating {}module \"{}\"", if is_static { "static " } else { "" }, name.bright_yellow());
                self.modlist.insert(name, refcount + 1);
            } else {
                log::warn!("Skipping static module \"{}\"", name.bright_yellow());
            }
        }
    }

    /// Save current state to the disk
    pub fn save(&self) -> Result<(), std::io::Error> {
        self.write()
    }

    /// Remove a module from the tree.
    ///
    /// Note, it does not removes a module from the list iff there are no more counters
    /// left and the pointers are zero. This decreases the counter, but doesn't write
    /// anything to a disk.
    pub fn remove(&mut self, name: String) -> Result<(), std::io::Error> {
        let mut optval = self.modlist.get(&name);
        if optval.is_none() {
            return Err(std::io::Error::new(ErrorKind::NotFound, format!("Unable to remove {:?}: module not found", name)));
        }

        let state: &mut i16 = &mut optval.as_mut().unwrap().to_owned();
        if *state > 0 {
            *state -= 1;
        }

        #[allow(clippy::comparison_chain)]
        if *state == 0 {
            log::info!("Removing \"{}\"", name);
            match self.modlist.remove(&name) {
                Some(_) => {}
                None => {
                    return Err((std::io::Error::new(
                        ErrorKind::NotFound,
                        format!("Unable to remove {:?}: module not found", name),
                    )));
                }
            }
        } else if *state > 0 {
            log::info!("Keeping \"{}\" because of {} references", name, *state);
            self.modlist.insert(name, *state);
        } else {
            log::warn!("Skipping static module \"{}\"", name);
        }

        Ok(())
    }

    /// Apply changes on a disk: remove from the media unused modules
    pub fn commit(&self) -> Result<(), std::io::Error> {
        log::info!("Applying changes");
        let r = self.write();
        if r.is_err() {
            log::error!("Error while saving data about used modules: {}", r.err().unwrap());
            process::exit(exitcode::IOERR);
        }

        Ok(())
    }
}
