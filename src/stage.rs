use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Stage {
    pub title: String,
    pub summary: String,
    pub location: String,
    pub date: u64,
    pub administrator: String,
    pub climate: f64,
    pub community: f64,
    pub nature: f64,
}

#[near_bindgen]
impl Stage {
    pub fn new(
        title: String,
        summary: String,
        location: String,
        date: u64,
        administrator: String,
        climate: f64,
        community: f64,
        nature: f64,
    ) -> Self {
        Self {
            title,
            summary,
            location,
            date,
            administrator,
            climate,
            community,
            nature,
        }
    }
}
