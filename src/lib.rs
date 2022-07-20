// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

mod product;
mod stage;

use product::Product;
use stage::Stage;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CircularChain {
    products: LookupMap<String, Product>,
}

impl Default for Wishlist {
    fn default() -> Self {
        Self {
            products: LookupMap::new(b"w".to_vec()),
        }
    }
}

#[near_bindgen]
impl Wishlist {
    #[payable]
    pub fn add_stage(
        &mut self,
        title: String,
        summary: String,
        location: String,
        climate: u64,
        community: f64,
        nature: f64,

        product_brand_title: String,
        product_image: String,
        product_title: String,
        product_summary: String,
        product_category: String,
        product_esg_score: f64,
    ) {
        let signer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();

        if let Some(mut product) = self.products.get(&signer) {
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
            self.products.insert(&signer, &product);
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
            self.products.insert(&signer, &product);
            self.settle_storage_cost(initial_storage, deposit, &signer);
        }
    }

    pub fn read_wishlist(&self, start: u32, limit: u32) -> Option<Vec<Stage>> {
        let signer = env::predecessor_account_id();

        if let Some(product) = self.products.get(&signer) {
            let stages: Vec<Stage> = product.show(start, limit);
            Some(stages)
        } else {
            Some(vec![])
        }
    }

    pub fn delete_car(&mut self, id: u64) -> Option<Stage> {
        let signer = env::predecessor_account_id();
        let initial_storage = self.initial_storage();

        if let Some(mut product) = self.products.get(&signer) {
            let removed_vehicle = product.remove(id);
            self.products.insert(&signer, &product);
            self.refund_storage_cost(initial_storage, &signer);
            Some(removed_vehicle)
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

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
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

    fn get_params() -> (String, String, String, u64, String, u64) {
        let image: String =
            String::from("https://www.ccarprice.com/products/Toyota_RAV4_Hybrid_LE_2022.jpg");
        let name: String = String::from("Toyota");
        let model: String = String::from("RAV4");
        let mileage: u64 = 10000;
        let year: String = String::from("2022");
        let price: u64 = 10000000;
        (image, name, model, mileage, year, price)
    }
}
