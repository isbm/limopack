mod actions;
mod clidef;
mod logger;
mod mdb;
mod mtree;
mod pakmod;
mod sysutils;

use clap::Error;
use std::{env, io::ErrorKind, process};

static VERSION: &str = "0.1";
static LOGGER: logger::STDOUTLogger = logger::STDOUTLogger;

/// Initialise logger etc
fn init(debug: &bool) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(if *debug { log::LevelFilter::Trace } else { log::LevelFilter::Info }))
}

fn if_err(res: Result<(), std::io::Error>) {
    if res.is_err() {
        log::error!("{}", res.err().unwrap().to_string());
        process::exit(exitcode::OSERR);
    }
}

#[allow(clippy::needless_collect)]
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut cli = clidef::cli(VERSION);

    if args.len() == 1 {
        return {
            cli.print_help().unwrap();
            Ok(())
        };
    }

    let params = cli.to_owned().get_matches();
    let debug: bool = params.get_flag("debug");

    init(&debug).unwrap();

    // Check if user has required access
    if params.get_flag("install") || params.get_flag("remove") || params.get_flag("apply") {
        if_err(sysutils::user_is_root());
    }

    let modlist = params.get_one::<String>("use");
    let modules: Vec<String> = if modlist.is_some() {
        params.get_many::<String>("use").unwrap().collect::<Vec<_>>().iter().map(|x| x.to_string()).collect()
    } else {
        vec![]
    };

    // If modules are not specified (or magic keyword?), then all are static,
    // because they are currently loaded and in use.
    let is_static = if modules.is_empty() { true } else { params.get_flag("static") };

    if params.get_flag("version") {
        println!("Version: {}", VERSION);
    } else if params.get_flag("tree") {
        actions::do_tree(&debug, &modules);
    } else if params.get_flag("list") {
        for modname in actions::do_list(&debug, &modules) {
            println!("{}", modname);
        }
    } else if params.get_flag("install") {
        if_err(actions::do_add(&debug, is_static, &modules));
    } else if params.get_flag("remove") {
        if_err(actions::do_remove(&debug, &modules));
    } else if params.get_flag("apply") {
        match params.get_one::<String>("pkname") {
            Some(pkname) => {
                if pkname.is_empty() {
                    if_err(Err(std::io::Error::new(ErrorKind::InvalidInput, "Package name is not specified")))
                }
                if_err(actions::do_unregister_pkg(&debug, pkname));
                if_err(actions::do_commit(&debug))
            }
            None => todo!(),
        }
    } else {
        cli.print_help().unwrap();
    }

    Ok(())
}
