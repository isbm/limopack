pub mod ktree {
    use crate::mdb::modules::modinfo;
    use crate::mtree::kerman::kman::KernelInfo;
    use std::collections::HashMap;

    pub struct KModuleTree<'kinfo> {
        kernel: KernelInfo<'kinfo>,
    }

    impl<'kinfo> KModuleTree<'kinfo> {
        pub fn new(kinfo: KernelInfo<'kinfo>) -> Self {
            KModuleTree { kernel: kinfo }
        }

        /// lsmod
        fn get_loaded_modules(&self) -> Vec<String> {
            modinfo::lsmod()
                .iter()
                .map(|modinfo| modinfo.name.to_owned())
                .collect()
        }

        /// Snapshot currently active modules (lsmod)
        pub fn get_loaded(&self) -> HashMap<String, Vec<String>> {
            self.get_specified(&self.get_loaded_modules())
        }

        /// Get all dependencies for the specified modules
        pub fn get_specified(&self, modules: &[String]) -> HashMap<String, Vec<String>> {
            if modules.len() == 0 {
                return self.kernel.get_deps_for(&self.get_loaded_modules());
            }

            self.kernel.get_deps_for(modules)
        }

        /// Same as a snapshot `get_loaded()` except it is merges
        /// all the dependencies into one list for an actual operations.
        pub fn merge_loaded(&self) -> Vec<String> {
            self.merge_specified(&self.get_loaded_modules())
        }

        /// Same as `get_specified` method, except it merges
        /// all the dependencies into one list for an actual operations.
        pub fn merge_specified(&self, modules: &[String]) -> Vec<String> {
            let mut deps: Vec<String> = vec![];
            for (module, data) in self.get_specified(modules) {
                deps.extend(data);
                deps.push(module);
            }

            deps.sort();
            deps
        }
    }
}
