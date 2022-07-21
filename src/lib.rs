use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

mod product;
mod stage;

use product::Product;
use stage::Stage;

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CircularChain {
    products: LookupMap<String, Product>,
}

impl Default for CircularChain {
    fn default() -> Self {
        Self {
            products: LookupMap::new(b"w".to_vec()),
        }
    }
}

#[near_bindgen]
impl CircularChain {
    #[payable]
    pub fn add_stage(
        &mut self,
        title: String,
        summary: String,
        location: String,
        climate: f64,
        community: f64,
        nature: f64,

        product_id: String,
        product_brand_title: String,
        product_image: String,
        product_title: String,
        product_summary: String,
        product_category: String
    ) {
        let signer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();
        let product_esg_score = (climate + community + nature)/3.0;

        if let Some(mut product) = self.products.get(&product_id) {
            assert!(product.stakeholders.contains(&env::signer_account_id()), "You not authorized to contribute to this Supply Chain");
            product.add(
                title,
                summary,
                location,
                env::block_timestamp(),
                env::signer_account_id(),
                climate as f64,
                community as f64,
                nature as f64
            );
            self.products.insert(&product_id, &product);
            self.settle_storage_cost(initial_storage, deposit, &signer);
        } else {
            let stakeholders = vec![env::signer_account_id()];
            let mut product = Product::new_product(product_brand_title, product_title,product_image, product_summary, product_category, product_esg_score as f64, env::signer_account_id(), stakeholders);
            product.add(
                title,
                summary,
                location,
                env::block_timestamp(),
                env::signer_account_id(),
                climate as f64,
                community as f64,
                nature as f64
            );
            self.products.insert(&product_id, &product);
            self.update_esg_score(product_id);
            self.settle_storage_cost(initial_storage, deposit, &signer);
        }
    }

    pub fn update_esg_score(&mut self, product_id: String) {
        if let Some(mut product) = self.products.get(&product_id) {
            let mut esg_score_sum = 0.0;
            assert_eq!(env::signer_account_id(),product.administrator, "You do not have permission to update");
            for stage in &product.stages {
                esg_score_sum = esg_score_sum + (stage.climate + stage.community + stage.nature)/3.0;
            }
            let aggregate_esg = esg_score_sum/(product.stages.len()) as f64;
            product.esg_score = aggregate_esg;
            let _lotto = &self.products.insert(&product_id, &product);
        } else {
            println!("No product with id {}", product_id);
        }
    }

    pub fn read_product(&self, product_id: String) -> Option<Product> {
        if let Some(product) = self.products.get(&product_id) {
            Some(product)
        } else {
            None
        }
    }

    pub fn read_stages(&self, product_id: String, start: u32, limit: u32) -> Option<Vec<Stage>> {

        if let Some(product) = self.products.get(&product_id) {
            let stages: Vec<Stage> = product.show(start, limit);
            Some(stages)
        } else {
            Some(vec![])
        }
    }


    pub fn delete_stage(&mut self, product_id: String, id: u64) -> Option<Stage> {
        let signer = env::predecessor_account_id();
        let initial_storage = self.initial_storage();

        if let Some(mut product) = self.products.get(&product_id) {
            let removed_stage = product.remove(id);
            self.products.insert(&product_id, &product);
            self.refund_storage_cost(initial_storage, &signer);
            Some(removed_stage)
        } else {
            None
        }
    }

    fn initial_storage(&self) -> u64 {
        let initial_storage = env::storage_usage();
        initial_storage
    }

    fn settle_storage_cost(&self, initial_storage: u64, attached_deposit: u128, signer: &str) {
        let current_storage = env::storage_usage();
        let used_storage = current_storage - initial_storage;
        let storage_unit_price = env::storage_byte_cost();

        if let Some(payable_storage_cost) = storage_unit_price.checked_mul(used_storage.into()) {
            assert!(attached_deposit >= payable_storage_cost);

            let excess = attached_deposit - payable_storage_cost;
            self.refund_excess(excess, signer);
        } else {
            panic!("Error calculating storage cost");
        }
    }

    fn refund_storage_cost(&self, initial_storage: u64, signer: &str) {
        let current_storage = env::storage_usage();
        let storage_released = initial_storage - current_storage;
        let storage_unit_price = env::storage_byte_cost();

        if let Some(refundable_storage_cost) =
            storage_unit_price.checked_mul(storage_released.into())
        {
            self.refund_excess(refundable_storage_cost, signer);
        } else {
            panic!("Error calculating storage cost");
        }
    }

    fn refund_excess(&self, excess: u128, signer: &str) {
        if excess > 0 {
            Promise::new(signer.to_string()).transfer(excess);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::env::log;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 1000000000000000000000000,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn get_params() -> (String, String, String, f64, f64, f64, String, String, String, String, String, String) {
        let title: String = String::from("Production");
        let summary: String = String::from("Palm Oil processing plant stage in Sumatra, Indonesia");
        let location: String = String::from("Sumatra, Indonesia");
        let climate = 4.0;
        let community = 4.0;
        let nature= 4.0;

        let product_id = String::from("Nestle Cooking Oil");
        let product_brand_title: String = String::from("Nestle");
        let product_image: String = String::from("https://financialtribune.com/sites/default/files/field/image/17january/12_oil.jpg");
        let product_title: String = String::from("Nestle ");
        let product_summary: String = String::from("Nestle Cooking Oil from Palm Oil");
        let product_category: String = String::from("Food");
            
        (title, summary, location, climate, community, nature, product_id, product_brand_title, product_image, product_title, product_summary, product_category)
    }

    #[test]
    fn add_to_stage() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_params();

        contract.add_stage(params.0, params.1, params.2, params.3, params.4, params.5, params.6, params.7, params.8, params.9, params.10, params.11);

        let product_id = String::from("Nestle Cooking Oil");

        if let Some(stages) = contract.read_stages(product_id,0, 3) {
            assert_eq!(1, stages.len());
            let test_params = get_params();
            assert_eq!(&stages[0].title, &test_params.0);
        } else {
            log(b"Error in the code");
        }
        
    }

    #[test]
    fn remove_from_stages() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_params();

        contract.add_stage(params.0, params.1, params.2, params.3, params.4, params.5, params.6, params.7, params.8, params.9, params.10, params.11);

        let product_id = String::from("Nestle Cooking Oil");
        if let Some(stages) = contract.read_stages(product_id,0, 3) {
            assert_eq!(1, stages.len());
        } else {
            log(b"Error reading stages");
        }

        let id = String::from("Nestle Cooking Oil");

        // Remove functionality
        contract.delete_stage(id,0);

        let index = String::from("Nestle Cooking Oil");

        if let Some(stages) = contract.read_stages(index, 0, 3) {
            assert_eq!(0, stages.len());
        } else {
            log(b"Error reading stages");
        }
    }

    #[test]
    fn update_esg_score(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_params();

        contract.add_stage(params.0, params.1, params.2, params.3, params.4, params.5, params.6, params.7, params.8, params.9, params.10, params.11);

        let product_id = String::from("Nestle Cooking Oil");

        contract.update_esg_score(product_id);

        let index = String::from("Nestle Cooking Oil");
        if let Some(product) = contract.read_product(index) {
            assert_eq!(product.esg_score,4.0);
        } else {
            log(b"Error in the code");
        }


    }
}
