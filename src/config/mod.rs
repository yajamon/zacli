use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

pub const CONFIG_FILENAME: &str = ".zacli.toml";

pub type Config = ConfigV01;

pub fn default_path() -> ::Result<PathBuf> {
    let mut path = env::home_dir()
        .ok_or("Homeディレクトリがみつかりませんでした".to_string())?;
    path.push(CONFIG_FILENAME);
    Ok(path)
}
pub fn open_config(path: &Path) -> ::Result<Config> {
    let contents = load_contents(path)?;
    let config = migrate(convert_to_config(&contents)?);
    Ok(config)
}

fn load_contents(path: &Path) -> ::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn convert_to_config(contents: &str) -> ::Result<ConfigList> {
    let parsed = contents.parse::<Value>()?;
    let version = parsed
        .as_table()
        .and_then(|table| table.get("version"))
        .and_then(|value| value.as_integer())
        .ok_or("Missing version".to_string())?;
    let config = match version {
        1 => ConfigList::V01(parsed.try_into()?),
        _ => return Err("Unknown config version".to_string().into()),
    };
    Ok(config)
}

fn migrate(config: ConfigList) -> Config {
    loop {
        config = match config {
            ConfigList::V01(c) => return c,
        }
    }
}

enum ConfigList {
    V01(ConfigV01),
}
#[derive(Serialize, Deserialize)]
pub struct ConfigV01 {
    pub version: u8,
    pub access_key: String,
    pub access_secret: String,
}
