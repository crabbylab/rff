use rff::config::Config;
use rff::error::AppError;
use rff::fs::walker::FileWalker;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

fn test_config(dir: &TempDir, all: bool) -> Config {
    Config {
        root: dir.path().to_path_buf(),
        all,
        multi: false,
        height: None,
        editor: "nano".into(),
    }
}

#[test]
fn respects_gitignore() -> Result<(), AppError> {
    let dir = TempDir::new().unwrap();

    File::create(dir.path().join(".gitignore"))?.write_all(b"target/\n*.log\n")?;

    // Ignored files
    fs::create_dir(dir.path().join("target")).unwrap();
    File::create(dir.path().join("target/debug.rs")).unwrap();
    File::create(dir.path().join("app.log")).unwrap();

    // Visible file
    fs::create_dir(dir.path().join("src")).unwrap();
    File::create(dir.path().join("src/main.rs")).unwrap();

    let cfg = test_config(&dir, false);
    let mut paths: Vec<String> = FileWalker::new(&cfg)
        .into_paths()
        .collect::<Result<Vec<_>, _>>()?;

    paths.sort();

    assert_eq!(paths, vec!["src/main.rs".to_string()]);
    Ok(())
}

#[test]
fn includes_ignored_when_all() -> Result<(), AppError> {
    let dir = TempDir::new().unwrap();

    File::create(dir.path().join(".gitignore"))?.write_all(b"target/\n")?;
    fs::create_dir(dir.path().join("target")).unwrap();
    File::create(dir.path().join("target/debug.rs")).unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    File::create(dir.path().join("src/lib.rs")).unwrap();

    let cfg = test_config(&dir, true);
    let paths: Vec<String> = FileWalker::new(&cfg)
        .into_paths()
        .collect::<Result<Vec<_>, _>>()?;

    let expected = ["src/lib.rs", "target/debug.rs"];
    for exp in expected {
        assert!(paths.contains(&exp.to_string()));
    }
    Ok(())
}

#[test]
fn skips_hidden_by_default() -> Result<(), AppError> {
    let dir = TempDir::new().unwrap();

    fs::create_dir(dir.path().join(".cache")).unwrap();
    File::create(dir.path().join(".cache/secret.txt")).unwrap();
    File::create(dir.path().join("README.md")).unwrap();

    let cfg = test_config(&dir, false);
    let paths: Vec<String> = FileWalker::new(&cfg)
        .into_paths()
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(paths, vec!["README.md".to_string()]);
    Ok(())
}

#[test]
fn emits_only_regular_files_relative() -> Result<(), AppError> {
    let dir = TempDir::new().unwrap();

    fs::create_dir(dir.path().join("src")).unwrap();
    File::create(dir.path().join("src/mod.rs")).unwrap();
    fs::create_dir(dir.path().join("assets")).unwrap();

    let cfg = test_config(&dir, true);
    let paths: Vec<String> = FileWalker::new(&cfg)
        .into_paths()
        .collect::<Result<Vec<_>, _>>()?;

    assert_eq!(paths, vec!["src/mod.rs".to_string()]);
    Ok(())
}

#[test]
fn handles_walk_error() -> Result<(), AppError> {
    let dir = TempDir::new().unwrap();
    let bad_path = dir.path().join("bad");
    fs::create_dir(&bad_path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bad_path).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&bad_path, perms).unwrap();
    }

    let cfg = test_config(&dir, true);
    let mut results = FileWalker::new(&cfg).into_paths();
    let first_err = results.find_map(|r| r.err());

    assert!(first_err.is_some());
    Ok(())
}
