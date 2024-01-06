mod models;

use std::process::exit;
use reqwest::{Client, RequestBuilder};
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::models::address::Address;
use crate::models::bin::Bin;
use crate::models::profile_data_response::{ProfileDataResponse};

#[tokio::main]
async fn main() {
    let authorization = get_authorization().await;
    println!("authorization token: {:}", authorization);

    println!("Please enter your postcode: ");
    let mut postcode = String::new();
    std::io::stdin().read_line(&mut postcode).unwrap();

    let address_list = match get_address_list(&authorization, &postcode).await {
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
        for address in &address_list {
            println!("{:}. {:}", counter, address.label);
            counter += 1;
        }
        println!("e. Exit");
        println!("\n Please enter the number to select your address: ");
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

    let uprn = get_uprn(&authorization, &address_list[selected_number].value).await;
    if uprn.is_empty() {
        println!("Error: UPRN is empty");
        exit(0);
    }

    get_bin_calendar(&authorization, &uprn).await;
}

async fn get_bin_calendar(authorization: &str, uprn: &str) {
    let url: &str = "https://www.fife.gov.uk/api/custom?action=powersuite_bin_calendar_collections&actionedby=bin_calendar&loadform=true&access=citizen&locale=en";
    // let url: String = "127.0.0.1".to_string();

    let json = json!({
        "name": "bin_calendar",
        "data": {"uprn": uprn}
    });
    let res = match build_post_request(url, authorization, &json).send().await {
        Ok(response) => response,
        Err(_) => {
            println!("Invalid response");
            return;
        },
    };
    if res.status().is_success() {
        let bin = res.json::<Bin>().await.unwrap();
        println!("Bin Calendar");
        for collection in bin.data.tab_collections {
            println!("{:} - {:} - {:}", collection.colour, collection.date, collection.tab_collection_type)
        }
    } else {
        println!("Invalid response")
    }
}

async fn get_address_list(authorization: &str, postcode: &str) -> Option<Address> {
    let url: &str = "https://www.fife.gov.uk/api/widget?action=propertysearch&actionedby=ps_3SHSN93&loadform=true&access=citizen&locale=en";

    let json = json!({
        "name": "bin_calendar",
        "data": {"postcode": postcode}
    });
    let res = match build_post_request(url, authorization, &json).send().await {
        Ok(response) => response,
        Err(_) => {
            println!("Invalid response");
            return None;
        },
    };
    if res.status().is_success() {
        let addresses = res.json::<Address>().await.unwrap();
        // for address in &addresses.data {
        //     println!("{:} - {:}", address.value, address.label)
        // }
        return Option::from(addresses);
    } else {
        println!("Invalid response");
        return None
    }
}

async fn get_uprn(authorization: &str, objectid: &str) -> String {
    let url: String = format!("https://www.fife.gov.uk/api/getobjectdata?objecttype=property&objectid={}", objectid);
    let json = json!({});


    let res = match build_post_request(&url, authorization, &json).send().await {
        Ok(response) => response,
        Err(_) => {
            println!("Invalid response");
            return String::new();
        },
    };
    if res.status().is_success() {
        let profile_data_response = res.json::<ProfileDataResponse>().await.unwrap();
        // println!("{:}", profile_data_response.profile_data.property_uprn);
        return profile_data_response.profile_data.property_uprn;
    } else {
        println!("Invalid response");
        return String::new();
    }
}

fn build_post_request(url: &str, authorization: &str, json: &Value) -> RequestBuilder {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    return client.post(url).headers(headers).json(json);
}

async fn get_authorization() -> String {
    let client = Client::new();
    let url: &str = "https://www.fife.gov.uk/api/citizen?preview=false&locale=en";
    // let url: String = "127.0.0.1".to_string();
    let res = match client.get(url).send().await {
        Ok(response) => response,
        Err(_) => {
            println!("Invalid response");
            return String::new();
        }
    };
    if res.status().is_success() {
        match res.headers().get("Authorization") {
            None => "authorization token is empty".to_owned(),
            Some(x) => {
                let token = x.to_str().expect("Unable to get token");
                return token.to_owned()
            },
        }
    } else {
        println!("Invalid response");
        return String::new();
    }
}