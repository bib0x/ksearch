use serde::Deserialize;
use std::{path::PathBuf, fs, io};

#[derive(Deserialize, Debug)]
pub struct Cheatsheet {
    pub description: String,
    pub data: Vec<String>,
    pub tags: Vec<String>
}

pub fn from_file(path: &PathBuf) -> Vec<Cheatsheet> {
    let json_file = fs::File::open(path).expect("file should open read only");
    let ch: Vec<Cheatsheet> = serde_json::from_reader(json_file).expect("file should be proper json");
    ch
}

pub fn find_topic(
    path: &PathBuf, 
    topic: &str, 
    search: &Option<&String>, 
    filter: &Option<&String>) 
-> io::Result<()> {
    let ch = from_file(&path);

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
