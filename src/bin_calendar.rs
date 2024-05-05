use crate::models::address::Address;
use crate::models::bin::{Bin, TabCollection};
use crate::models::profile_data_response::ProfileDataResponse;

use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};
use serde_json::{json, Value};
use chrono::{DateTime, Local};
use serde::{Serialize, Serializer};

pub fn serialize_dt<S>(dt: &Option<DateTime<Local>>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if let Some(dt) = dt {
        dt.format("%m/%d/%Y %H:%M")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

#[derive(Serialize, Clone)]
pub struct BinColor {
    pub colors: Vec<String>,
    #[serde(serialize_with = "serialize_dt")]
    pub update_date: Option<DateTime<Local>>,
    pub collect_date: String
}

impl BinColor {
    pub fn new() -> Self {
        BinColor {
            colors: vec![],
            update_date: Default::default(),
            collect_date: String::new(),
        }
    }
}

fn collect_date(date: String) -> String {
    let split_date: Vec<&str> = date.split(", ").collect();
    if split_date.len() != 3 {
        return String::new();
    }
    return split_date[1].to_string();
}

pub fn get_bin_color(tab_collections: Vec<TabCollection>) -> Option<BinColor> {
    if tab_collections.is_empty() {
        return None;
    }
    let mut bin_data: BinColor = BinColor::new();
    bin_data.update_date = Option::from(Local::now());
    let target_date: String = tab_collections[0].date.to_string();
    bin_data.collect_date = collect_date(target_date.clone());
    for tab_collection in tab_collections {
        if tab_collection.date == target_date {
            let color = tab_collection.colour;
            bin_data.colors.push(color);
        }
    }
    if bin_data.colors.is_empty() {
        return None;
    }
    return Option::from(bin_data);
}

pub async fn get_bin_calendar(authorization: &str, uprn: &str) -> Option<Bin> {
    let url: &str = "https://www.fife.gov.uk/api/custom?action=powersuite_bin_calendar_collections&actionedby=bin_calendar&loadform=true&access=citizen&locale=en";
    // let url: String = "127.0.0.1".to_string();

    let json = json!({
        "name": "bin_calendar",
        "data": {"uprn": uprn}
    });
    let res = match build_post_request(url, authorization, &json)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Invalid response");
            println!("{}", e);
            return None;
        }
    };
    if !res.status().is_success() {
        return None
    }
    let bin = res.json::<Bin>().await.unwrap();
    return Option::from(bin);
}

pub async fn get_address_list(authorization: &str, postcode: &str) -> Option<Address> {
    let url: &str = "https://www.fife.gov.uk/api/widget?action=propertysearch&actionedby=ps_3SHSN93&loadform=true&access=citizen&locale=en";

    let json = json!({
        "name": "bin_calendar",
        "data": {"postcode": postcode}
    });
    let res = match build_post_request(url, authorization, &json)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Invalid response");
            println!("{}", e);
            return None;
        }
    };
    return if res.status().is_success() {
        let addresses = res.json::<Address>().await.unwrap();
        // for address in &addresses.data {
        //     println!("{:} - {:}", address.value, address.label)
        // }
        Option::from(addresses)
    } else {
        println!("Invalid response");
        None
    };
}

pub async fn get_uprn(authorization: &str, objectid: &str) -> String {
    let url: String = format!(
        "https://www.fife.gov.uk/api/getobjectdata?objecttype=property&objectid={}",
        objectid
    );
    let json = json!({});

    let res = match build_post_request(&url, authorization, &json)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Invalid response");
            println!("{}", e);
            return String::new();
        }
    };
    return if res.status().is_success() {
        let profile_data_response = res.json::<ProfileDataResponse>().await.unwrap();
        // println!("{:}", profile_data_response.profile_data.property_uprn);
        profile_data_response.profile_data.property_uprn
    } else {
        println!("Invalid response");
        String::new()
    };
}

fn build_post_request(url: &str, authorization: &str, json: &Value) -> RequestBuilder {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    return client.post(url).headers(headers).json(json);
}

pub async fn get_authorization() -> String {
    let client = Client::new();
    let url: &str = "https://www.fife.gov.uk/api/citizen?preview=false&locale=en";
    // let url: String = "127.0.0.1".to_string();
    let res = match client.get(url).send().await {
        Ok(response) => response,
        Err(e) => {
            println!("Invalid response");
            println!("{}", e);
            return String::new();
        }
    };
    if res.status().is_success() {
        match res.headers().get("Authorization") {
            None => "authorization token is empty".to_owned(),
            Some(x) => {
                let token = x.to_str().expect("Unable to get token");
                return token.to_owned();
            }
        }
    } else {
        println!("Invalid response");
        return String::new();
    }
}
