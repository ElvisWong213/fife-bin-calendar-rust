use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bin {
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub results_returned: String,
    pub tab_collections: Vec<TabCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct TabCollection {
    pub colour: String,
    pub date: String,
    #[serde(rename = "type")]
    pub tab_collection_type: String,
}