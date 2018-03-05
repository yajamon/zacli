use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

pub const CONFIG_FILENAME: &str = ".zacli.toml";

pub struct Config {}

impl Config {
    pub fn default_path() -> ::Result<PathBuf> {
        let mut path = env::home_dir()
            .ok_or("Homeディレクトリがみつかりませんでした".to_string())?;
        path.push(CONFIG_FILENAME);
        Ok(path)
    }
    pub fn open_config(path: &Path) -> ::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
