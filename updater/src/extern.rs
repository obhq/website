use std::fs;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::github_request;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub keys: Keys,
    pub locations: Locations,
    pub ids: Ids,
}

#[derive(Serialize, Deserialize)]
pub struct Keys {
    pub compat_github_url: String,
    pub github_url: String,
    pub github_token: String,
    pub homebrew_token: String,
    pub ps4_useragent: String,
    pub tmdb_hash: String,
    pub workflow_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Locations {
    pub games_folder: String,
    pub homebrew_folder: String,
    pub homebrew_database: String,
    pub database: String,
    pub stats_folder: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ids {
    pub homebrew: u64,
    pub playable: u64,
    pub ingame: u64,
    pub intro: u64,
    pub boots: u64,
    pub nothing: u64,
}

pub fn config_creator() -> Config {
    use std::path::Path;

    const FOLDER_CONFIG: &str = "./config/";

    let config_data: Config = {
        let config = Config {
            keys: Keys {
                compat_github_url: "https://api.github.com/repos/obhq/compatibility".to_string(),
                github_url: "https://api.github.com/repos/obhq/obliteration".to_string(),
                github_token: "".to_string(),
                homebrew_token: "".to_string(),
                ps4_useragent: "Mozilla/5.0 (PlayStation; PlayStation 4/11.00) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15".to_string(),
                tmdb_hash: "".to_string(),
                workflow_url: "https://api.github.com/repos/obhq/obliteration/actions/workflows/36859008/runs".to_string(),

            },
            locations: Locations {
                games_folder: "./images/games/".to_string(),
                homebrew_folder: "./images/hb/".to_string(),
                homebrew_database: "./HBstore.db".to_string(),
                database: "./main.db".to_string(),
                stats_folder: "".to_string(),
            },
            ids: Ids {
                homebrew: 6164722453,
                playable: 6164497050,
                ingame: 6164500133,
                intro: 6164505028,
                boots: 6164509950,
                nothing: 6164514963,
            },
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


    if !Path::new(config_data.locations.games_folder.as_str()).exists() {
        fs::create_dir_all(config_data.locations.games_folder.as_str()).expect("Error creating folders!");
    }
    if !Path::new(config_data.locations.homebrew_folder.as_str()).exists() {
        fs::create_dir_all(config_data.locations.homebrew_folder.as_str()).expect("Error creating folders!");
    }
    if !Path::new(config_data.locations.stats_folder.as_str()).exists() {
        fs::create_dir_all(config_data.locations.stats_folder.as_str()).expect("Error creating folders!");
    }

    config_data
}


pub enum ImageTypes<'lt> {
    Game(&'lt str, String, &'lt String, &'lt Vec<u8>),
    Hb(&'lt str, String, &'lt Connection, &'lt String),
}

pub fn image_handler(image_type: ImageTypes) -> Result<(), anyhow::Error> {
    use hmac::{Hmac, Mac};
    use sha1::Sha1;

    return match image_type {
        // Game issue
        ImageTypes::Game(user_agent, path, code, tmdb_hash) => {
            // create the url based on the tmdb hash
            let mut hmac = Hmac::<Sha1>::new_from_slice(tmdb_hash)?;
            let image_code = format!("{}_00", code);
            hmac.update(image_code.as_bytes());
            let hmac = hmac.finalize().into_bytes();
            let hash = hex::encode_upper(hmac);

            let hash_url = format!(
                "https://tmdb.np.dl.playstation.net/tmdb2/{}_{}/{}.json",
                image_code, hash, image_code
            );

            // get image url from link
            let url = match ureq::get(hash_url.as_ref())
                .set("User-Agent", user_agent)
                .call()
            {
                Ok(response) => {
                    let temp: Value = response.into_json().expect("Failed to parse JSON");

                    temp.get("icons")
                        .ok_or_else(|| anyhow::anyhow!("Failed to parse JSON"))?
                        .get(0)
                        .ok_or_else(|| anyhow::anyhow!("Failed to parse JSON"))?
                        .get("icon")
                        .and_then(Value::as_str)
                        .ok_or_else(|| anyhow::anyhow!("Failed to parse JSON"))?
                        .to_string()
                }
                Err(response) => return Err(anyhow::anyhow!("Image request failed: {}", response)),
            };

            // DOWNLOAD IMAGE ;3
            match image_downloader(url, user_agent, path) {
                Ok(_) => Ok(()),
                Err(err) => Err(anyhow::anyhow!("Error while downloading image: {}", err)),
            }
        }

        // Homebrew issue
        ImageTypes::Hb(user_agent, path, homebrew_db, title) => {
            let mut stmt = homebrew_db
                .prepare("SELECT image FROM homebrews WHERE name = (?1) COLLATE NOCASE")?;

            let url = {
                let temp = stmt
                    .query_map([&title], |row| Ok(row.get::<_, String>(0)))?
                    .next();

                match temp {
                    Some(result) => result??,
                    None => return Err(anyhow::anyhow!("No image url found for {}", title)),
                }
            };

            // DOWNLOAD IMAGE ;3
            match image_downloader(url, user_agent, path) {
                Ok(_) => Ok(()),
                Err(err) => Err(anyhow::anyhow!("Error while downloading image: {}", err)),
            }
        }
    };
}

fn image_downloader(url: String, user_agent: &str, location: String) -> Result<(), anyhow::Error> {
    let response = ureq::get(&url).set("User-Agent", user_agent).call()?;
    let mut reader = response.into_reader();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let img = image::load_from_memory(&buffer)?;
    let resized_img = img.resize_exact(256, 256, image::imageops::Lanczos3);
    resized_img.save_with_format(location, image::ImageFormat::Avif)?;
    Ok(())
}

#[derive(Serialize)]
struct StatsJson {
    stars: String,
    issues: String,
    devbuilds: String,
}

pub fn stats_creator(path: &str, token: &str, github_main: &str, github_comp: &str, workflow: &str) -> Result<(), anyhow::Error> {
    let stars = github_request(github_main, token)
        .get("stargazers_count")
        .and_then(Value::as_u64)
        .expect("Failed to parse JSON o.o!");

    let issues = github_request(github_comp, token)
        .get("open_issues_count")
        .and_then(Value::as_u64)
        .expect("Failed to parse JSON o.o!");

    let devbuilds = github_request(workflow, token)
        .get("workflow_runs")
        .unwrap()
        .get(0)
        .unwrap()
        .get("run_number")
        .and_then(Value::as_u64)
        .expect("Failed to parse JSON o.o!");

    let stats_json: StatsJson = StatsJson {
        stars: stars.to_string(),
        issues: issues.to_string(),
        devbuilds: devbuilds.to_string(),
    };

    let stats_string = serde_json::to_string(&stats_json).expect("Error serializing the stats.json!");
    fs::write(format!("{}stats.json", path), stats_string).expect("Error creating config!");
    Ok(())
}