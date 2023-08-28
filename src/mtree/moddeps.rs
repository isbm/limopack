pub mod ktree {
    use crate::mdb::modules::modinfo;
    use crate::mtree::kerman::kman::KernelInfo;
    use std::collections::{BTreeSet, HashMap, HashSet};

    pub struct KModuleTree<'kinfo> {
        kernel: &'kinfo KernelInfo<'kinfo>,
    }

    impl<'kinfo> KModuleTree<'kinfo> {
        pub fn new(kinfo: &'kinfo KernelInfo<'kinfo>) -> Self {
            KModuleTree { kernel: kinfo }
        }

        /// lsmod
        pub fn get_loaded_modules(&self) -> Vec<String> {
            modinfo::lsmod().iter().map(|modinfo| modinfo.name.to_owned()).collect()
        }

        /// Snapshot currently active modules (lsmod)
        #[allow(dead_code)]
        pub fn get_loaded_deps(&self) -> HashMap<String, Vec<String>> {
            self.get_specified_deps(&self.get_loaded_modules())
        }

        /// Get all dependencies for the specified modules
        pub fn get_specified_deps(&self, modules: &[String]) -> HashMap<String, Vec<String>> {
            match modules.is_empty() {
                true => return self.kernel.get_deps_for(&self.get_loaded_modules()),
                false => (),
            }

            self.kernel.get_deps_for(modules)
        }

        /// Same as a snapshot `get_loaded()` except it is merges
        /// all the dependencies into one list for an actual operations.
        #[allow(dead_code)]
        pub fn merge_loaded_deps(&self) -> HashSet<String> {
            self.merge_specified_deps(&self.get_loaded_modules())
        }

        /// Same as `get_specified` method, except it merges
        /// all the dependencies into one list for an actual operations.
        pub fn merge_specified_deps(&self, modules: &[String]) -> HashSet<String> {
            let mut deps = HashSet::default();
            for (module, data) in self.get_specified_deps(modules) {
                deps.extend(data);
                deps.insert(module);
            }

            deps
        }
    }
}
