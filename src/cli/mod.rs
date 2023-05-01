use clap::{Command, Arg, ArgAction};

pub fn build_cli(program_name: &'static str) -> Command {
    Command::new(program_name)
        .about("CLI to search knowlege from JSON cheatsheets")
        .arg(
            Arg::new("search")
            .long("search")
            .short('s')
            .help("Term to search")
        )
        .arg(
            Arg::new("topic")
            .long("topic")
            .short('t')
            .help("Targeted search topic")
        )
        .arg(
            Arg::new("filter")
            .long("filter")
            .short('f')
            .help("Search filters such as tags")
        )
        .arg(
            Arg::new("env")
            .long("environment")
            .short('e')
            .action(ArgAction::SetTrue)
            .help("Show environment variable")
        )
        .arg(
            Arg::new("path")
            .long("path")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("Show topic path if exist")
        )
}
