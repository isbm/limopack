pub mod ktree {
    use crate::mtree::kerman::kman::KernelInfo;
    use std::collections::HashMap;

    pub struct KModuleTree<'kinfo> {
        kernel: KernelInfo<'kinfo>,
    }

    impl<'kinfo> KModuleTree<'kinfo> {
        pub fn new(kinfo: KernelInfo<'kinfo>) -> Self {
            KModuleTree { kernel: kinfo }
        }

        pub fn load(mut self) -> Self {
            self
        }

        pub fn get_tree(&self) {}

        /*
        pub fn get_deplist(&self) -> HashMap<String, Vec<String>> {
            return self.deplist.clone();
        }
        */
    }
}
