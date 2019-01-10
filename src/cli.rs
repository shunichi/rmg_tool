use clap::{App, Arg, SubCommand};

pub enum Command {
    None,
    Status,
    Diff,
    Down { no_switch: bool },
}

pub struct Options {
    pub command: Command,
    pub branch: Option<String>,
}

pub fn parse_opts() -> Options {
    let matches = App::new("Rails migration diff")
        .version(crate_version!())
        .subcommand(SubCommand::with_name("status")
            .about("Show migration status"))
        .subcommand(SubCommand::with_name("diff")
            .about("Diff migration")
            .arg(Arg::with_name("BRANCH")
                .required(true)
                .help("Specify branch name switching to")))
        .subcommand(SubCommand::with_name("down")
            .about("Migrate down to branch and switch to branch")
            .arg(Arg::with_name("BRANCH")
                .required(true)
                .help("Specify branch name switching to"))
            .arg(Arg::with_name("no-switch")
                .short("n")
                .long("no-switch")
                .help("Only migrate down")))
        .get_matches();

    let options = Options {
      command: Command::None,
      branch: None,
    };
    match matches.subcommand() {
        ("status", _) => Options {
            command: Command::Status,
            ..options
        },
        ("diff", Some(sm)) => Options {
            command: Command::Diff,
            branch: sm.value_of("BRANCH").map(|s| s.to_string())
        },
        ("down", Some(sm)) => Options {
            command: Command::Down { no_switch: sm.is_present("no-switch")},
            branch: sm.value_of("BRANCH").map(|s| s.to_string())
        },
        _ => options,
    }
}
