use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use chrono::{DateTime, prelude::*, Timelike, Utc};
use hex::{encode, FromHex};
use md5::{Digest, Md5};
use regex::Regex;
use rusqlite::{Connection, params};
use serde_json::Value;

mod r#extern;

struct Issue<'lt> {
    id: u64,
    code: String,
    title: String,
    tag: &'lt str,
    issue_type: &'lt str,
    last_updated: String,
    image_exists: bool,
}

fn main() {
    println!("1/3 : Reading config :3");
    let mut issues: Vec<Issue> = Vec::new();

    let start_time = Instant::now();
    let minute: u32 = Utc::now().minute();

    let mut skipped_images: u64 = 0;
    let code_regex = Regex::new(r"[a-zA-Z]{4}[0-9]{5}").unwrap();

    let config_data: r#extern::Config = r#extern::config_creator();

    let api_url: &str = config_data.keys.compat_github_url.as_str();
    let github_url: &str = config_data.keys.github_url.as_str();
    let api_token: &str = config_data.keys.github_token.as_str();
    let hb_user_agent: &str = config_data.keys.homebrew_token.as_str();
    let game_user_agent: &str = config_data.keys.ps4_useragent.as_str();
    let tmdb_hash_bytes: Vec<u8> = Vec::from_hex(config_data.keys.tmdb_hash).expect("Invalid tmdb hash!");
    let workflow_url: &str = config_data.keys.workflow_url.as_str();

    let images_games_folder: &str = config_data.locations.games_folder.as_str();
    let images_hb_folder: &str = config_data.locations.homebrew_folder.as_str();
    let hb_db_path: &str = config_data.locations.homebrew_database.as_str();
    let db_path: &str = config_data.locations.database.as_str();
    let stats_folder: &str = config_data.locations.stats_folder.as_str();

    let tag_homebrew: u64 = config_data.ids.homebrew;
    let tag_playable: u64 = config_data.ids.playable;
    let tag_ingame: u64 = config_data.ids.ingame;
    let tag_intro: u64 = config_data.ids.intro;
    let tag_boots: u64 = config_data.ids.boots;
    let tag_nothing: u64 = config_data.ids.nothing;


    // Checks if homebrew database is up-to-date
    if !(4..=57).contains(&minute) || !Path::new(hb_db_path).exists() {
        match ureq::get("https://api.pkg-zone.com/api.php?db_check_hash=true")
            .set("User-Agent", hb_user_agent)
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
                let local_hash: String = match fs::read(hb_db_path) {
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
                        .set("User-Agent", hb_user_agent)
                        .call()
                    {
                        Ok(response) => {
                            let mut file = File::create(hb_db_path).expect("failed to create file");

                            match std::io::copy(&mut response.into_reader(), &mut file) {
                                Ok(_) => {
                                    println!("Saved database in: '{}' Successfully!", hb_db_path)
                                }
                                Err(err) => {
                                    panic!("Aborting, error saving homebrew database: {}", err)
                                }
                            };
                        }
                        Err(err) => {
                            if !Path::new(hb_db_path).exists() {
                                panic!("Error downloading database and couldn't find the file locally.");
                            }
                            eprintln!("Request failed: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                if !Path::new(hb_db_path).exists() {
                    panic!("Error getting database hash and couldn't find the file locally.");
                }
                eprintln!("Request failed: {}", err);
            }
        }
    }

    println!("2/3 : Starting Database Update :3");
    // Setup database connections and make sql tables
    let database_hb = Connection::open(hb_db_path).expect("Failed to open SQLite connection");
    let mut database = Connection::open(db_path).expect("Failed to open SQLite connection");

    {
        let statements = "
            CREATE TABLE IF NOT EXISTS issues (
              id INT(6) PRIMARY KEY,
              code VARCHAR(10),
              title VARCHAR(130),
              tag VARCHAR(20),
              type VARCHAR(6),
              updatedDate VARCHAR(12),
              image boolean DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS gameSkips (
              id INT(6) PRIMARY KEY
            );";

        database
            .execute_batch(statements)
            .expect("Failed to create database tables!");
    }

    let page_count: u64 = {
        let total_issues: u64 = github_request(api_url, api_token)
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
                api_url, page
            );
            // need to clone otherwise it is a reference *cat crying emoji*
            github_request(url.as_str(), api_token)
                .as_array()
                .cloned()
                .unwrap()
        };

        let page_time = Instant::now();

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
                    x if x.contains(&tag_playable) => tag = "Playable",
                    x if x.contains(&tag_ingame) => tag = "Ingame",
                    x if x.contains(&tag_intro) => tag = "Intro",
                    x if x.contains(&tag_boots) => tag = "Boots",
                    x if x.contains(&tag_nothing) => tag = "Nothing",
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
                    _x if tags.contains(&tag_homebrew) => {
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

            let game_path = format!("{}{}.avif", images_games_folder, &code);
            let hb_path = format!("{}{}.avif", images_hb_folder, &title);
            let mut image_exists: bool = false;

            // CHECK IF THE GAME IS ALREADY SAVED, IF NOT PROCEED
            if issue_type == "GAME" && !Path::new(&game_path).exists() {
                let mut stmt = database
                    .prepare("SELECT * FROM gameSkips WHERE id = (?1) COLLATE NOCASE")
                    .expect("Database query failed!");

                // CHECK IF THE ID IS IN THE SKIP TABLE
                if !stmt.exists(params![&id]).expect("Database query failed!") {
                    match r#extern::image_handler(r#extern::ImageTypes::Game(
                        game_user_agent,
                        game_path,
                        &code,
                        &tmdb_hash_bytes,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln!("Image download failed! {}", err);
                            skipped_images += 1;
                            let mut stmt = database
                                .prepare("INSERT INTO gameSkips (id) VALUES (?)")
                                .expect("Database query failed!");

                            stmt.execute([id])
                                .expect("error executing database insert for gameSkips");
                        }
                    }
                } else {
                    skipped_images += 1;
                }
            } else if issue_type == "HB" && !Path::new(&hb_path).exists() {
                let mut stmt = database
                    .prepare("SELECT * FROM gameSkips WHERE id = (?1) COLLATE NOCASE")
                    .expect("Database query failed!");

                // CHECK IF THE ID IS IN THE SKIP TABLE
                if !stmt.exists(params![&id]).expect("Database query failed!") {
                    match r#extern::image_handler(r#extern::ImageTypes::Hb(
                        hb_user_agent,
                        hb_path,
                        &database_hb,
                        &title,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln!("Image download failed! {}", err);
                            skipped_images += 1;
                            let mut stmt = database
                                .prepare("INSERT INTO gameSkips (id) VALUES (?)")
                                .expect("Database query failed!");

                            stmt.execute([id])
                                .expect("error executing database insert for gameSkips");
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
            issues.push(Issue {
                id,
                code,
                title,
                tag,
                issue_type,
                last_updated,
                image_exists,
            });
            // println!("Title in: {}μs", (Instant::now() - issue_time).as_micros());
        }
        println!(
            "Page {} in: {}ms",
            page,
            (Instant::now() - page_time).as_millis()
        );
    }

    println!("3/3 : Starting Database transfer :3");

    // put issues into database
    let new_issues_count: f64 = issues.len() as f64;

    let old_issues_count: f64 = database
        .query_row("SELECT COUNT(*) FROM issues", [], |row| row.get(0))
        .expect("error getting the old issues count");

    // remove 5% from old issue count
    if new_issues_count > (0.95 * old_issues_count) {
        database
            .execute("DELETE FROM issues", ())
            .expect("Error while moving database!");

        let transaction = database.transaction().unwrap();
        {
            let mut stmt = transaction
                .prepare("INSERT or IGNORE INTO issues (id, code, title, tag, type, updatedDate, image) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .expect("Database query failed!");

            for issue in issues {
                stmt.execute(params![
                    issue.id,
                    issue.code,
                    issue.title,
                    issue.tag,
                    issue.issue_type,
                    issue.last_updated,
                    issue.image_exists,
                ])
                    .expect("Database insertion failed!");
            }
        }
        transaction.commit().expect("Database insertion failed!");
    } else {
        eprintln!("Something is wrong with newIssues, skipping update.");
    }

    // create the stats.json
    match r#extern::stats_creator(stats_folder, api_token, github_url, api_url, workflow_url) {
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