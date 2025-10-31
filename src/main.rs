mod config;
mod error;
mod fs;

use crate::config::Config;
use crate::error::AppError;
use crate::fs::walker::FileWalker;

fn main() -> Result<(), AppError> {
    let config = Config::try_parse()?;

    println!("Root: {}", config.root.display());
    println!("All: {}", config.all);
    println!("Editor: {}", config.editor);

    let walker = FileWalker::new(&config);
    for path in walker.into_paths().take(100) {
        let path = path?;
        println!("{}", path);
    }

    Ok(())
}
