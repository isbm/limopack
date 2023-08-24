use crate::mtree::{self, kerman::kman};
use clap::Error;
use exitcode;
use std::{collections::HashMap, fs::File, io::ErrorKind, path::Path, process};

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

pub struct ModList {
    // Map to path to a module which referring to a number.
    // The number is referring to negative, zero and positive values:
    //   - negative value (-1) is "S" (static module)
    //   - zero value makes a module to be a subject for garbage collection
    //   - any positive value is a counter for the references
    modlist: HashMap<String, i16>,
    kversion: String,
}

impl ModList {
    /// Constructor
    pub fn new(kversion: String) -> Self {
        ModList {
            modlist: HashMap::default(),
            kversion,
        }
    }

    // Get storage path
    fn get_storage_path(&self) -> String {
        Path::new(kman::MOD_D)
            .join(&self.kversion)
            .join(MOD_STOR)
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Read used modules from the storage
    fn load(&self) -> Result<(), std::io::Error> {
        let rfp: Result<File, std::io::Error> = File::open(self.get_storage_path());
        if rfp.is_err() {
            return Err(rfp.err().unwrap());
        }
        let x = rfp.unwrap();

        Ok(())
    }

    /// Write data of used modules to the storage
    fn write(&self) -> Result<(), std::io::Error> {
        let fr = File::create(self.get_storage_path());
        if fr.is_err() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("Error while saving data about used modules: {}", fr.err().unwrap()),
            ));
        }
        Ok(())
    }

    /// Add a module. This increases the counter, but doesn't write anything to a disk.
    pub fn add(&self, name: String, is_static: bool) {}

    /// Remove a module. This decreases the counter, but doesn't write anything to a disk.
    pub fn remove(&self, name: String) {}

    /// Apply changes on a disk: remove from the media unused modules
    pub fn commit(&self) {
        let r = self.write();
        if r.is_err() {
            log::error!("Error while saving data about used modules: {}", r.err().unwrap());
            process::exit(exitcode::IOERR);
        }
    }
}
