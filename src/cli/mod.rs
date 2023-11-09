use clap::{Arg, ArgAction, Command};

pub fn build_cli(program_name: &'static str) -> Command {
    Command::new(program_name)
        .about("CLI to search knowlege from JSON cheatsheets")
        .arg(
            Arg::new("search")
                .long("search")
                .short('s')
                .default_value("")
                .help("Term to search"),
        )
        .arg(
            Arg::new("topic")
                .long("topic")
                .short('t')
                .default_value("")
                .help("Targeted search topic"),
        )
        .arg(
            Arg::new("filter")
                .long("filter")
                .short('f')
                .default_value("")
                .help("Search filters such as tags"),
        )
        .arg(
            Arg::new("env")
                .long("environment")
                .short('e')
                .action(ArgAction::SetTrue)
                .help("Show environment variable"),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .action(ArgAction::SetTrue)
                .help("Show topic path if exist"),
        )
        .arg(
            Arg::new("generate")
                .long("generate")
                .short('G')
                .action(ArgAction::SetTrue)
                .help("Generate CUE notes as JSON file"),
        )
        .arg(
            Arg::new("inventory")
                .long("inventory")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("List all available topics"),
        )
}

#[test]
fn verify_app() {
    build_cli("ksearch_test").debug_assert();
}
