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

/// Add (register) kernel modules to be preserved
pub fn do_add(debug: &bool, modules: &[String]) -> Result<(), std::io::Error> {
    Ok(())
}

/// Remove (unregister) kernel modules from being preserved
pub fn do_remove(debug: &bool, modules: &[String]) -> Result<(), std::io::Error> {
    Ok(())
}

/// Commit changes on the disk. This will permanently remove unused kernel modules
/// from the disk.
pub fn do_commit(debug: &bool, modules: &[String]) -> Result<(), std::io::Error> {
    Ok(())
}

/// Unregister specified package from the package manager database.
/// Yuck!...
pub fn do_unregister_pkg(debug: &bool, pkgname: String) -> Result<(), std::io::Error> {
    Ok(())
}
