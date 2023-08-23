mod mdb;
mod mtree;
mod clidef;
mod actions;

use clap::Error;
use std::env;

static VERSION: &str = "0.1";

#[allow(clippy::needless_collect)]
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut cli: clap::App<'_> = clidef::cli(VERSION);

    if args.len() == 1 {
        return Ok(cli.print_help().unwrap());
    }

    let params = cli.to_owned().get_matches();
    let debug: bool = params.get_flag("debug");
    let modules: Vec<String>;

    let modlist = params.get_one::<String>("use");
    if !modlist.is_none() {
        modules = params
            .get_many::<String>("use")
            .unwrap()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect();
    } else {
        modules = vec![];
    }

    if params.get_flag("version") {
        println!("Version: {}", VERSION);
    } else if params.get_flag("tree") {
        actions::do_tree(&debug, &modules);
    } else if params.get_flag("list") {
        actions::do_list(&debug, &modules);
    } else {
        cli.print_help().unwrap();
    }

    Ok(())
}
