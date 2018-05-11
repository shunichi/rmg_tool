use clap::{App, Arg, SubCommand};

pub enum Command {
    None,
    Status,
    Diff,
    Down,
}

pub struct Options {
    pub command: Command,
    pub branch: Option<String>,
}

pub fn parse_opts() -> Options {
    let matches = App::new("rails migration diff")
        .version(crate_version!())
        .subcommand(SubCommand::with_name("status")
            .about("show migration status"))
        .subcommand(SubCommand::with_name("diff")
            .about("diff migration")
            .arg(Arg::with_name("BRANCH")
                .required(true)
                .help("specify branch name switching to")))
        .subcommand(SubCommand::with_name("down")
            .about("migrate down to branch")
            .arg(Arg::with_name("BRANCH")
                .required(true)
                .help("specify branch name switching to")))
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
            command: Command::Down,
            branch: sm.value_of("BRANCH").map(|s| s.to_string())
        }, 
        _ => options,
    }
}
