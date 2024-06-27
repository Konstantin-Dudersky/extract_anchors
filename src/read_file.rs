use std::fs::read_to_string;

use walkdir::DirEntry;

pub fn read_file(entry: DirEntry) -> crate::Result<String> {
    read_to_string(entry.path()).map_err(|e| crate::Error::ReadFile {
        file_name: entry.file_name().to_string_lossy().into_owned(),
        error: e.to_string(),
    })
}
