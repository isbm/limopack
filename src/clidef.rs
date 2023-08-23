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
        .arg(
            Arg::new("use")
                .short('u')
                .long("use")
                .help("Specifycomma-separated list of kernel modules to be used.")
                .value_delimiter(',')
        )
        .arg(
            Arg::new("tree")
                .short('e')
                .long("tree")
                .help("Display module dependency tree.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("Display in a sorted flat list format all modules that will
  be used. This includes all dependencies and already marked and existing modules.")
        )
        .arg(
            Arg::new("install")
                .short('i')
                .long("install")
                .help("Mark specified modules as needed for the system.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .help("Remove specified modules as no longer needed for the system,
  so they can be purged from the disk. This operation only marks
  the modules to be removed, but does not actually removes them.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("apply")
                .short('a')
                .long("apply")
                .help("Apply the changes, vacuuming all unneded/unregisterd (non-marked)
  kernel modules, those are still exist on a disk, but always unused.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Set debug mode for more verbose output.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Get current version.")
                .action(ArgAction::SetTrue),
        )
        .disable_version_flag(true)
        .disable_colored_help(false)
        .styles(styles)
        .after_help("NOTE: This program is designed to be mainly used with software packaging utilities and helpers.")
}
