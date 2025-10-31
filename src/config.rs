use crate::error::AppError;
use clap::Parser;
use std::env::VarError;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author = "Joel Palmer", version, about = "Rust Fuzzy Finder")]
pub struct Config {
    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub multi: bool,

    #[arg(long, value_parser = parse_height)]
    pub height: Option<u16>,

    #[arg(default_value = ".")]
    pub root: PathBuf,

    #[arg(default_value = "nano")]
    pub editor: String,
}

impl Config {
    pub fn try_parse() -> Result<Self, AppError> {
        let mut cfg = <Self as Parser>::parse();

        if cfg.root == PathBuf::new() {
            cfg.root = std::env::current_dir().map_err(AppError::from)?;
        }

        cfg.editor = std::env::var("EDITOR")
            .map_err(|e| match e {
                VarError::NotPresent => AppError::NoEditor,
                VarError::NotUnicode(s) => {
                    AppError::Walk(format!("EDITOR contains invalid Unicode: {:?}", s))
                }
            })
            .unwrap_or("nano".into());

        Ok(cfg)
    }
}

fn parse_height(s: &str) -> Result<u16, String> {
    if let Some(percentage) = s.strip_suffix('%') {
        let percent: u16 = percentage
            .parse()
            .map_err(|_| format!("Invalid percentage: {percentage}"))?;
        if percent > 100 {
            return Err("Percentage cannot exceed 100".into());
        }
        let (_cols, rows) =
            crossterm::terminal::size().map_err(|e| format!("Failed to get terminal size: {e}"))?;
        let height = (rows as u32 * percent as u32 / 100) as u16;
        if height == 0 {
            return Err("Computed height is zero".into());
        }
        Ok(height)
    } else {
        s.parse().map_err(|_| format!("Invalid height value: {s}"))
    }
}
