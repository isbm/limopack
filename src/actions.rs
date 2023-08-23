use crate::mtree::kerman::kman::get_kernel_infos;
use crate::mtree::moddeps::ktree::KModuleTree;

/// Show module dependency tree.
///
/// Module examples:
///    sunrpc
///    9pnet_xen
///    bluetooth/hci_nokia.ko
///    ltc3815.ko
///    snd-soc-skl-ssp-clk
pub fn do_tree(debug: &bool, modules: &[String]) {
    for ki in get_kernel_infos(debug) {
        log::info!("Displaying module dependencies as a tree per a module");
        let kmtree = KModuleTree::new(ki);
        for (m, d) in kmtree.get_specified(modules) {
            println!("{m}");
            for dm in d {
                println!("  \\__{dm}");
            }
        }
    }
}

/// List dependencies from all specified modules
/// in a flat sorted format
pub fn do_list(debug: &bool, modules: &[String]) {
    for ki in get_kernel_infos(debug) {
        let kmtree = KModuleTree::new(ki);
        for m in kmtree.merge_specified(modules) {
            println!("{m}");
        }
    }
}
