use clap::builder::styling;
use clap::{Arg, ArgAction, Command};

/// Define CLI arguments and styling
pub fn cli(version: &'static str) -> Command {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::White.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::White.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::BrightCyan.on_default())
        .placeholder(styling::AnsiColor::Cyan.on_default());

    Command::new("limopack")
        .version(version)
        .about("[Li]nux [Mo]dule [Pack]age Helper")
        // Config
        .arg(
            Arg::new("use")
                .short('u')
                .long("use")
                .help("Specify comma-separated list of kernel modules to be used.\n")
                .value_delimiter(','),
        )
        .arg(
            Arg::new("static")
                .short('s')
                .long("static")
                .action(ArgAction::SetTrue)
                .help("Use specified modules as static (i.e. stays permanently)"),
        )
        // Display
        .arg(
            Arg::new("tree")
                .short('e')
                .long("tree")
                .help("Display module dependency tree.")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("list").short('l').long("list").action(ArgAction::SetTrue).help(
            "Display in a sorted flat list format all modules that will
  be used. This includes all dependencies and already marked
  and existing modules.",
        ))
        .arg(Arg::new("pkname").short('p').long("pkname").value_delimiter(',').help(
            "Specify a package name, which needs to be un-registered
  from the package manager database in order to be visible to the system as
  non-existing, so the system can bring it again for an update or installation.\n",
        ))
        // Writable
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .action(ArgAction::SetTrue)
                .help("Mark specified modules as needed for the system."),
        )
        .arg(Arg::new("remove").short('r').long("remove").action(ArgAction::SetTrue).help(
            "Remove specified modules as no longer needed for the system,
  so they can be purged from the disk. This operation only marks
  the modules to be removed, but does not actually removes them.",
        ))
        .arg(Arg::new("apply").short('a').long("apply").action(ArgAction::SetTrue).help(
            "Apply the changes, vacuuming all unneded/unregisterd (non-marked)
  kernel modules, those are still exist on a disk, but
  always unused.\n",
        ))
        // Other
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Set debug mode for more verbose output."),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Get current version."),
        )
        .disable_version_flag(true)
        .disable_colored_help(false)
        .styles(styles)
        .after_help("NOTE: This program is designed to be mainly used with software packaging utilities and helpers.")
}
