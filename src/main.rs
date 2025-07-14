use std::{io, env, path::PathBuf, process::exit};

mod cli;
mod knowledge;

fn main() -> io::Result<()> {
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

    let mut knowledges_found = false;

    // Standalone option
    // -e
    // List KSEARCH environment variables
    if env_flag {
        println!("KSEARCH_PATH\t{}", csheet_paths.as_str());
        match env::var("KSEARCH_COLORED") {
            Ok(_) => println!("KSEARCH_COLORED\tenabled"),
            _ => println!("KSEARCH_COLORED\tdisabled"),
        }
        return Ok(());
    }

    // Options needing to loop over KSEARCH_PATH
    let directories : Vec<_> = csheet_paths.split(":").collect();

    // -l
    // List fullpath of Toml files used to store knowledges
    if list_flag {
        for directory in directories {
            knowledge::list_fullpath(&directory)?
        }
        return Ok(());
    }

    // -i
    // List all topics name according to there location
    if inventory_flag {
        for directory in directories {
            println!("");
            println!("{}", directory);
            let _ = knowledge::find_files(
                &directory,
                "",
                "",
                inventory_flag,
                match_color_flag,
            );
            println!("");
        }
        return Ok(());
    }

    // -p & -t
    // List fullpath of a Toml file if it exists
    if path_flag && topic.len() > 0 {
        for directory in directories {
            if knowledge::topic_exists(&directory, &topic) {
                println!("{}/{}.toml", directory, topic);
            } else {
                println!("Unknown topic named '{}'", topic);
            }
        }
        return Ok(())
    }

    // -t
    // Search in a dedicated Toml file
    if topic.len() > 0 {
        for directory in directories {
            if knowledge::topic_exists(&directory, &topic) {
                let mut tomlpath = PathBuf::from(directory);
                tomlpath.push(topic.clone());
                tomlpath.set_extension("toml");

                let content = knowledge::from_file(&tomlpath);
                knowledges_found = knowledge::show_topic(&content.knowledges, &topic, &search, &filter, match_color_flag);
            }
        }
        if knowledges_found {
            println!("");
        }
        return Ok(());
    }

    // If no topic have been targeted, search in all files
    for directory in directories {
        let res = knowledge::find_files(&directory, &search, &filter, inventory_flag, match_color_flag);
        knowledges_found = res.is_ok_and(|r| r == true);
    }

    if knowledges_found {
        println!("");
    }

    Ok(())
}
