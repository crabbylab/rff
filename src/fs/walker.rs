use crate::config::Config;
use crate::error::AppError;
use ignore::{WalkBuilder, WalkState};
use std::path::PathBuf;
use std::sync::mpsc;

pub struct FileWalker {
    root: PathBuf,
    walker: ignore::WalkParallel,
}

impl FileWalker {
    pub fn new(config: &Config) -> Self {
        let mut builder = WalkBuilder::new(&config.root);
        builder.hidden(!config.all);

        if !config.all {
            let gitignore_path = config.root.join(".gitignore");
            if gitignore_path.exists() {
                let _ = builder.add_ignore(&gitignore_path);
            }
            builder.git_ignore(true);
        } else {
            builder.git_ignore(false);
        }

        builder.follow_links(true);

        Self {
            root: config.root.clone(),
            walker: builder.build_parallel(),
        }
    }

    pub fn into_paths(self) -> impl Iterator<Item = Result<String, AppError>> {
        let root = self.root;
        let (tx, rx) = mpsc::channel();

        self.walker.run(move || {
            let tx = tx.clone();
            let root = root.clone();
            Box::new(move |entry| {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        let _ = tx.send(Err(AppError::Walk(err.to_string())));
                        return WalkState::Skip;
                    }
                };

                if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    return WalkState::Continue;
                }

                let path = entry.path();
                let rel = match path.strip_prefix(&root) {
                    Ok(rel) => rel,
                    Err(_) => {
                        let _ = tx.send(Err(AppError::Walk("Path not under root".into())));
                        return WalkState::Skip;
                    }
                };

                let _ = tx.send(Ok(rel.to_string_lossy().into_owned()));
                WalkState::Continue
            })
        });

        rx.into_iter()
    }
}
