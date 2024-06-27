use std::path::{Path, PathBuf};

pub fn create_new_path(old_path: &Path, source_dir: &str, target_dir: &str) -> PathBuf {
    let path = old_path.to_string_lossy();

    let source_dir = source_dir.trim_end_matches('/');
    let target_dir = target_dir.trim_end_matches('/');

    let path = path.replace(source_dir, target_dir);
    let path = path.split('.').collect::<Vec<&str>>();
    let path = path[0..path.len() - 1].join(".");

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
