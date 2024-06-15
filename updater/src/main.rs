use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use chrono::{DateTime, prelude::*, Timelike, Utc};
use hex::encode;
use md5::{Digest, Md5};
use regex::Regex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::{AppConfig, get_config};

mod utils;
mod config;

#[derive(Serialize, Deserialize)]
struct Issue {
    id: u64,
    code: String,
    title: String,
    tag: String,
    r#type: String,
    updated: String,
    image: bool,
}

fn main() {
    println!("1/4 : Reading config :3");
    let start_time = Instant::now();
    let minute: u32 = Utc::now().minute();

    let mut skipped_images: u64 = 0;
    let code_regex = Regex::new(r"[a-zA-Z]{4}[0-9]{5}").unwrap();

    let config: AppConfig = get_config();

    // Checks if homebrew database is up-to-date
    if !(4..=57).contains(&minute) || !Path::new(&config.homebrew_database).exists() {
        match ureq::get("https://api.pkg-zone.com/api.php?db_check_hash=true")
            .set("User-Agent", &config.homebrew_token)
            .call()
        {
            Ok(response) => {
                let new_hash: String = {
                    let body: Value = response.into_json().expect("Failed to parse JSON");

                    body.get("hash")
                        .and_then(Value::as_str)
                        .expect("Failed to parse JSON o.o!")
                        .to_string()
                };

                // this also checks if the file exists, if it doesn't the hash will not match and the database will be downloaded :3!
                let local_hash: String = match fs::read(&config.homebrew_database) {
                    Ok(file) => encode(Md5::digest(file)),
                    Err(err) => {
                        println!("Database not found: {}", err);
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
                    match ureq::get("https://api.pkg-zone.com/store.db")
                        .set("User-Agent", &config.homebrew_token)
                        .call()
                    {
                        Ok(response) => {
                            let mut file = File::create(&config.homebrew_database).expect("failed to create file");

                            match std::io::copy(&mut response.into_reader(), &mut file) {
                                Ok(_) => {
                                    println!("Saved database in: \"{}\" Successfully!", &config.homebrew_database)
                                }
                                Err(err) => {
                                    panic!("Aborting, error saving homebrew database: {}", err)
                                }
                            };
                        }
                        Err(err) => {
                            if !Path::new(&config.homebrew_database).exists() {
                                panic!("Error downloading database and couldn't find the file locally.");
                            }
                            eprintln!("Request failed: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                if !Path::new(&config.homebrew_database).exists() {
                    panic!("Error getting database hash and couldn't find the file locally.");
                }
                eprintln!("Request failed: {}", err);
            }
        }
    }

    println!("2/4 : Initializing Databases :3");

    // Setup databases
    let database_hb = Connection::open(&config.homebrew_database).expect("Failed to open SQLite connection");

    let mut new_issues: Vec<Issue> = Vec::new();

    let old_issues: Vec<Issue> = {
        let json_string: String = fs::read_to_string("../database.json").unwrap_or_default();

        serde_json::from_str(&json_string).unwrap_or_default()
    };


    let mut game_skips: Vec<u64> = {
        let file = Path::new("./game_skips.json");

        if file.exists() {
            let json_string = fs::read_to_string("./game_skips.json").unwrap_or_default();

            serde_json::from_str(&json_string).unwrap_or_default()
        } else {
            Vec::new()
        }
    };

    // println!("{:?}", &old_issues);

    println!("3/4 : Starting Database Update :3");

    let page_count: u64 = {
        let total_issues: u64 = github_request(&config.api_url, &config.api_token)
            .get("open_issues_count")
            .and_then(Value::as_u64)
            .expect("Failed to parse JSON o.o!");

        let total_pages: u64 = total_issues.div_ceil(100);

        println!("Total Issues: {}", total_issues);
        println!("Total Pages: {}", total_pages);
        total_pages
    };


    for page in 1..=page_count {
        let github_issues = {
            let url = format!(
                "{}/issues?page={}&per_page=100&state=open&direction=ASC",
                &config.api_url, page
            );

            // need to clone otherwise it is a reference *cat crying emoji*
            github_request(url.as_str(), &config.api_token)
                .as_array()
                .cloned()
                .unwrap()
        };

        // let page_time = Instant::now();

        for issue in github_issues {
            // let issue_time = Instant::now();

            let id: u64 = issue
                .get("number")
                .and_then(Value::as_u64)
                .expect("Failed to parse JSON o.o!");

            let mut title = issue
                .get("title")
                .and_then(Value::as_str)
                .expect("Failed to parse JSON o.o!")
                .to_string();

            let last_updated = {
                let github_time = issue
                    .get("updated_at")
                    .and_then(Value::as_str)
                    .expect("Failed to parse JSON o.o!");

                let parsed_time =
                    DateTime::parse_from_rfc3339(github_time).expect("Error parsing update time");

                DateTime::<Local>::from_naive_utc_and_offset(
                    parsed_time.naive_utc(),
                    *Local::now().offset(),
                )
                    .format("%d/%m/%Y")
                    .to_string()
            };

            let tag: &str;
            let issue_type: &str;
            let code: String;

            {
                // get the best tag
                let tags = {
                    let array = issue
                        .get("labels")
                        .and_then(Value::as_array)
                        .expect("Failed to parse JSON o.o!");

                    array
                        .iter()
                        .map(|label| label.get("id").unwrap().as_u64().unwrap().to_owned())
                        .collect::<Vec<_>>()
                };

                match &tags {
                    x if x.contains(&config.tag_playable) => tag = "Playable",
                    x if x.contains(&config.tag_ingame) => tag = "InGame",
                    x if x.contains(&config.tag_intro) => tag = "Intro",
                    x if x.contains(&config.tag_boots) => tag = "Boots",
                    x if x.contains(&config.tag_nothing) => tag = "Nothing",
                    _ => tag = "N/A",
                }

                let mut temp_code = code_regex
                    // get the last one O.O
                    .find_iter(&title)
                    .last()
                    .map(|x| x.as_str().to_uppercase())
                    .unwrap_or_default();

                title = title
                    .replace(&temp_code, "")
                    .trim()
                    .trim_end_matches('-')
                    .trim()
                    .to_string();

                match temp_code.as_str() {
                    x if x.starts_with("CUSA") => issue_type = "GAME",
                    x if x.starts_with("PCJS") => issue_type = "GAME",
                    x if x.starts_with("PLJM") => issue_type = "GAME",
                    x if x.starts_with("PLJS") => issue_type = "GAME",
                    _x if tags.contains(&config.tag_homebrew) => {
                        issue_type = "HB";
                        if temp_code.is_empty() {
                            temp_code = "HomeBrew".to_string()
                        }
                    }
                    _ => {
                        eprintln!("skipped: {:?} + {:?}", title, temp_code);
                        continue;
                    }
                }
                code = temp_code
            }

            let game_path = format!("{}{}.avif", &config.games_folder, &code);
            let hb_path = format!("{}{}.avif", &config.homebrew_folder, &title);
            let mut image_exists: bool = false;

            // CHECK IF THE GAME IS ALREADY SAVED, IF NOT PROCEED
            if issue_type == "GAME" && !Path::new(&game_path).exists() {
                // todo
                // let mut stmt = database
                //     .prepare("SELECT * FROM gameSkips WHERE id = (?1) COLLATE NOCASE")
                //     .expect("Database query failed!");

                // CHECK IF THE ID IS IN THE SKIP TABLE
                // if !stmt.exists(params![&id]).expect("Database query failed!") {
                if game_skips.contains(&id) {
                    match utils::image_handler(utils::ImageTypes::Game(
                        &config.ps4_useragent,
                        game_path,
                        &code,
                        &config.tmdb_hash,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln!("Image download failed! {}", err);
                            skipped_images += 1;
                            // let mut stmt = database
                            //     .prepare("INSERT INTO gameSkips (id) VALUES (?)")
                            //     .expect("Database query failed!");


                            // stmt.execute([id])
                            //     .expect("error executing database insert for gameSkips");

                            game_skips.push(id);
                        }
                    }
                } else {
                    skipped_images += 1;
                }
            } else if issue_type == "HB" && !Path::new(&hb_path).exists() {
                // let mut stmt = database
                //     .prepare("SELECT * FROM gameSkips WHERE id = (?1) COLLATE NOCASE")
                //     .expect("Database query failed!");

                // CHECK IF THE ID IS IN THE SKIP TABLE
                // if !stmt.exists(params![&id]).expect("Database query failed!") {
                if game_skips.contains(&id) {
                    match utils::image_handler(utils::ImageTypes::Hb(
                        &config.homebrew_token,
                        hb_path,
                        &database_hb,
                        &title,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln!("Image download failed! {}", err);
                            skipped_images += 1;

                            game_skips.push(id);
                        }
                    }
                } else {
                    skipped_images += 1;
                }
            } else if issue_type == "GAME" || issue_type == "HB" {
                // meaning image exists
                image_exists = true;
            }

            // println!(
            //     "{} : {} [{}] - {}, {}   {}",
            //     &id, &title, &code, &issue_type, &tag, &last_updated
            // );
            new_issues.push(Issue {
                id,
                code,
                title,
                tag: tag.to_string(),
                r#type: issue_type.to_string(),
                updated: last_updated,
                image: image_exists,
            });
            // println!("Title in: {}μs", (Instant::now() - issue_time).as_micros());
        }
        // println!(
        //     "Page {} in: {}ms",
        //     page,
        //     (Instant::now() - page_time).as_millis()
        // );
    }

    println!("3/3 : Starting Database transfer :3");

    // put issues into database
    let new_issues_count: f64 = new_issues.len() as f64;


    // todo get current issues.json
    let old_issues_count: f64 = old_issues.len() as f64;

    // remove 5% from old issue count
    if new_issues_count > (0.95 * old_issues_count) {
        let issues_json = serde_json::to_string(&new_issues).expect("Uh oh! Error creating the json string!");
        fs::write(format!("./database.json"), issues_json).expect("Error creating Json!");


        // database
        //     .execute("DELETE FROM issues", ())
        //     .expect("Error while moving database!");
        //
        // let transaction = database.transaction().unwrap();
        // {
        //     let mut stmt = transaction
        //         .prepare("INSERT or IGNORE INTO issues (id, code, title, tag, type, updatedDate, image) VALUES (?, ?, ?, ?, ?, ?, ?)")
        //         .expect("Database query failed!");
        //
        //     for issue in issues {
        //         stmt.execute(params![
        //             issue.id,
        //             issue.code,
        //             issue.title,
        //             issue.tag,
        //             issue.issue_type,
        //             issue.last_updated,
        //             issue.image_exists,
        //         ])
        //             .expect("Database insertion failed!");
        //     }
        // }
        // transaction.commit().expect("Database insertion failed!");
    } else {
        eprintln!("Something is wrong with newIssues, skipping update.");
    }

    // create the stats.json
    match utils::stats_creator(&config.stats_folder, &config.api_token, &config.github_url, &config.api_url, &config.workflow_url) {
        Ok(_) => (),
        Err(err) => eprintln!("Error creating stats.json! {}", err),
    }

    // Calculate the elapsed time
    println!(
        "Completed update in: {} s",
        (Instant::now() - start_time).as_secs_f32()
    );
    println!("Failed images: {}", skipped_images);
}

fn github_request(url: &str, token: &str) -> Value {
    match ureq::get(url)
        .set("User-Agent", "obliteration.net")
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", &format!("Bearer {}", token))
        .call()
    {
        Ok(response) => response.into_json().expect("Failed to parse JSON"),
        Err(response) => panic!("Github request failed: {}", response),
    }
}