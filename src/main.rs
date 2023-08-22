mod mtree;
mod mdb;

use clap::{Arg, ArgAction, Command};
use mtree::kerman::kman::get_kernel_infos;
use std::env;

fn cli() -> Command<'static> {
    Command::new("linmodpak - Linux Module Package helper")
        .arg(
            Arg::new("tree")
                .short('e')
                .long("tree")
                .help("Display module dependency tree")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Set to debug mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Get current version")
                .action(ArgAction::SetTrue),
        )
}

#[allow(clippy::needless_collect)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let _ = cli().print_help();
        return;
    }

    let params = cli().get_matches();
    let debug: bool = params.get_flag("debug");

    if params.get_flag("tree") {
        if debug {
            println!("Getting kernel modules");
        }

        let modules = vec![
            "sunrpc".to_owned(),
            "9pnet_xen".to_owned(),
            "bluetooth/hci_nokia.ko".to_owned(),
            "ltc3815.ko".to_owned(),
            "snd-soc-skl-ssp-clk".to_owned(),
        ];

        for k in get_kernel_infos(&debug).iter() {
            println!("Examining {}", k.get_dep_path());
            let t = k.get_deps_for(&modules);
            for (m, d) in t {
                println!("{m}");
                for dm in d {
                    println!("  \\__{dm}");
                }
            }
        }
    }
}
