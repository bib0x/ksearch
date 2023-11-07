use serde::Deserialize;
use std::{env, fs, io, path::PathBuf};

#[derive(Clone, Deserialize, Debug)]
pub struct Cheatsheet {
    pub description: String,
    pub data: Vec<String>,
    pub tags: Vec<String>,
}

impl Cheatsheet {
    pub fn display_nocolor(&self, topic: &str) {
        println!("");

        if self.tags.len() > 0 {
            println!("[{}][{}] {}", topic, self.tags.join(" "), self.description);
        } else {
            println!("[{}] {}", topic, self.description);
        }

        for d in self.data.iter() {
            println!("- {}", d);
        }
    }

    pub fn display_colorized(&self, topic: &str) {
        println!("");

        if self.tags.len() > 0 {
            println!(
                "\x1b[93;1m[{}]\x1b[0m\x1b[94;1m[{}]\x1b[0m",
                topic,
                self.tags.join(" ")
            );
        } else {
            println!("\x1b[93;1m[{}]\x1b[0m", topic);
        }

        println!("\x1b[92;1m#\x1b[0m {}", self.description);

        for d in self.data.iter() {
            println!("\x1b[92;1m>>>\x1b[0m \x1b[95m{}\x1b[0m", d);
        }
    }

    pub fn display(&self, topic: &str) {
        match env::var("KSEARCH_COLORED") {
            Ok(_) => self.display_colorized(&topic),
            _ => self.display_nocolor(&topic),
        }
    }
}

pub fn from_file(path: &PathBuf) -> Vec<Cheatsheet> {
    let json_file = fs::File::open(path).expect("file should open read only");
    let ch: Vec<Cheatsheet> =
        serde_json::from_reader(json_file).expect("file should be proper json");
    ch
}

fn parse_topic(
    path: &PathBuf,
    search: &Option<&String>,
    filter: &Option<&String>,
) -> io::Result<Vec<Cheatsheet>> {
    let ch = from_file(&path);

    let matches: Vec<Cheatsheet> = if let Some(f) = filter {
        if let Some(s) = search {
            ch.iter()
                .filter(|e| e.description.contains(*s) && e.tags.contains(f))
                .map(|c| c.clone())
                .collect()
        } else {
            ch.iter()
                .filter(|e| e.tags.contains(f))
                .map(|c| c.clone())
                .collect()
        }
    } else {
        if let Some(s) = search {
            ch.iter()
                .filter(|e| e.description.contains(*s))
                .map(|c| c.clone())
                .collect()
        } else {
            ch.iter().map(|c| c.clone()).collect()
        }
    };

    Ok(matches)
}

pub fn show_topic(path: &PathBuf, topic: &str, search: &Option<&String>, filter: &Option<&String>) {
    match parse_topic(&path, &search, &filter) {
        Ok(cheatsheets) => {
            for ch in cheatsheets.iter() {
                ch.display(&topic);
            }
            println!("");
        }
        _ => println!("No topic found."),
    }
}

pub fn find_files(
    path: &PathBuf,
    search: &Option<&String>,
    filter: &Option<&String>,
    inventory: bool,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(extension) = p.extension() {
            if extension == "json" {
                let mut pp = p.clone();
                pp.set_extension("");

                if let Some(topic_ostr) = pp.file_name() {
                    if let Some(topic) = topic_ostr.to_str() {
                        if inventory {
                            println!("{}", topic);
                        } else {
                            show_topic(&p, &topic, &search, &filter);
                        }
                    }
                    // XXX: Add log here
                }
            }
        }
    }
    Ok(())
}
