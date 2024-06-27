//! Тип ошибок

/// Тип ошибок
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RemoveTargetDir(std::io::Error),

    #[error("File: {file_name}, no start anchor for: {anchor_name}")]
    NoStartAnchor {
        file_name: String,
        anchor_name: String,
    },

    #[error("File: {file_name}, no end anchor for: {anchor_name}")]
    NoEndAnchor {
        file_name: String,
        anchor_name: String,
    },

    #[error("File: {file_name}, end anchor before start: {anchor_name}")]
    EndAnchorBeforeStart {
        file_name: String,
        anchor_name: String,
    },

    #[error("Wrong CLI parameters: {0}. Need 2: source_dir and target_dir")]
    CliParameters(String),

    #[error("Error reading file: {file_name}, error: {file_name}")]
    ReadFile { file_name: String, error: String },
}
