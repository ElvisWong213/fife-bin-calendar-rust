use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileDataResponse {
    #[serde(rename = "profileData")]
    pub profile_data: ProfileData
}

#[derive(Serialize, Deserialize)]
pub struct ProfileData {
    #[serde(rename = "property-UPRN")]
    pub property_uprn: String,
}