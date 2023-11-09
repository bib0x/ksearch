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
    cheatsheets: &Vec<Cheatsheet>,
    search: &str,
    filter: &str,
) -> io::Result<Vec<Cheatsheet>> {
    //let ch = from_file(&path);

    let matches: Vec<Cheatsheet> = if filter.len() > 0 {
        let f = String::from(filter.clone());
        if search.len() > 0 {
            cheatsheets
                .iter()
                .filter(|e| e.description.contains(search) && e.tags.contains(&f))
                .map(|c| c.clone())
                .collect()
        } else {
            cheatsheets
                .iter()
                .filter(|e| e.tags.contains(&f))
                .map(|c| c.clone())
                .collect()
        }
    } else {
        if search.len() > 0 {
            cheatsheets
                .iter()
                .filter(|e| e.description.contains(search))
                .map(|c| c.clone())
                .collect()
        } else {
            cheatsheets.iter().map(|c| c.clone()).collect()
        }
    };

    Ok(matches)
}

pub fn show_topic(cheatsheets: &Vec<Cheatsheet>, topic: &str, search: &str, filter: &str) {
    match parse_topic(&cheatsheets, &search, &filter) {
        Ok(cheatsheets) => {
            for ch in cheatsheets.iter() {
                ch.display(&topic);
            }
            println!("");
        }
        _ => println!("No topic found."),
    }
}

pub fn find_files(path: &PathBuf, search: &str, filter: &str, inventory: bool) -> io::Result<()> {
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
                            let cheatsheets = from_file(&p);
                            show_topic(&cheatsheets, &topic, &search, &filter);
                        }
                    }
                    // XXX: Add log here
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_git_cheatsheet() -> Vec<Cheatsheet> {
        let json = r#"[
    {
        "description": "quick show branch and file changes",
        "data": [
            "git status -s -b"
        ],
        "tags": [
            "git-status"
        ]
    },
    {
        "description": "quick show submodules status",
        "data": [
            "git submodule status"
        ],
        "tags": [
            "git-submodule"
        ]
    },
    {
        "description": "search for terms/string in commit message history",
        "data": [
            "git log --all --grep='\u003cmessage\u003e'",
            "git log --grep='\u003cmessage\u003e'",
            "git log --author='\u003cusername\u003e'"
        ],
        "tags": []
    }
]"#;
        let cheatsheets: Vec<Cheatsheet> = serde_json::from_str(json).unwrap();
        cheatsheets
    }

    #[test]
    fn test_git_cheatsheet_without_filter_should_return_one() {
        let cheatsheets = get_git_cheatsheet();
        let search = "branch";
        let filter = "";
        match parse_topic(&cheatsheets, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_cheatsheet_without_filter_should_return_two() {
        let cheatsheets = get_git_cheatsheet();
        let search = "quick";
        let filter = "";
        match parse_topic(&cheatsheets, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 2);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_cheatsheet_with_filter_should_return_one() {
        let cheatsheets = get_git_cheatsheet();
        let search = "";
        let filter = "git-status";
        match parse_topic(&cheatsheets, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_cheatsheet_without_search_or_filter_should_return_three() {
        let cheatsheets = get_git_cheatsheet();
        let search = "";
        let filter = "";
        match parse_topic(&cheatsheets, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 3);
            }
            Err(_) => panic!("test failed"),
        }
    }
}
