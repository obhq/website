use std::fs;

use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;

use crate::config::Config;

pub enum ImageTypes<'lt> {
    Game(&'lt str, String, &'lt String, &'lt String),
    Hb(&'lt str, String, &'lt Connection, &'lt String),
}

pub fn image_handler(image_type: ImageTypes) -> Result<(), anyhow::Error> {
    use hmac::{Hmac, Mac};
    use sha1::Sha1;
    use hex::FromHex;

    return match image_type {
        // Game issue
        ImageTypes::Game(user_agent, path, code, tmdb_hex) => {
            // create the url based on the tmdb hash
            let mut hmac = Hmac::<Sha1>::new_from_slice(&Vec::from_hex(tmdb_hex)?)?;
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
    use crate::github_request;

    let stars = github_request(github_main, token)
        .get("stargazers_count")
        .and_then(Value::as_u64)
        .expect("Failed to parse JSON o.o!");

    let issues = github_request(github_comp, token)
        .get("open_issues_count")
        .and_then(Value::as_u64)
        .expect("Failed to parse JSON o.o!");

    let dev_builds = github_request(workflow, token)
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
        devbuilds: dev_builds.to_string(),
    };

    let stats_string = serde_json::to_string(&stats_json)?;

    fs::write(path, stats_string)?;
    Ok(())
}

pub fn homebrew_database_updater(config: &Config) -> Result<(), anyhow::Error> {
    use md5::{Digest, Md5};
    use chrono::{Timelike, Utc};
    use hex::encode;
    use std::fs::File;
    use std::path::Path;

    let minute: u32 = Utc::now().minute();
    //let minute: u32 = 58; // for debug

    // Checks if homebrew database is up-to-date
    if !(4..=57).contains(&minute) || !Path::new(&config.homebrew_database).exists() {
        let hash_response = ureq::get("https://api.pkg-zone.com/api.php?db_check_hash=true")
            .set("User-Agent", &config.homebrew_token)
            .call()?;

        let new_hash: String = {
            let body: Value = hash_response.into_json()?;

            body.get("hash")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow::anyhow!("Error while getting new_hash!"))?
                .to_string()
        };

        // this also checks if the file exists, if it doesn't the hash will not match and the database will be downloaded :3!
        let local_hash: String = match fs::read(&config.homebrew_database) {
            Ok(file) => encode(Md5::digest(file)),
            Err(err) => {
                println!("Homebrew Database not found: {}", err);
                "0".to_string()
            }
        };

        // Compares the current hash with the new hash
        if new_hash == local_hash {
            println!("Homebrew Database is up-to-date!");
        } else {
            println!(
                "MD5Hash: {} => {} \nUpdating database!",
                local_hash, new_hash
            );

            // Downloads the new database
            let database_response = ureq::get("https://api.pkg-zone.com/store.db")
                .set("User-Agent", &config.homebrew_token)
                .call()?;

            let mut file = File::create(&config.homebrew_database).expect("failed to create file");

            match std::io::copy(&mut database_response.into_reader(), &mut file) {
                Ok(_) => {
                    println!("Saved database in: \"{}\" Successfully!", &config.homebrew_database);
                }
                Err(err) => {
                    panic!("Aborting, error saving homebrew database: {}", err)
                }
            };
        }
    }
    Ok(())
}