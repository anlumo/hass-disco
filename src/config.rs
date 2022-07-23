use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub hass_token: String,
    pub host: String,
    pub port: Option<u16>,
    pub tls: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Entities {
    pub disco: Vec<String>,
    pub input: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub entities: Entities,
}

pub fn get_config(file: impl AsRef<Path>) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    toml::from_slice(&bytes).map_err(|err| err.into())
}
