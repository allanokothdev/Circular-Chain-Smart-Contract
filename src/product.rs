use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

use crate::stage::Stage;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    brand: String,
    title: String,
    image: String,
    summary: String,
    category: String,
    esg_score: f64,
    administrator: String,
    stakeholders: Vec<String>,
    stages: Vec<Stage>,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            brand: "".to_string(),
            title: "".to_string(),
            image: "".to_string(),
            summary: "".to_string(),
            category: "".to_string(),
            esg_score: 0 as f64,
            administrator: "".to_string(),
            stakeholders: vec![],
            stages: vec![],
        }
    }
}

#[near_bindgen]
impl Product {
    pub fn new_product(
        brand: String,
        title: String,
        image: String,
        summary: String,
        category: String,
        esg_score: f64,
        administrator: String,
        stakeholders: Vec<String>,
    ) -> Self {
        Self { 
          brand,
          title,
          image,
          summary,
          category,
          esg_score,
          administrator,
          stakeholders,
          stages: vec![]
         }
    }

    pub fn add(
        &mut self,
        title: String,
        summary: String,
        location: String,
        date: u64,
        administrator: String,
        climate: f64,
        community: f64,
        nature: f64,
    ) {
        let stage: Stage = Stage::new(
            title,
            summary,
            location,
            date,
            administrator,
            climate,
            community,
            nature,
        );
        self.stages.push(stage);
    }

    pub fn show(&self, start: u32, limit: u32) -> Vec<Stage> {
        let result: Vec<Stage> = self
            .stages
            .iter()
            .skip(start as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        result
    }

    pub fn remove(&mut self, index: u64) -> Stage {
        let size: u64 = self.stages.len() as u64;
        assert!(size > 0 && index < size, "Chain is Empty!");
        self.stages.remove(index as usize)
    }
}
