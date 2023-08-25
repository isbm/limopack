use std::io::ErrorKind;

use crate::mdb::modlist;
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
        let kmtree: KModuleTree<'_> = KModuleTree::new(&ki);
        for (m, d) in kmtree.get_specified_deps(modules) {
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
        let kmtree: KModuleTree<'_> = KModuleTree::new(&ki);
        for m in kmtree.merge_specified_deps(modules) {
            println!("{m}");
        }
    }
}

/// Add or remove kernel modules
fn _add_remove(debug: &bool, add: bool, is_static: bool, modules: &mut Vec<String>) -> Result<(), std::io::Error> {
    for ki in get_kernel_infos(debug) {
        let kmtree: KModuleTree<'_> = KModuleTree::new(&ki);
        let rml: Result<modlist::ModList<'_>, std::io::Error> = modlist::ModList::new(&ki);

        if rml.is_err() {
            return Err(rml.err().unwrap());
        }

        // Use lsmod?
        if modules.is_empty() {
            for s in kmtree.get_loaded_modules().iter().map(|x| x.to_owned()).collect::<Vec<String>>() {
                modules.push(s);
            }
        }

        modules.sort();

        log::info!("Scope of {} modules", modules.len());

        let mut ml: modlist::ModList<'_> = rml.unwrap();
        for modname in &mut *modules {
            if add {
                ml.add(modname.to_string(), is_static);
            } else {
                let res = ml.remove(modname.to_string());
                if res.is_err() {
                    return Err(std::io::Error::new(ErrorKind::InvalidInput, res.err().unwrap()));
                }
            };
        }

        let res = ml.save();
        if res.is_err() {
            return Err(res.err().unwrap());
        }
    }

    Ok(())
}

/// Add (register) kernel modules to be preserved
pub fn do_add(debug: &bool, is_static: bool, modules: &[String]) -> Result<(), std::io::Error> {
    _add_remove(debug, true, is_static, &mut modules.iter().map(|x| x.to_string()).collect())
}

/// Remove (unregister) kernel modules from being preserved
pub fn do_remove(debug: &bool, modules: &[String]) -> Result<(), std::io::Error> {
    _add_remove(debug, false, false, &mut modules.iter().map(|x| x.to_string()).collect())
}

/// Commit changes on the disk. This will permanently remove unused kernel modules
/// from the disk.
pub fn do_commit(debug: &bool) -> Result<(), std::io::Error> {
    for ki in get_kernel_infos(debug) {}
    Ok(())
}

/// Unregister specified package from the package manager database.
/// Yuck!...
pub fn do_unregister_pkg(debug: &bool, pkgname: String) -> Result<(), std::io::Error> {
    Ok(())
}
