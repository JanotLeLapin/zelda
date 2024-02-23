use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Web {
    pub port: u16,
    pub host: String,
    pub ui_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Database {
    pub port: u16,
    pub host: String,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub music_path: String,
    pub web: Web,
    pub database: Database,
}
