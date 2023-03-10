use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CenturyPerson {
    #[serde(rename = "century")]
    century: String, 
    #[serde(rename = "person")]
    pope: Pope,
    #[serde(rename = "current")]
    current: bool
}

#[derive(Debug, Deserialize, Serialize)]
struct Pope {
    #[serde(rename = "Pontiff Number")]
    pontiff_number: String,
    #[serde(rename = "Pontificate")]
    pontificate: String,
    #[serde(rename = "Personal Name")]
    personal_name: String,
    #[serde(rename = "Name in English")]
    name_in_english: String,
    #[serde(rename = "Name in Latin")]
    name_in_latin: String,
    #[serde(rename = "Date of Birth")]
    date_of_birth: String,
    #[serde(rename = "Birthplace")]
    birthplace: String,
    #[serde(rename = "Age at start of Papacy")]
    age_at_start_of_papacy: String,
    #[serde(rename = "Age at end of Papacy")]
    age_at_end_of_papacy: String
}