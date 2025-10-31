mod config;
mod error;
mod fs;

use crate::config::Config;
use crate::error::AppError;

fn main() -> Result<(), AppError> {
    let config = Config::try_parse()?;

    println!("Root: {}", config.root.display());
    println!("All: {}", config.all);
    println!("Editor: {}", config.editor);

    Ok(())
}
