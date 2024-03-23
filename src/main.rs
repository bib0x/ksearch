use std::{io, env, path::Path, path::PathBuf, process::exit};
use std::fs;

mod cli;
mod knowledge;

pub fn topic_exists(path: &PathBuf, topic: &str) -> bool {
    let mut p = Path::new(path).join(topic);
    p.set_extension("toml");
    p.exists()
}


pub fn list_fullpath(path: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if knowledge::is_toml_file(&p) {
            println!("{}", p.display());
        }
    }
    Ok(())
}

pub fn show_paths(path: &PathBuf, topic: &str) {
    if topic_exists(&path, &topic) {
        println!("{}/{}.toml", path.display(), topic);
    }
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

    let env_flag = matches.get_flag("env");
    let path_flag = matches.get_flag("path");
    let inventory_flag = matches.get_flag("inventory");
    let match_color_flag = matches.get_flag("match_color");
    let list_flag = matches.get_flag("list");

    let has_topic = topic.len() > 0;

    let mut knowledges_found = false;

    if env_flag {
        println!("KSEARCH_PATH={}", csheet_paths.as_str());
        match env::var("KSEARCH_COLORED") {
            Ok(_) => println!("KSEARCH_COLORED OK"),
            _ => println!("KSEARCH_COLORED KO"),
        }
    } else {
        for path in csheet_paths.split(":") {
            let mut tomlpath = PathBuf::from(path);
            if list_flag {
                let _ = list_fullpath(&tomlpath);
            } else {
                if has_topic {
                    if !topic_exists(&tomlpath, &topic) {
                        println!("Unknown topic named '{}'", topic);
                    } else {
                        if path_flag {
                            show_paths(&tomlpath, &topic);
                        } else {
                            tomlpath.push(topic.clone());
                            tomlpath.set_extension("toml");
                            let topic_content = knowledge::from_file(&tomlpath);
                            knowledges_found = knowledge::show_topic(
                                &topic_content.knowledges,
                                &topic,
                                &search,
                                &filter,
                                match_color_flag,
                            );
                        }
                    }
                } else {
                    if inventory_flag {
                        println!("");
                        println!("{}", tomlpath.display());
                        let _ = knowledge::find_files(
                            &tomlpath,
                            "",
                            "",
                            inventory_flag,
                            match_color_flag,
                        );
                        println!("");
                    } else {
                        let res = knowledge::find_files(
                            &tomlpath,
                            &search,
                            &filter,
                            inventory_flag,
                            match_color_flag,
                        );

                        if res.is_ok() {
                            knowledges_found = true;
                        }
                    }
                }
            }
        }
        if knowledges_found {
            println!("");
        }
    }
}
