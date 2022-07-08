use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Brand {
    pub image: String,
    pub title: String,
    pub summary: String,
    pub industry: String,
    pub region: String,
    pub publisher: String,
}

impl PartialEq for Brand {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Default for Brand {
    fn default() -> Self {
        Self {
            image: "".to_string(),
            title: "".to_string(),
            summary: "".to_string(),
            industry: "".to_string(),
            region: "".to_string(),
            publisher: "".to_string(),
        }
    }
}

#[near_bindgen]
impl Brand {
    pub fn new(
        image: String,
        title: String,
        summary: String,
        industry: String,
        region: String,
        publisher: String,
        
    ) -> Self {
        Self {
            image,
            title,
            summary,
            industry,
            region,
            publisher,
        }
    }
}

