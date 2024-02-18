use serde::Deserialize;
use std::{env, fs, io, path::PathBuf};

use toml;

#[derive(Clone, Deserialize, Debug)]
pub struct TopicContent {
  pub knowledges: Vec<Knowledge>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Knowledge {
    pub description: String,
    pub data: Vec<String>,
    pub tags: Vec<String>,
}

impl Knowledge {
    fn colorized_match(&self, search: &str) -> String {
        let colorized_search = format!("\x1b[91;1m{}\x1b[0m", search);
        return self.description.replace(search, &colorized_search);
    }

    fn display_nocolor(&self, topic: &str, search: &str, match_colored: bool) {
        println!("");

        let description = if match_colored {
            self.colorized_match(&search)
        } else {
            self.description.clone()
        };

        if self.tags.len() > 0 {
            println!("[{}][{}] {}", topic, self.tags.join(" "), description);
        } else {
            println!("[{}] {}", topic, description);
        }

        for d in self.data.iter() {
            // Check if the data is a comment to display it with dedicated color
            if d.starts_with("//") {
                println!("{}", d);
            } else {
                println!("- {}", d);
            }
        }
    }

    fn display_colorized(&self, topic: &str, search: &str, match_colored: bool) {
        println!("");

        let description = if match_colored {
            self.colorized_match(&search)
        } else {
            self.description.clone()
        };

        if self.tags.len() > 0 {
            println!(
                "\x1b[93;1m[{}]\x1b[0m\x1b[94;1m[{}]\x1b[0m",
                topic,
                self.tags.join(" ")
            );
        } else {
            println!("\x1b[93;1m[{}]\x1b[0m", topic);
        }

        println!("\x1b[92;1m#\x1b[0m {}", description);

        for d in self.data.iter() {
            // Check if the data is a comment to display it with dedicated color
            if d.starts_with("//") {
                println!("\x1b[90;1m{}\x1b[0m", d);
            } else {
                println!("\x1b[92;1m>>>\x1b[0m \x1b[95m{}\x1b[0m", d);
            }
        }
    }

    pub fn display(&self, topic: &str, search: &str, match_colored: bool) {
        match env::var("KSEARCH_COLORED") {
            Ok(_) => self.display_colorized(&topic, &search, match_colored),
            _ => self.display_nocolor(&topic, &search, match_colored),
        }
    }
}

pub fn from_file(path: &PathBuf) -> TopicContent {
    let content = fs::read_to_string(path).expect("could not read the toml file");
    let topic_content: TopicContent = toml::from_str(&content).unwrap();
    topic_content
}

fn parse_topic(
    knowledges: &[Knowledge],
    search: &str,
    filter: &str,
) -> io::Result<Vec<Knowledge>> {
    let matches: Vec<Knowledge> = if filter.len() > 0 {
        let f = String::from(filter);
        if search.len() > 0 {
            knowledges
                .iter()
                .filter(|e| e.description.contains(search) && e.tags.contains(&f))
                .map(|c| c.clone())
                .collect()
        } else {
            knowledges
                .iter()
                .filter(|e| e.tags.contains(&f))
                .map(|c| c.clone())
                .collect()
        }
    } else {
        if search.len() > 0 {
            knowledges
                .iter()
                .filter(|e| e.description.contains(search))
                .map(|c| c.clone())
                .collect()
        } else {
            knowledges.iter().map(|c| c.clone()).collect()
        }
    };

    Ok(matches)
}

pub fn is_toml_file(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => extension == "toml",
        None => false,
    }
}

pub fn show_topic(
    knowledges: &[Knowledge],
    topic: &str,
    search: &str,
    filter: &str,
    match_colored: bool,
) -> bool {
    let mut knowledges_found = false;
    match parse_topic(&knowledges, &search, &filter) {
        Ok(knowledges) => {
            knowledges_found = true;
            for ch in knowledges.iter() {
                ch.display(&topic, &search, match_colored);
            }
        }
        _ => println!("No topic found."),
    }
    knowledges_found
}

pub fn find_files(
    path: &PathBuf,
    search: &str,
    filter: &str,
    inventory_flag: bool,
    match_colored: bool,
) -> io::Result<bool> {
    let mut knowledges_found = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if is_toml_file(&p) {
            let tmp_path = p.clone();
            // Safe to unwrap cause we checked that we got a file with a toml extension previously
            let topic_ostr = tmp_path.file_stem().unwrap();
            if let Some(topic) = topic_ostr.to_str() {
                if inventory_flag {
                    println!("{}", topic);
                } else {
                    let topic_content = from_file(&p);
                    let knowledges = topic_content.knowledges;
                    knowledges_found =
                        show_topic(&knowledges, &topic, &search, &filter, match_colored);
                }
            } // XXX: Add Else and log
        }
    }
    Ok(knowledges_found)
}

#[cfg(test)]
mod tests {
    use super::*;

     fn get_git_knowledge() -> Vec<Knowledge> {
         let toml_content = r#"
[[knowledges]]
description = "quick show branch and file changes"
data = [
    "git status -s -b"
]
tags= [
    "git-status"
]

[[knowledges]]
description = "quick show submodules status"
data = [
    "git submodule status"
]
tags = [
    "git-submodule"
]

[[knowledges]]
description= "search for terms/string in commit message history"
data= [
    "git log --all --grep='\u003cmessage\u003e'",
    "git log --grep='\u003cmessage\u003e'",
    "git log --author='\u003cusername\u003e'"
]
tags= []
"#;
         let tc : TopicContent = toml::from_str(toml_content).unwrap();
         tc.knowledges
     }

    #[test]
    fn test_git_knowledge_without_filter_should_return_one() {
        let knowledges = get_git_knowledge();
        let search = "branch";
        let filter = "";
        match parse_topic(&knowledges, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_knowledge_without_filter_should_return_two() {
        let knowledges = get_git_knowledge();
        let search = "quick";
        let filter = "";
        match parse_topic(&knowledges, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 2);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_knowledge_with_filter_should_return_one() {
        let knowledges = get_git_knowledge();
        let search = "";
        let filter = "git-status";
        match parse_topic(&knowledges, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                assert_eq!(res[0].data, ["git status -s -b"]);
                assert_eq!(res[0].tags, ["git-status"]);
            }
            Err(_) => panic!("test failed"),
        }
    }

    #[test]
    fn test_git_knowledge_without_search_or_filter_should_return_three() {
        let knowledges = get_git_knowledge();
        let search = "";
        let filter = "";
        match parse_topic(&knowledges, &search, &filter) {
            Ok(res) => {
                assert_eq!(res.len(), 3);
            }
            Err(_) => panic!("test failed"),
        }
    }
}
