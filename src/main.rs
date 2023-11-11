use std::{env, io, path::Path, path::PathBuf, process::exit};

mod cli;
mod cue;
mod knowledge;

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

    let env_flag = matches.get_flag("env");
    let path_flag = matches.get_flag("path");
    let generate_flag = matches.get_flag("generate");
    let inventory_flag = matches.get_flag("inventory");
    let match_color_flag = matches.get_flag("match_color");
    let list_cue_flag = matches.get_flag("list");

    let has_topic = topic.len() > 0;

    if env_flag {
        println!("KSEARCH_PATH={}", csheet_paths.as_str());
        match env::var("KSEARCH_COLORED") {
            Ok(_) => println!("KSEARCH_COLORED OK"),
            _ => println!("KSEARCH_COLORED KO"),
        }
    } else {
        for path in csheet_paths.split(":") {
            if generate_flag {
                let cuepath = Path::new(path).join("cue");
                let jsonpath = Path::new(path).join("json");
                if let Err(_) = cue::export_as_json(&cuepath, &jsonpath) {
                    eprintln!("Cue export to json failed for {}", cuepath.display());
                    exit(1);
                }
            } else if list_cue_flag {
                let cuepath = Path::new(path).join("cue");
                let _ = cue::list_fullpath(&cuepath);
            } else {
                let mut jsonpath = Path::new(path).join("json");
                if has_topic {
                    if path_flag {
                        let _ = show_paths(&jsonpath, &topic);
                    } else {
                        jsonpath.push(topic.clone());
                        jsonpath.set_extension("json");

                        let knowledges = knowledge::from_file(&jsonpath);
                        knowledge::show_topic(
                            &knowledges,
                            &topic,
                            &search,
                            &filter,
                            match_color_flag,
                        );
                    }
                } else {
                    if inventory_flag {
                        println!("{}", jsonpath.display());
                        let _ = knowledge::find_files(
                            &jsonpath,
                            "",
                            "",
                            inventory_flag,
                            match_color_flag,
                        );
                        println!("");
                    } else {
                        let _ = knowledge::find_files(
                            &jsonpath,
                            &search,
                            &filter,
                            inventory_flag,
                            match_color_flag,
                        );
                    }
                }
            }
        }
    }
}
