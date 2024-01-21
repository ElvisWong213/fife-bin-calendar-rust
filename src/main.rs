mod models;
mod bin_calendar;

use std::process::exit;
use std::fs;
use tokio::sync::Mutex;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use chrono::Local;

use crate::bin_calendar::{BinColor, get_bin_color};
use crate::models::bin::{Bin};

#[macro_use] extern crate rocket;
const UPRN_FILE_PATH: &str = "uprn.txt";

struct Config {
    uprn: String,
    bin_color: Mutex<BinColor>
}

impl Config {
    fn new() -> Self{
        Config {
            uprn: String::new(),
            bin_color: Mutex::from(BinColor::new()),
        }
    }
}

#[get("/")]
async fn index(config: &State<Config>) -> Json<Option<BinColor>> {
    let mut bin_color = config.bin_color.lock().await;
    let duration: i64 = match bin_color.update_date {
        None => 12,
        Some(target_time) => {
            Local::now().signed_duration_since(target_time).num_hours()
        }
    };
    let mut color: Option<BinColor> = None;
    if duration >= 12 {
        let authorization: String = bin_calendar::get_authorization().await;
        let bin_calendar: Option<Bin> = bin_calendar::get_bin_calendar(&authorization, &config.uprn).await;
        color = get_bin_color(bin_calendar.unwrap().data.tab_collections);
        *bin_color = match Clone::clone(&color) {
            None => {
                return Json(None)
            },
            Some(data) => data
        };
    }
    Json(color)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut config: Config = Config::new();
    config.uprn = setup().await;
    if config.uprn.is_empty() {
        exit(0)
    }
    let _rocket = rocket::build()
        .manage(config)
        .mount("/", routes![index, dump])
        .launch()
        .await?;
    Ok(())
}

async fn setup() -> String {
    let mut uprn: String = read_file(UPRN_FILE_PATH);
    if uprn.is_empty() {
        uprn = get_user_uprn().await;
        if uprn.is_empty() {
            return String::new()
        } else {
            save_file(&uprn, UPRN_FILE_PATH);
        }
    }
    return uprn
}

fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path);
    return content.unwrap_or_else(|_| {
        println!("DEBUG: File does not exist");
        String::new()
    })
}

fn save_file(uprn: &str, file_path: &str) -> bool {
    return match fs::write(file_path, uprn) {
        Ok(_) => {
            println!("File saved");
            true
        },
        Err(_) => {
            println!("Fail save the file {:?}", UPRN_FILE_PATH);
            false
        },
    }
}

async fn get_user_uprn() -> String {
    let authorization = bin_calendar::get_authorization().await;
    // println!("authorization token: {:}", authorization);

    println!("Please enter your postcode: ");
    let mut postcode = String::new();
    std::io::stdin().read_line(&mut postcode).unwrap();

    let address_list = match bin_calendar::get_address_list(&authorization, &postcode).await {
        None => {
            println!("Error: Unable to get addresses");
            exit(0);
        }
        Some(addresses) => {
            let data = addresses.data;
            if data.is_empty() {
                println!("Error: Unable to get addresses");
                exit(0);
            }
            data
        }
    };

    let mut selected_number: usize;
    loop {
        let mut counter: u16 = 1;
        println!();
        for address in &address_list {
            println!("{:}. {:}", counter, address.label);
            counter += 1;
        }
        println!("e. Exit");
        println!("\nPlease enter the number to select your address: ");
        let mut selected_address = String::new();
        std::io::stdin().read_line(&mut selected_address).unwrap();
        if selected_address.trim() == "e" {
            exit(0)
        } else {
            selected_number = match selected_address.trim().parse::<usize>() {
                Ok(value) => value - 1,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if selected_number < address_list.len() {
                break;
            } else {
                println!("Invalid number");
                continue;
            }
        }
    }

    return bin_calendar::get_uprn(&authorization, &address_list[selected_number].value).await;
}