use std::{path::PathBuf,env, process::exit,io};

mod cli;
mod cheatsheet;

pub fn show_paths(path: &PathBuf, topic: &str) -> io::Result<()> {
    let mut p = path.clone();
    p.push(topic); 
    p.set_extension("json");
    if p.exists() {
        println!("{}", p.display());
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

    let matches = cli::build_cli("ksearch").get_matches();
    let search = matches.get_one::<String>("search");
    let topic = matches.get_one::<String>("topic");
    let filter = matches.get_one::<String>("filter");
    let env = matches.get_flag("env");
    let show_path = matches.get_flag("path");

    if env {
        println!("KSEARCH={}", csheet_paths.as_str());
    } else {
        for path in csheet_paths.split(":") {
            let mut pathbuf = PathBuf::new();
            pathbuf.push(path);
            if let Some(topic) = topic {
                if show_path {
                    let _ = show_paths(&pathbuf, &topic);
                } else {
                    pathbuf.push(topic); 
                    pathbuf.set_extension("json");
                    let _ = cheatsheet::find_topic(&pathbuf, &topic, &search, &filter);
                }         
            } else {
                let _ = cheatsheet::find_files(&pathbuf, &search, &filter);
            }
        }
    }
}