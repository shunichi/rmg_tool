use std;
use std::fs;

pub enum MigrationStatus {
    Up,
    Down,
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

pub fn migration_files() -> Vec<Migration> {
    let entries = fs::read_dir("./db/migrate").unwrap();
    entries.filter_map( |r| 
        r.ok()
            .and_then(|entry| to_file_name(&entry))
            .and_then(|name| 
                Some(Migration {
                    status: MigrationStatus::Up,
                    version: name,
                    description: String::from(""),
                })
            )
    ).collect()
}
