use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub data: Vec<AddressData>
}

#[derive(Serialize, Deserialize)]
pub struct AddressData {
    pub value: String,
    pub label: String
}