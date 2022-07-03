use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
pub type Timestamp = u64;

#[near_bindgen]
#[derive(Debug, Clone,)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Stage {
    pub stage_id: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub publisher: String,
    pub date: Timestamp,
    pub ingredients: Vec<String>,
    pub climate: u8,
    pub community: u8,
    pub nature: u8,
    pub waste: u8,
    pub workers: u8,
}

impl Default for Stage {
    fn default() -> Self {
        Self {

            stage_id: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            location: "".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            publisher: "".to_string(),
            date: 0,
            ingredients: vec![],
            climate: 0,
            community: 0,
            nature: 0,
            waste: 0,
            workers: 0,

        }
    }
}

impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        self.stage_id == other.stage_id
    }
}

#[near_bindgen]
impl Stage {
    pub fn new(
        stage_id: String,
        title: String,
        description: String,
        location: String,
        latitude: f64,
        longitude: f64,
        publisher: String,
        date: Timestamp,
        ingredients: Vec<String>,
        climate: u8,
        community: u8,
        nature: u8,
        waste: u8,
        workers: u8,
    ) -> Stage {
        Self {
            stage_id,
            title,
            description,
            location,
            latitude,
            longitude,
            publisher,
            date,
            ingredients,
            community,
            climate,
            nature,
            waste,
            workers,
        }
    }
}
