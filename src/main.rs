mod actions;
mod clidef;
mod logger;
mod mdb;
mod mtree;

use clap::Error;
use std::env;

static VERSION: &str = "0.1";
static LOGGER: logger::STDOUTLogger = logger::STDOUTLogger;

/// Initialise logger etc
pub fn init(debug: &bool) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| {
        log::set_max_level(if *debug {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
    })
}

#[allow(clippy::needless_collect)]
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut cli = clidef::cli(VERSION);

    if args.len() == 1 {
        return {
            cli.print_help().unwrap();
            Ok(())
        }
    }

    let params = cli.to_owned().get_matches();
    let debug: bool = params.get_flag("debug");

    init(&debug).unwrap();

    let modlist = params.get_one::<String>("use");
    let modules: Vec<String> = if modlist.is_some() {
        params
            .get_many::<String>("use")
            .unwrap()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect()
    } else {
        vec![]
    };

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
