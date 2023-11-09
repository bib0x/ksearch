use std::{env, io, path::Path, path::PathBuf, process::exit};

mod cheatsheet;
mod cli;
mod cue;

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

    // Safe to unwrap cause we set default value from .Arg() definitions
    let search: String = matches.get_one::<String>("search").unwrap().to_string();
    let topic: String = matches.get_one::<String>("topic").unwrap().to_string();
    let filter: String = matches.get_one::<String>("filter").unwrap().to_string();

    let env = matches.get_flag("env");
    let show_path = matches.get_flag("path");
    let generate_flag = matches.get_flag("generate");
    let inventory = matches.get_flag("inventory");

    let has_topic = topic.len() > 0;

    if env {
        println!("KSEARCH={}", csheet_paths.as_str());
    } else {
        for path in csheet_paths.split(":") {
            if generate_flag {
                let cuepath = Path::new(path).join("cue");
                let jsonpath = Path::new(path).join("json");
                if let Err(_) = cue::export_as_json(&cuepath, &jsonpath) {
                    eprintln!("Cue export to json failed for {}", cuepath.display());
                    exit(1);
                }
            } else {
                let mut jsonpath = Path::new(path).join("json");
                if has_topic {
                    if show_path {
                        let _ = show_paths(&jsonpath, &topic);
                    } else {
                        jsonpath.push(topic.clone());
                        jsonpath.set_extension("json");

                        let cheatsheets = cheatsheet::from_file(&jsonpath);
                        cheatsheet::show_topic(&cheatsheets, &topic, &search, &filter);
                    }
                } else {
                    if inventory {
                        println!("{}", jsonpath.display());
                        let _ = cheatsheet::find_files(&jsonpath, "", "", inventory);
                        println!("");
                    } else {
                        let _ = cheatsheet::find_files(&jsonpath, &search, &filter, inventory);
                    }
                }
            }
        }
    }
}
