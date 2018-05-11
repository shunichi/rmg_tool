use std;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub enum MigrationStatus {
    Up,
    Down,
}

impl MigrationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MigrationStatus::Up => " up ",
            MigrationStatus::Down => "down",
        }
    }
}

pub struct Migration {
    pub version: String,
    pub description: String,
    pub status: MigrationStatus,
}

fn to_file_name(entry: &std::fs::DirEntry) -> Option<String> {
    entry.file_type().ok().and_then(|file_type|
        if file_type.is_file() {
            let file_name = entry.file_name();
            let lossy_name = file_name.to_string_lossy();
            Some(lossy_name.into_owned())
        } else {
            None
        }
    )
}

fn split_file_name(name: &str) -> Option<(String, String)>{
    let re = Regex::new(r"(\d+)_(.+)\.rb").unwrap();
    re.captures(name).map(|c|
        (c.get(1).unwrap().as_str().to_string(), c.get(2).unwrap().as_str().to_string())
    )
}

fn capitalize(s: &str) -> String {
    if s.len() == 0 {
        String::from("")
    } else {
        let first_char = s.chars().nth(0).unwrap();
        let first_len = first_char.len_utf8();
        let upcased_len = first_char.to_uppercase().fold(0, |acc, c| acc + c.len_utf8());
        let mut string = String::with_capacity(s.len() - first_len + upcased_len);
        for uc in first_char.to_uppercase() {
            string.push(uc);
        }
        string.push_str(&s[first_len..]);
        string
    }       
}

fn to_capitalized_human_string(s: &str) -> String {
    capitalize(&s.replace('_', " "))
}

pub fn migration_files() -> HashMap<String, Migration> {
    let entries = fs::read_dir("./db/migrate").unwrap();
    entries.filter_map( |r| 
        r.ok()
            .and_then(|entry| to_file_name(&entry))
            .and_then(|name| split_file_name(&name))
            .and_then(|(version, description)| 
                Some(Migration {
                    status: MigrationStatus::Down,
                    version: version,
                    description: to_capitalized_human_string(&description),
                })
            )
    ).map(|m| (m.version.clone(), m)).collect()
}
