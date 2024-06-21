use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub compat_api_url: String,
    pub main_api_url: String,
    pub github_api_token: String,
    pub homebrew_token: String,
    pub ps4_useragent: String,
    pub tmdb_hex: String,
    pub workflow_url: String,

    pub game_images_folder: String,
    pub homebrew_images_folder: String,
    pub homebrew_database: String,
    pub game_skips_database: String,
    pub database: String,
    pub stats_file: String,

    pub tag_homebrew: u64,
    pub tag_playable: u64,
    pub tag_ingame: u64,
    pub tag_intro: u64,
    pub tag_boots: u64,
    pub tag_nothing: u64,
}

pub fn config_creator() -> Config {
    use std::path::Path;

    const FOLDER_CONFIG: &str = "./config/";

    let config_data: Config = {
        let config_file = format!("{}config.toml", FOLDER_CONFIG);

        if !Path::new(&config_file).exists() {
            let config = Config {
                compat_api_url: "https://api.github.com/repos/obhq/compatibility".to_string(),
                main_api_url: "https://api.github.com/repos/obhq/obliteration".to_string(),
                github_api_token: "".to_string(),
                homebrew_token: "".to_string(),
                ps4_useragent: "Mozilla/5.0 (PlayStation; PlayStation 4/11.00) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15".to_string(),
                tmdb_hex: "".to_string(),
                workflow_url: "https://api.github.com/repos/obhq/obliteration/actions/workflows/36859008/runs".to_string(),

                game_images_folder: "./images/games/".to_string(),
                homebrew_images_folder: "./images/hb/".to_string(),
                homebrew_database: "./HBstore.db".to_string(),
                game_skips_database: "./game_skips.json".to_string(),
                database: "./database.json".to_string(),
                stats_file: "./stats.json".to_string(),

                tag_homebrew: 6164722453,
                tag_playable: 6164497050,
                tag_ingame: 6164500133,
                tag_intro: 6164505028,
                tag_boots: 6164509950,
                tag_nothing: 6164514963,
            };

            // create default config
            fs::create_dir_all(FOLDER_CONFIG).expect("Error creating folders!");
            fs::write(&config_file, toml::to_string_pretty(&config).unwrap()).expect("Error creating config!");

            return config;
        } else {
            let toml_content = fs::read_to_string(&config_file).expect("Error reading config");
            toml::from_str(&toml_content).expect("Error deserializing TOML")
        }
    };

    // create needed folders
    let game_images_folder = Path::new(&config_data.game_images_folder);
    let homebrew_images_folder = Path::new(&config_data.homebrew_images_folder);

    let database_folder = Path::new(&config_data.database).parent().unwrap();
    let hb_database_folder = Path::new(&config_data.homebrew_database).parent().unwrap();
    let game_skips_database_folder = Path::new(&config_data.game_skips_database).parent().unwrap();
    let stats_file_folder = Path::new(&config_data.stats_file).parent().unwrap();


    if !game_images_folder.exists() {
        fs::create_dir_all(game_images_folder).expect("Error creating folders for \"game_image\"!");
    }

    if !homebrew_images_folder.exists() {
        fs::create_dir_all(homebrew_images_folder).expect("Error creating folders for \"homebrew_image\"!");
    }

    if !database_folder.exists() {
        fs::create_dir_all(database_folder).expect("Error creating folders for \"database\"!");
    }

    if !hb_database_folder.exists() {
        fs::create_dir_all(hb_database_folder).expect("Error creating folders for \"hb_database\"!");
    }

    if !game_skips_database_folder.exists() {
        fs::create_dir_all(game_skips_database_folder).expect("Error creating folders for \"game_skips_database\"!");
    }

    if !stats_file_folder.exists() {
        fs::create_dir_all(stats_file_folder).expect("Error creating folders for \"stats_file\"!");
    }

    config_data
}