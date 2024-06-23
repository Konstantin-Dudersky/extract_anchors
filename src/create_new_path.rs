use std::path::{Path, PathBuf};

pub fn create_new_path(old_path: &Path, source_dir: &str, target_dir: &str) -> PathBuf {
    let path = old_path.to_string_lossy();
    let path = path.replace(source_dir, target_dir);
    let path = path.split('.').collect::<Vec<&str>>();
    let path = path[0..path.len() - 1].join(".");

    // let mut path = path.split('/').collect::<Vec<&str>>();
    // let len = path.len();

    // // Меняем корневую папку
    // path[0] = root_folder;

    // // Меняем название файла на папку
    // let filename = path[len - 1];
    // let folder_name = filename.split('.').collect::<Vec<&str>>()[0];
    // path[len - 1] = folder_name;

    // let new_path = path.join("/");
    PathBuf::from(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let source_dir = "old_folder";
        let target_dir = "new_folder";
        let old_path = Path::new("old_folder/subfolder/file.rs");
        let new_path = create_new_path(old_path, source_dir, target_dir);
        assert_eq!(new_path.to_string_lossy(), "new_folder/subfolder/file");
    }
}
