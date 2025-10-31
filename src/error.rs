use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walk error: {0}")]
    Walk(String),

    #[error("No $EDITOR and no fallback")]
    NoEditor,

    #[error("Failed to spawn editor")]
    EditorSpawn,
}
