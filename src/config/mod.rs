use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

pub const CONFIG_FILENAME: &str = ".zacli.toml";

pub type Config = Config_V01;

pub fn default_path() -> ::Result<PathBuf> {
    let mut path = env::home_dir()
        .ok_or("Homeディレクトリがみつかりませんでした".to_string())?;
    path.push(CONFIG_FILENAME);
    Ok(path)
}
pub fn open_config(path: &Path) -> ::Result<Value> {
    let contents = load_contents(path)?;
    let config = contents.as_str().parse::<Value>()?;
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
    let version = parsed["version"]
        .as_integer()
        .ok_or("Missing version".to_string())?;
    let config = match version {
        1 => ConfigList::V01(parsed.try_into()?),
        _ => return Err("Unknown config version".to_string().into()),
    };
    Ok(config)
}

enum ConfigList {
    V01(Config_V01),
}
#[derive(Serialize, Deserialize)]
struct Config_V01 {
    pub version: u8,
    pub access_key: String,
    pub access_secret: String,
}
