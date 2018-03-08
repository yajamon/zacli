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
pub fn save_config(path: &Path, config: &Config) -> ::Result<()> {
    let contents = ::toml::to_string(config)?;
    write_contents(path, &contents);
    Ok(())
}
pub fn new() -> Config {
    migrate(new_core_config())
}
fn new_core_config() -> ConfigList {
    ConfigList::V01(ConfigV01 {
        version: 1,
        access_key: "YOUR_ACCESS_KEY".to_string(),
        access_secret: "YOUR_ACCESS_SECRET".to_string(),
    })
}

fn load_contents(path: &Path) -> ::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_contents(path: &Path, contents: &str) -> ::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
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
    // 次のConfig versionが生まれたときに必要になるのだ
    // loop {
    //     config = match config {
    //         ConfigList::V01(c) => return c,
    //     }
    // }
    let ConfigList::V01(c) = config;
    c
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
