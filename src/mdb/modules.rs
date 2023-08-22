 pub mod modinfo {
    /// ModInfo contains current live module information
    pub struct ModInfo {
        name: String,
        memsize: usize,
        instances: u8,
        dependencies: Vec<String>,
        memoffset: usize,  // Available for root only
    }

    /// lsmod is just parse /proc/modules
    pub fn lsmod() -> Vec<ModInfo>{
        let mut curr_mods: Vec<ModInfo> = vec![];
        //File::open();

        curr_mods
    }
 }