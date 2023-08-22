mod mtree;

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

        for k in get_kernel_infos(&debug).iter() {
            println!(">>> {}", k.get_dep_path());
        }
    }
}
