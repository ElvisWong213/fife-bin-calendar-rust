mod models;
mod bin_calendar;

use std::process::exit;
use std::fs;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::models::bin::{Bin};

#[macro_use] extern crate rocket;
static FILE_PATH: &str = "uprn.txt";

#[get("/")]
async fn index() -> Json<Option<Bin>> {
    let authorization = bin_calendar::get_authorization().await;
    let uprn: String = read_file();
    Json(bin_calendar::get_bin_calendar(&authorization, &uprn).await)
}

#[get("/test")]
async fn dump() -> Status {
    Status::NotAcceptable
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let uprn: String = setup().await;
    if uprn.is_empty() {
        exit(0)
    }
    let _rocket = rocket::build()
        .mount("/", routes![index, dump])
        .launch()
        .await?;
    Ok(())
}

async fn setup() -> String {
    let mut uprn: String = read_file();
    if uprn.is_empty() {
        uprn = get_user_uprn().await;
        if uprn.is_empty() {
            return String::new()
        } else {
            save_file(&uprn);
        }
    }
    return uprn
}

fn read_file() -> String {
    let content = fs::read_to_string(FILE_PATH);
    return content.unwrap_or_else(|_| {
        println!("DEBUG: File does not exist");
        String::new()
    })
}

fn save_file(uprn: &str) -> bool {
    return match fs::write(FILE_PATH, uprn) {
        Ok(_) => {
            println!("File saved");
            true
        },
        Err(_) => {
            println!("Fail save the file {:?}", FILE_PATH);
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