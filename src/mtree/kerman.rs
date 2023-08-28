pub mod kman {
    use std::fs::{read_dir, read_to_string};
    use std::path::{Path, PathBuf};
    use std::{
        collections::{HashMap, HashSet},
        process::Command,
    };

    pub static MOD_D: &str = "/lib/modules";
    pub static MOD_DEP_F: &str = "modules.dep";
    pub static MOD_INFO_EXE: &str = "/usr/sbin/modinfo";

    /// Metadata about the kernel and details about it
    #[derive(Debug, Clone)]
    pub struct KernelInfo<'kinfo> {
        pub version: String,
        path: PathBuf,
        dep_path: PathBuf,
        deplist: HashMap<String, Vec<String>>,
        is_valid: bool,
        _loaded: bool,
        debug: &'kinfo bool,
    }

    impl<'kinfo> KernelInfo<'kinfo> {
        /// Creates an instance of a KernelInfo struct with the version
        /// of the kernel and paths to required points for module analysis
        pub fn new(kver: &str, debug: &'kinfo bool) -> Self {
            KernelInfo {
                version: kver.to_owned(),
                path: PathBuf::from(MOD_D),
                dep_path: PathBuf::from(""),
                deplist: HashMap::default(),
                _loaded: false,
                is_valid: false,
                debug,
            }
            .init()
        }

        /// Initialise the KernelInfo. This can be ran only once per an instance.
        fn init(mut self) -> Self {
            if self._loaded {
                return self;
            }

            self.path = self.path.join(&self.version);
            self.dep_path = self.dep_path.join(self.path.as_os_str()).join(MOD_DEP_F);
            self.load_deps();
            self._loaded = true;

            self
        }

        /// Load module dependencies
        /// Skip if there is no /lib/modules/<version/kernel directory
        fn load_deps(&mut self) {
            if self._loaded {
                return;
            }

            let modpath = PathBuf::from(MOD_D).join(&self.version).join("kernel");
            self.is_valid = Path::new(modpath.to_str().unwrap()).is_dir();
            if self.is_valid {
                for line in read_to_string(self.dep_path.as_os_str()).unwrap().lines() {
                    if let Some(sl) = line.split_once(':') {
                        let (modpath, moddeps) = (sl.0.trim(), sl.1.trim());
                        let mut deplist: Vec<String> = vec![];

                        if !moddeps.is_empty() {
                            deplist = moddeps.split(' ').into_iter().map(|x| x.to_owned()).collect();
                            if *self.debug {
                                log::debug!("Found {} dependencies for {}", deplist.len(), modpath);
                            }
                        }

                        self.deplist.insert(modpath.to_owned(), deplist);
                    }
                }
            }
        }

        /// Returns true if there are actual modules on the media for this kernel.
        /// There are often kernel paths left after a kernel was not completely purged.
        fn is_valid(&self) -> bool {
            self.is_valid
        }

        /// Get path of dependencies file
        #[allow(dead_code)]
        pub fn get_dep_path(&self) -> &str {
            self.dep_path.to_str().unwrap()
        }

        /// Find a full path to a module
        /// Example: "sunrpc.ko" will be resolved as "kernel/net/sunrpc/sunrpc.ko"
        ///
        /// Some modules are named differently on the disk than in the memory.
        /// In this case they are tried to be resolved via external "modinfo".
        fn expand_module_name<'a>(&'a self, name: &'a String) -> &String {
            let mut m_name: String;
            if !name.ends_with(".ko") {
                m_name = format!("{}.ko", name); // "sunrpc" -> "sunrpc.ko"
            } else {
                m_name = name.to_owned();
            }

            if !m_name.starts_with("kernel/") {
                // name or partial path
                if !m_name.contains('/') {
                    m_name = format!("/{}", m_name); // "sunrpc.ko" -> "/sunrpc.ko"
                }

                for fmodname in self.deplist.keys() {
                    // Eliminate to a minimum 3rd fallback via modinfo by trying replacing underscore with a minus.
                    // This not always works, because some modules called mixed.
                    let mm_name = m_name.replace('_', "-");
                    if fmodname.ends_with(&m_name) || fmodname.ends_with(&mm_name) {
                        return fmodname;
                    }
                }
            }

            let out = Command::new(MOD_INFO_EXE).arg(name).output();
            match out {
                Ok(_) => match String::from_utf8(out.unwrap().stdout) {
                    Ok(data) => {
                        for line in data.lines().map(|el| el.replace(" ", "")) {
                            if line.starts_with("filename:/") && line.contains("/kernel/") {
                                let t_modname = format!("kernel/{}", line.split("/kernel/").collect::<Vec<&str>>()[1]);
                                for fmodname in self.deplist.keys() {
                                    if *fmodname == t_modname {
                                        return fmodname;
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => log::error!("Unable to get info about module \"{name}\": {}", err),
                },
                Err(_) => log::error!("Module {name} not found on the disk"),
            }

            name
        }

        /// Resolve dependencies for one module
        /// This is an internal method
        fn get_mod_dep(&self, name: &String, mods: &mut HashSet<String>) {
            let mdeps = self.deplist.get(name).unwrap();
            for mdep in mdeps {
                mods.insert(mdep.to_owned());

                // If a dependency has its own dependencies
                let d_mdeps = self.deplist.get(mdep).unwrap();
                if !d_mdeps.is_empty() {
                    for d_dep in d_mdeps {
                        mods.insert(d_dep.to_owned());
                        self.get_mod_dep(d_dep, mods);
                    }
                }
            }
        }

        /// Resolve all module dependencies
        pub fn get_deps_for(&self, names: &[String]) -> HashMap<String, Vec<String>> {
            let mut mod_tree: HashMap<String, Vec<String>> = HashMap::new();
            for kmodname in names {
                let r_kmodname = self.expand_module_name(kmodname);
                if !r_kmodname.contains('/') {
                    log::warn!("Module not found on a disk: {}", r_kmodname);
                    continue;
                }

                let mut mod_deps: HashSet<String> = HashSet::default();
                let mut r_deps: Vec<String> = vec![];

                self.get_mod_dep(r_kmodname, &mut mod_deps);

                for v in mod_deps {
                    r_deps.push(v);
                }
                mod_tree.insert(r_kmodname.to_owned(), r_deps);
            }

            mod_tree
        }
    }

    /// Get the list of existing kernels in the system.
    pub fn get_kernel_infos(debug: &bool) -> Vec<KernelInfo> {
        let mut kernels: Vec<KernelInfo> = vec![];
        for fres in read_dir(MOD_D).unwrap() {
            let fd = fres.unwrap();
            if fd.file_type().unwrap().is_dir() {
                let kinfo: KernelInfo<'_> = KernelInfo::new(fd.path().file_name().unwrap().to_str().unwrap(), debug);
                if kinfo.is_valid() {
                    kernels.push(kinfo);
                }
            }
        }

        kernels
    }
}
