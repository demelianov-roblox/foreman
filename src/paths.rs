//! Contains all of the paths that Foreman needs to deal with.

use std::{fs, io, path::PathBuf};

static DEFAULT_USER_CONFIG: &str = include_str!("../resources/default-foreman.toml");

pub fn base_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".foreman");
    dir
}

pub fn tools_dir() -> PathBuf {
    let mut dir = base_dir();
    dir.push("tools");
    dir
}

pub fn bin_dir() -> PathBuf {
    let mut dir = base_dir();
    dir.push("bin");
    dir
}

pub fn user_config() -> PathBuf {
    let mut path = base_dir();
    path.push("foreman.toml");
    path
}

pub fn create() -> io::Result<()> {
    fs::create_dir_all(base_dir())?;
    fs::create_dir_all(bin_dir())?;
    fs::create_dir_all(tools_dir())?;

    let config = user_config();
    if let Err(err) = fs::metadata(&config) {
        if err.kind() == io::ErrorKind::NotFound {
            fs::write(&config, DEFAULT_USER_CONFIG)?;
        } else {
            return Err(err);
        }
    }

    Ok(())
}
