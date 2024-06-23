//! Утилита извлечения отрывков исходного кода в отдельные файлы

#![warn(missing_docs)]

mod anchors_in_file;
mod create_new_path;
mod error;
mod extract_anchor_name_from_line;
mod result;
mod unindent;

pub use error::Error;
pub use result::Result;

use std::{
    env,
    fs::{read_to_string, remove_dir_all},
};

use tracing::{debug, error, level_filters::LevelFilter, warn};
use walkdir::WalkDir;

use anchors_in_file::{AnchorKind, AnchorsInFile};
use create_new_path::create_new_path;
use extract_anchor_name_from_line::extract_anchor_name_from_line;
use unindent::unindent;

const ANCHOR_START: &str = "ANCHOR:";
const ANCHOR_END: &str = "ANCHOR_END:";

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    match main_() {
        Ok(_) => (),
        Err(err) => error!("{err}"),
    }
}

fn main_() -> crate::Result<()> {
    // Читаем параметры командной строки
    let args: Vec<String> = env::args().collect();
    let (source_dir, target_dir) = match &args[..] {
        [_, a, b] => (a.clone(), b.clone()),
        _ => {
            // error!("Wrong arguments. Need 2");
            let err = format!("{:?}", args);
            let err = crate::Error::CliParameters(err);
            return Err(err);
        }
    };

    // Удаляем целевую папку
    let res = delete_target_directory(&target_dir);
    if let Err(err) = res {
        warn!("{}", err);
    }

    // Рекурсивно проходим по исходной папке
    for entry in WalkDir::new(&source_dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let file = read_to_string(entry.path()).unwrap();
        let file = file.split('\n').collect::<Vec<&str>>();

        let mut info = AnchorsInFile::new(entry.path().to_str().unwrap());
        for (line_number, line) in file.iter().enumerate() {
            let anchor_name = extract_anchor_name_from_line(line, ANCHOR_START);
            if let Some(anchor_name) = anchor_name {
                info.push(&anchor_name, line_number, AnchorKind::Start)
            }

            let anchor_name = extract_anchor_name_from_line(line, ANCHOR_END);
            if let Some(anchor_name) = anchor_name {
                info.push(&anchor_name, line_number, AnchorKind::End)
            }
        }
        let info = info.info().unwrap();
        if info.is_empty() {
            continue;
        }
        debug!("File: {}; anchors: {:?}", entry.path().display(), info);
        let new_path = create_new_path(entry.path(), &source_dir, &target_dir);
        std::fs::create_dir_all(&new_path).unwrap();
        for info_part in info.iter() {
            let mut new_file = vec![];
            for line in file
                .iter()
                .skip(info_part.1)
                .take(info_part.2 - info_part.1 + 1)
            {
                new_file.push(*line);
            }
            println!("{:?}", new_file);
            let new_path = format!("{}/{}.rs", new_path.to_string_lossy(), info_part.0);
            let new_file = unindent(&new_file);
            std::fs::write(new_path, new_file).unwrap();
        }
    }

    // Анализируем каждый файл, сохраняем отрывок в целевой папке
    Ok(())
}

fn delete_target_directory(target_dir: &str) -> crate::Result<()> {
    remove_dir_all(target_dir).map_err(crate::Error::RemoveTargetDir)?;
    Ok(())
}
