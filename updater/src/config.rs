use std::fs;

use hex::FromHex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    compat_github_url: String,
    github_url: String,
    github_token: String,
    homebrew_token: String,
    ps4_useragent: String,
    tmdb_hash: String,
    workflow_url: String,

    games_folder: String,
    homebrew_folder: String,
    homebrew_database: String,
    database: String,
    stats_folder: String,

    homebrew: u64,
    playable: u64,
    ingame: u64,
    intro: u64,
    boots: u64,
    nothing: u64,
}

pub struct AppConfig {
    pub api_url: String,
    pub github_url: String,
    pub api_token: String,
    pub homebrew_token: String,
    pub ps4_useragent: String,
    pub tmdb_hash: Vec<u8>,
    pub workflow_url: String,

    pub games_folder: String,
    pub homebrew_folder: String,
    pub homebrew_database: String,
    pub database: String,
    pub stats_folder: String,

    pub tag_homebrew: u64,
    pub tag_playable: u64,
    pub tag_ingame: u64,
    pub tag_intro: u64,
    pub tag_boots: u64,
    pub tag_nothing: u64,
}


pub fn get_config() -> AppConfig {
    let config_data: Config = config_creator();

    AppConfig {
        api_url: config_data.compat_github_url,
        github_url: config_data.github_url,
        api_token: config_data.github_token,
        homebrew_token: config_data.homebrew_token,
        ps4_useragent: config_data.ps4_useragent,
        tmdb_hash: Vec::from_hex(config_data.tmdb_hash).expect("Invalid tmdb hash!"),
        workflow_url: config_data.workflow_url,

        games_folder: config_data.games_folder,
        homebrew_folder: config_data.homebrew_folder,
        homebrew_database: config_data.homebrew_database,
        database: config_data.database,
        stats_folder: config_data.stats_folder,

        tag_homebrew: config_data.homebrew,
        tag_playable: config_data.playable,
        tag_ingame: config_data.ingame,
        tag_intro: config_data.intro,
        tag_boots: config_data.boots,
        tag_nothing: config_data.nothing,
    }
}

fn config_creator() -> Config {
    use std::path::Path;

    const FOLDER_CONFIG: &str = "./config/";

    let config_data: Config = {
        let config = Config {
            compat_github_url: "https://api.github.com/repos/obhq/compatibility".to_string(),
            github_url: "https://api.github.com/repos/obhq/obliteration".to_string(),
            github_token: "".to_string(),
            homebrew_token: "".to_string(),
            ps4_useragent: "Mozilla/5.0 (PlayStation; PlayStation 4/11.00) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15".to_string(),
            tmdb_hash: "".to_string(),
            workflow_url: "https://api.github.com/repos/obhq/obliteration/actions/workflows/36859008/runs".to_string(),

            games_folder: "./images/games/".to_string(),
            homebrew_folder: "./images/hb/".to_string(),
            homebrew_database: "./HBstore.db".to_string(),
            database: "./main.db".to_string(),
            stats_folder: "".to_string(),

            homebrew: 6164722453,
            playable: 6164497050,
            ingame: 6164500133,
            intro: 6164505028,
            boots: 6164509950,
            nothing: 6164514963,
        };

        let config_string = toml::to_string_pretty(&config).unwrap();
        let config_file = format!("{}config.toml", FOLDER_CONFIG);

        if !Path::new(&config_file).exists() {
            fs::create_dir_all(FOLDER_CONFIG).expect("Error creating folders!");
            fs::write(&config_file, config_string).expect("Error creating config!");
        }

        let toml_content = fs::read_to_string(&config_file).expect("Error reading config");
        toml::from_str(&toml_content).expect("Error deserializing TOML")
    };


    if !Path::new(config_data.games_folder.as_str()).exists() {
        fs::create_dir_all(config_data.games_folder.as_str()).expect("Error creating folders!");
    }
    if !Path::new(config_data.homebrew_folder.as_str()).exists() {
        fs::create_dir_all(config_data.homebrew_folder.as_str()).expect("Error creating folders!");
    }
    if !Path::new(config_data.stats_folder.as_str()).exists() {
        fs::create_dir_all(config_data.stats_folder.as_str()).expect("Error creating folders!");
    }

    config_data
}