use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

use crate::stage::Stage;
pub type Timestamp = u64;

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    pub product_id: String,
    pub image: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub brand_id: String,
    pub date: Timestamp,
    pub rating: u8,
    pub stakeholders: Vec<String>,
    pub stages: Vec<Stage>,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

impl Default for Product {
    fn default() -> Self {
        Self {
            product_id: "".to_string(),
            image: "".to_string(),
            title: "".to_string(),
            summary: "".to_string(),
            category: "".to_string(),
            brand_id: "".to_string(),
            date: 0,
            rating: 0,
            stakeholders: vec![],
            stages: vec![],
        }
    }
}

impl Product {
    pub fn new(
        product_id: String,
        image: String,
        title: String,
        summary: String,
        category: String,
        brand_id: String,
        date: Timestamp,
        rating: u8,
        stakeholders: Vec<String>,
        stages: Vec<Stage>,
    ) -> Self {
        Self {
            product_id,
            image,
            title,
            summary,
            category,
            brand_id,
            date,
            rating,
            stakeholders,
            stages,
        }
    }

    pub fn new_stages(
        &mut self,
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
    ) {
        let stage = Stage::new(
            stage_id,
            title,
            description,
            location,
            latitude,
            longitude,
            publisher,
            date,
            ingredients,
            climate,
            community,
            nature,
            waste,
            workers,
        );
        self.stages.push(stage);
    }

    pub fn fetch_stages(&self, start: u32, limit: u32) -> Vec<Stage> {
        let result = self
            .stages
            .iter()
            .skip(start as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        result
    }

    pub fn remove(&mut self, index: u64) -> Stage {
        let size = self.stages.len() as u64;
        assert!(size > 0 && index < size, "Stages is empty");
        self.stages.remove(index as usize)
    }
}
