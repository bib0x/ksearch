use std::{path::{Path, PathBuf}, error::Error, env, process::exit, string, fs, io};
use clap::{Command, Arg};
use serde::Deserialize;

pub fn build_cli(program_name: &'static str) -> Command {
    Command::new(program_name)
        .about("CLI to search knowlege from JSON cheatsheets")
        .arg(
            Arg::new("search")
            .long("search")
            .short('s')
            .required(true)
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
}

// [ { 
//   description: "test",  
//   data: [ 
//     "ls -l",
//     "truc"
// ]
// },...]
#[derive(Deserialize, Debug)]
pub struct Cheatsheet {
    pub description: String,
    pub data: Vec<String>,
    pub tags: Vec<String>
}

pub fn find_topic(path: &PathBuf, topic: &str, search: &str, filter: &Option<&String>) -> io::Result<()> {
    let mut p = path.clone();
    p.push(topic); 
    p.set_extension("json");
    
    let json_file = fs::File::open(p).expect("file should open read only");
    let ch: Vec<Cheatsheet> = serde_json::from_reader(json_file).expect("file should be proper JSON");

    let matches : Vec<&Cheatsheet> = if let Some(f) = filter {
         ch.iter().filter(|e| { e.description.contains(search) && e.tags.contains(f) }).map(|c| c).collect()
    } else {
         ch.iter().filter(|e| { e.description.contains(search) }).map(|c| c).collect()
    };

    for m in matches.iter() {
        println!("{}", m.description);
        for d in m.data.iter() {
            println!("- {}", d);
        } 
        println!("");
    }

    Ok(())
}

pub fn find_files(path: &PathBuf, search: &str, filter: &Option<&String>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(extension) = p.extension() {
            if extension == "json" {
                let json_file = fs::File::open(p).expect("file should open read only");
                let ch: Vec<Cheatsheet> = serde_json::from_reader(json_file).expect("file should be proper JSON");

                let matches : Vec<&Cheatsheet> = if let Some(f) = filter {
                    ch.iter().filter(|e| { e.description.contains(search) && e.tags.contains(f) }).map(|c| c).collect()
                } else {
                    ch.iter().filter(|e| { e.description.contains(search) }).map(|c| c).collect()
                };

                for m in matches.iter() {
                    println!("{}", m.description);
                    for d in m.data.iter() {
                        println!("- {}", d);
                    } 
                println!("");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let csheet_paths = match env::var("KSEARCH_PATH") {
        Ok(envpath) => envpath,
        Err(_) => {
            eprintln!("Environment variable KSEARCH_PATH not defined.");
            exit(1);
        }
    };

    let matches = build_cli("ksearch").get_matches();
    let search = matches.get_one::<String>("search").unwrap();
    let topic = matches.get_one::<String>("topic");
    let filter = matches.get_one::<String>("filter");

    for path in csheet_paths.split(":") {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        if let Some(topic) = topic {
            let _ = find_topic(&pathbuf, &topic, &search, &filter);
        } else {
            let _ = find_files(&pathbuf, &search, &filter);
        }
    }


}

#[test]
fn verify_app() {
    build_cli("ksearch_test").debug_assert();
}
