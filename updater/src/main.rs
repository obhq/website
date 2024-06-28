use std::fs;
use std::path::Path;
use std::time::Instant;

use regex::Regex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::config_creator;
use crate::utils::homebrew_database_updater;

mod utils;
mod config;
mod macros;

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
    println_green!("1/4 : Reading Config :3");
    let start_time = Instant::now();

    let mut failed_images: u64 = 0;
    let code_regex = Regex::new(r"[a-zA-Z]{4}[0-9]{5}").unwrap();

    let (config, secrets) = match config_creator() {
        Ok(data) => data,
        Err(err) => panic_red!("Error getting config data, Aborting! || Err: {}", err.root_cause()),
    };

    println_green!("2/4 : Initializing Databases :3");

    homebrew_database_updater(&config, &secrets).unwrap_or_else(|err| {
        if !Path::new(&config.homebrew_database).exists() {
            panic_red!("Error downloading database and couldn't find the file locally. || Error: {}", err.root_cause());
        } else {
            eprintln_red!("Couldnt update the homebrew database, however a local copy was found, continuing. ||  Error: {}", err.root_cause());
        }
    });

    // Setup databases
    let database_hb = Connection::open(&config.homebrew_database).expect("Failed to open SQLite connection to homebrew database!");

    let mut new_issues: Vec<Issue> = Vec::new();

    let old_issues: Vec<Issue> = {
        let json_string: String = fs::read_to_string(&config.database).unwrap_or_default();

        serde_json::from_str(&json_string).unwrap_or_default()
    };

    let mut game_skips: Vec<u64> = {
        let file = Path::new(&config.game_skips_database);

        if file.exists() {
            let json_string = fs::read_to_string(&config.game_skips_database).unwrap_or_default();

            serde_json::from_str(&json_string).unwrap_or_default()
        } else {
            Vec::new()
        }
    };

    println_green!("3/4 : Starting Main Database Update :3");

    // gets the total pages
    let page_count: u64 = {
        let total_issues: u64 = github_request(&config.compat_api_url, &secrets.github_api_token)
            .get("open_issues_count")
            .and_then(Value::as_u64)
            .expect("Failed to parse JSON o.o!");

        let total_pages: u64 = total_issues.div_ceil(100);

        println_cyan!("Total Issues: {}", total_issues);
        println_cyan!("Total Pages: {}", total_pages);
        total_pages
    };


    for page in 1..=page_count {
        let github_issues = {
            let url = format!(
                "{}/issues?page={}&per_page=100&state=open&direction=DESC",
                &config.compat_api_url, page
            );

            // need to clone otherwise it is a reference -.-
            github_request(url.as_str(), &secrets.github_api_token)
                .as_array()
                .cloned()
                .unwrap()
        };

        let page_time = Instant::now();

        for issue in github_issues {
            let id: u64 = issue
                .get("number")
                .and_then(Value::as_u64)
                .expect("Failed to parse JSON o.o!");

            let mut title: String = issue
                .get("title")
                .and_then(Value::as_str)
                .expect("Failed to parse JSON o.o!")
                .to_string();

            // RFC3339 String
            let last_updated: String = issue
                .get("updated_at")
                .and_then(Value::as_str)
                .expect("Failed to parse JSON o.o!")
                .to_string();

            // get the best tag
            let status_tag: String = {
                let tag_array: Vec<u64> = issue
                    .get("labels")
                    .and_then(Value::as_array)
                    .unwrap_or(&vec![])

                    .iter()
                    .map(|label| label.get("id").and_then(Value::as_u64).unwrap_or_default())
                    .collect::<Vec<u64>>();

                match tag_array {
                    x if x.contains(&config.tag_playable) => "Playable".to_string(),
                    x if x.contains(&config.tag_ingame) => "InGame".to_string(),
                    x if x.contains(&config.tag_intro) => "Intro".to_string(),
                    x if x.contains(&config.tag_boots) => "Boots".to_string(),
                    x if x.contains(&config.tag_nothing) => "Nothing".to_string(),
                    _ => "N/A".to_string(),
                }
            };

            let mut code: String = {
                let temp_code: String = code_regex
                    // get the last one O.O
                    .find_iter(&title)
                    .last()
                    .map(|x| x.as_str().to_uppercase())
                    .unwrap_or_default();

                // cleanup title
                title = title
                    .replace(&temp_code, "")
                    .trim()
                    .trim_end_matches('-')
                    .trim()
                    .to_string();

                temp_code
            };

            let issue_type: String = {
                // sadly need to get this again, otherwise I cant confirm if it's a homebrew
                let tag_array: Vec<u64> = issue
                    .get("labels")
                    .and_then(Value::as_array)
                    .unwrap_or(&vec![])

                    .iter()
                    .map(|label| label.get("id").and_then(Value::as_u64).unwrap_or_default())
                    .collect();

                // check if it's a normal game
                if code.starts_with("CUSA") || code.starts_with("PCJS") || code.starts_with("PLJM") || code.starts_with("PLJS") {
                    "GAME".to_string()

                    // check if it's a homebrew
                } else if tag_array.contains(&config.tag_homebrew) {
                    if code.is_empty() {
                        code = "HomeBrew".to_string();
                    }

                    "HB".to_string()

                    // the rest will not be processed further
                } else {
                    eprintln_red!("Skipped: {:?} + {:?}", title, code);
                    continue;
                }
            };

            let image_path = match issue_type.as_str() {
                "GAME" => format!("{}{}.avif", &config.game_images_folder, &code),
                "HB" => format!("{}{}.avif", &config.homebrew_images_folder, &title),
                _ => {
                    eprintln_red!("Skipped: {:?} + {:?}", title, code);
                    continue;
                }
            };

            let mut image_exists: bool = false;

            // CHECK IF THE GAME IS ALREADY SAVED, IF NOT PROCEED
            if issue_type == "GAME" && !Path::new(&image_path).exists() {

                // check if the id is in the skip table
                if !game_skips.contains(&id) {
                    match utils::image_handler(utils::ImageTypes::Game(
                        &config.ps4_useragent,
                        image_path,
                        &code,
                        &secrets.tmdb_hex,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln_red!("Image Download Failed! || {}", err.root_cause());
                            failed_images += 1;

                            game_skips.push(id);
                        }
                    }
                } else {
                    failed_images += 1;
                }
            } else if issue_type == "HB" && !Path::new(&image_path).exists() {

                // check if the id is in the skip table
                if !game_skips.contains(&id) {
                    match utils::image_handler(utils::ImageTypes::Hb(
                        &secrets.homebrew_api_token,
                        image_path,
                        &database_hb,
                        &title,
                    )) {
                        Ok(_) => image_exists = true,
                        Err(err) => {
                            eprintln_red!("Image Download Failed! || {}", err.root_cause());
                            failed_images += 1;

                            game_skips.push(id);
                        }
                    }
                } else {
                    failed_images += 1;
                }
            } else if Path::new(&image_path).exists() {
                image_exists = true;
            }

            // put issue into new_issues
            new_issues.push(Issue {
                id,
                code,
                title,
                tag: status_tag.to_string(),
                r#type: issue_type.to_string(),
                updated: last_updated,
                image: image_exists,
            });
        }
        println_cyan!(
            "Page {} in: {}ms",
            page,
            (Instant::now() - page_time).as_millis()
        );
    }


    println_green!("3/3 : Starting Database Transfer :3");

    // save the game_skips.json
    let game_skips_json = serde_json::to_string(&game_skips).expect("Uh oh! Error creating the json string!");
    fs::write(&config.game_skips_database, game_skips_json).expect("Error saving the \"game_skips_database\" file!");

    let new_issues_count: f64 = new_issues.len() as f64;
    let old_issues_count: f64 = old_issues.len() as f64;

    println_cyan!("New issues count: {} || Old issues count: {}", new_issues_count, old_issues_count);

    // remove 5% from old_issues count and see if the new_issues is bigger. this ensures that new_issues isn't fucked somehow
    if new_issues_count > (0.95 * old_issues_count) {
        // save the database
        println_cyan!("Continuing update!");

        let issues_json = serde_json::to_string(&new_issues).expect("Uh oh! Error creating the json database string!");
        fs::write(&config.database, issues_json).expect("Error saving the \"database\" file!");
    } else {
        eprintln_red!("Something is wrong with newIssues, skipping update!\n If a lot of issues got removed from the compatibility page then ignore this error and just remove the \"{}\" file!", &config.database);
    }

    // create the stats.json
    match utils::stats_creator(&config.stats_file, &secrets.github_api_token, &config.main_api_url, &config.compat_api_url, &config.workflow_url) {
        Ok(_) => (),
        Err(err) => eprintln_red!("Error creating the \"{}\" file! {}", &config.stats_file, err),
    }


    // Calculate the elapsed time
    println_green!(
        "Completed update in: {} s",
        (Instant::now() - start_time).as_secs_f32()
    );

    println_cyan!("Failed images: {}", failed_images);
}

fn github_request(url: &str, token: &str) -> Value {
    match ureq::get(url)
        .set("User-Agent", "obliteration.net")
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", &format!("Bearer {}", token))
        .call()
    {
        Ok(response) => response.into_json().expect("Failed to parse JSON"),
        Err(response) => panic_red!("Github request failed: {}", response),
    }
}