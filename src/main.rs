use std::{path::PathBuf, error::Error, env, process::exit, fs, io};
use serde::Deserialize;

mod cli;

#[derive(Deserialize, Debug)]
pub struct Cheatsheet {
    pub description: String,
    pub data: Vec<String>,
    pub tags: Vec<String>
}

// XXX : Return Result to manage error
pub fn get_json_data(path: &PathBuf) -> Vec<Cheatsheet> {
    let json_file = fs::File::open(path).expect("file should open read only");
    let ch: Vec<Cheatsheet> = serde_json::from_reader(json_file).expect("file should be proper JSON");
    ch
}

pub fn find_topic(
    path: &PathBuf, 
    topic: &str, 
    search: &Option<&String>, 
    filter: &Option<&String>) 
-> io::Result<()> {
    let ch = get_json_data(&path);

    let matches : Vec<&Cheatsheet> = if let Some(f) = filter {
         if let Some(s) = search {
            ch.iter().filter(|e| { e.description.contains(*s) && e.tags.contains(f) }).map(|c| c).collect()
         } else {
            ch.iter().filter(|e| { e.tags.contains(f) }).map(|c| c).collect()
         }
    } else {
         if let Some(s) = search {
            ch.iter().filter(|e| { e.description.contains(*s) }).map(|c| c).collect()
         } else {
            ch.iter().collect()
         }
    };

    for m in matches.iter() {
        println!("[{}] {}", topic, m.description);
        for d in m.data.iter() {
            println!("- {}", d);
        } 
        println!("");
    }

    Ok(())
}

pub fn find_files(path: &PathBuf, search: &Option<&String>, filter: &Option<&String>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(extension) = p.extension() {
            if extension == "json" {
                let mut pp = p.clone();
                pp.set_extension("");

                if let Some(topic_ostr) = pp.file_name() {
                    if let Some(topic) = topic_ostr.to_str() {
                        let _ = find_topic(&p, &topic, &search, &filter);
                    }
                    // XXX: Add log here
                }
            }
        }
    }
    Ok(())
}

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
                    let _ = find_topic(&pathbuf, &topic, &search, &filter);
                }         
            } else {
                let _ = find_files(&pathbuf, &search, &filter);
            }
        }
    }
}

#[test]
fn verify_app() {
    cli::build_cli("ksearch_test").debug_assert();
}
