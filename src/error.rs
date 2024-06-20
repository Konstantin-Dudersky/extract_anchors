#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RemoveTargetDir(std::io::Error),
}
