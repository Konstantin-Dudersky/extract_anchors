mod error;
mod extract_anchor_name_from_line;
pub use error::Error;
use walkdir::WalkDir;

use std::{
    collections::HashMap,
    fs::{read_to_string, remove_dir},
};
use tracing::error;

const SOURCE_DIR: &str = "source_dir";
const TARGET_DIR: &str = "target_dir";

fn main() {
    // Удаляем целевую папку
    let result = delete_target_directory(TARGET_DIR);
    if let Err(err) = result {
        error!("Error: {err}");
    }

    // Рекурсивно проходим по исходной папке
    for entry in WalkDir::new(SOURCE_DIR) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let file = read_to_string(entry.path()).unwrap();

        for (index, line) in file.split("\n").enumerate() {
            if line.contains("ANCHOR") {}
        }

        println!("{}", entry.path().display());
    }

    // Анализируем каждый файл, сохраняем отрывок в целевой папке
}

fn delete_target_directory(target_dir: &str) -> Result<(), crate::Error> {
    remove_dir(target_dir).map_err(crate::Error::RemoveTargetDir)?;
    Ok(())
}

pub struct InfoLine {
    begin: usize,
    end: usize,
}
