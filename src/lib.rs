/*
 * This is a Simple Smart Contract that Traces Sustainability of Consumer Goods Supply Chain
 *      1. Brands
 *      2. Products
 *      3. Stages
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, ext_contract, near_bindgen, setup_alloc, Promise};

pub type AccountId = String;
pub type BrandId = String;
pub type ProductId = String;
pub type Timestamp = u64;

mod brand;
mod product;
mod stage;

use brand::Brand;
use product::Product;
use stage::Stage;

setup_alloc!();

const COUNTER_XCC_GAS: u64 = 7_000_000_000_000;
const COUNTER_DEPOSIT: u128 = 0;

const CIRCULARCHAINCONTRACT_XCC_GAS: u64 = 5_000_000_000_000;
const CIRCULARCHAINCONTRACT_DEPOSIT: u128 = 0;

#[ext_contract(ext_counter)]
pub trait Counter {
    fn save_name(&mut self, name: String) -> String;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn response_callback(&mut self) -> String;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CircularChain {
    pub admin_id: String,
    brands: LookupMap<String, Brand>,
    products: Vector<Product>,
}

impl Default for CircularChain {
    fn default() -> Self {
        Self {
            admin_id: env::predecessor_account_id(),
            brands: LookupMap::new(b"w".to_vec()),
            products: Vector::new(b"w"),
        }
    }
}

#[near_bindgen]
impl CircularChain {
    //#[payable]
    pub fn create_brand (
        &mut self,
        brand_id: String,
        image: String,
        title: String,
        summary: String,
        industry: String,
        region: String,
    ) {
        let signer = env::predecessor_account_id();
        let publisher = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();

        // Checks if the brands collection contains a Brand associ ated with a given BrandID
        if self.brands.get(&brand_id).is_none() {
            // If there is no such entry, we initialize a new Brand and insert the given Brand items and then push the Brand into the map
            let brand = Brand::new(
                image,
                title,
                summary,
                industry,
                region,
                publisher,
            );
            self.brands.insert(&brand_id, &brand);
            self.settle_storage_cost(initial_storage, deposit, &signer);

            env::log("Brand added successfully!".as_bytes());
        } else {
            // If the BrandID exists in the collection, we just retrieve the Brand, insert details and insert it back again.
            let mut brand = self.brands.get(&brand_id).unwrap();
            assert_eq!(
                brand.publisher, signer,
                "You do not have permission to edit this brand"
            );
            brand.image = image;
            brand.title = title;
            brand.summary = summary;
            brand.industry = industry;
            brand.region = region;
            self.brands.insert(&brand_id, &brand);
            self.settle_storage_cost(initial_storage, deposit, &signer);

            env::log("Brand updated successfully!".as_bytes());
        }
    }

    pub fn delete_brand(&mut self, brand_id: String) -> Option<Brand> {
        let signer = env::predecessor_account_id();
        let initial_storage = self.initial_storage();

        if let Some(brand) = self.brands.get(&brand_id) {
            assert_eq!(
                brand.publisher, signer,
                "You dont have permission to delete brand"
            );
            self.brands.remove(&brand_id);
            self.refund_storage_cost(initial_storage, &signer);
            env::log("Brands deleted successfully!".as_bytes());
            Some(brand)
        } else {
            //panic!("Brand does not exist for this Index");
            None
        }
    }

    pub fn read_brand(self, brand_id: String) -> Option<Brand> {
        // If the BrandID exists in the collection, we just retrieve the Brand, insert details and insert it back again.

        if let Some(brand) = self.brands.get(&brand_id) {
            env::log("Brand fetched successfully!".as_bytes());
            Some(brand)
        } else {
            //panic!("Brand does not exist for this Index");
            None
        }
    }

    pub fn read_brands(&self) -> Option<&LookupMap<String, Brand>> {
        let brands = &self.brands;
        env::log("Brands fetched successfully!".as_bytes());
        Some(brands)
    }

    //CRUD operations on Products
    pub fn create_product(
        &mut self,
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
    ) {
        let signer = env::predecessor_account_id();
        let publisher = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();

        let product = Product::new(
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
            publisher,
        );
        self.products.push(&product);
        self.settle_storage_cost(initial_storage, deposit, &signer);
        env::log("Product added successfully!".as_bytes());
    }

    pub fn read_products(&self) -> Vec<Product> {
        let products = &self.products;
        return products.to_vec();
    }

    pub fn read_product(self, index: u64) -> Option<Product> {
        if let Some(product) = self.products.get(index) {
            env::log("Product retrieved successfully!".as_bytes());
            Some(product)
        } else {
            env::log("Product not found!".as_bytes());
            None
        }
    }

    pub fn update_product(
        &mut self,
        index: u64,
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

    ) {
        let signer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();
        assert!(stakeholders.contains(&signer),"You do not have permission to publish to edit this Item");

        let mut product = self.products.get(index).unwrap();
        product.product_id = product_id;
        product.image = image;
        product.title = title;
        product.summary = summary;
        product.category = category;
        product.brand_id = brand_id;
        product.date = date;
        product.rating = rating;
        product.stakeholders = stakeholders;
        product.stages = stages;

        self.products.push(&product);
        self.settle_storage_cost(initial_storage, deposit, &signer);
        env::log("Product added successfully!".as_bytes());
    }

    pub fn delete_product(&mut self, index: u64) -> Option<Product> {
        let signer = env::predecessor_account_id();
        let initial_storage = self.initial_storage();

        if let Some(product) = self.products.get(index) {
            assert_eq!(product.publisher, signer,"You dont have permission to delete brand");
            self.products.swap_remove(index);
            self.refund_storage_cost(initial_storage, &signer);
            env::log("Product deleted successfully!".as_bytes());
            Some(product)
        } else {
            env::log("Product not found!".as_bytes());
            None
        }
    }

    pub fn create_stage(
        &mut self,
        index: u64,
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
        let signer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let initial_storage = self.initial_storage();

        if let Some(mut product) = self.products.get(index) {
            env::log("Product retrieved successfully!".as_bytes());
            assert!(product.stakeholders.contains(&signer), "You do not have permission to delete this Stage");
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
            product.stages.push(stage);
            self.products.push(&product);
            env::log("Stage added successfully!".as_bytes());
            self.settle_storage_cost(initial_storage, deposit, &signer);
        } else {
            env::log("Product not found!".as_bytes());
        }
    }

    pub fn read_stages(&self, index: u64, start: u32, limit: u32) -> Option<Vec<Stage>> {
        if let Some(product) = self.products.get(index) {
            env::log("Product retrieved successfully!".as_bytes());
            let stages = product.fetch_stages(start, limit);
            env::log("Stage fetched successfully!".as_bytes());
            Some(stages)
        } else {
            env::log("Product not found!".as_bytes());
            None
        }
    }

    pub fn delete_stages(&mut self, index: u64, product_id: u64) -> Option<Stage> {
        let signer = env::predecessor_account_id();
        let initial_storage = self.initial_storage();

        if let Some(mut product) = self.products.get(product_id) {
            env::log("Product retrieved successfully!".as_bytes());
            assert!(product.stakeholders.contains(&signer), "You do not have permission to delete this Stage");
            let removed_stage = product.remove(index);
            self.products.push(&product);
            env::log("Stage deleted successfully!".as_bytes());
            self.refund_storage_cost(initial_storage, &signer);
            Some(removed_stage)
        } else {
            env::log("Product not found!".as_bytes());
            None
        }
    }

    /**
     * ==============================================================================
     * STORAGE STAKING
     * ==============================================================================
     */

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

    pub fn xcc_counter(&self, name: String) -> Promise {
        ext_counter::save_name(
            name,
            &self.counter_account_id(),
            COUNTER_DEPOSIT,
            COUNTER_XCC_GAS,
        )
        .then(ext_self::response_callback(
            &env::current_account_id(),
            CIRCULARCHAINCONTRACT_DEPOSIT,
            CIRCULARCHAINCONTRACT_XCC_GAS,
        ))
    }

    //#[private]
    //pub fn response_callback(&mut self, #[callback] name: String) -> String {
    //  let mut result = String::from(name);
    //   result.push_str(" : Processed in circular chain smart contract");
    //   result
    //}

    fn counter_account_id(&self) -> String {
        "allanokoth.testnet".to_string()
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

    fn get_brand_params() -> (
        String,
        String,
        String,
        String,
        String,
        String,
    ) {
        let image: String = String::from("https://valueinvestasia.com/wp-content/uploads/2017/07/1200px-Jardine_Matheson_Holdings_logo.jpg");
        let title: String = String::from("Jardine Matheson");
        let summary: String = String::from("Astra is advancing its sustainability journey, combining our focus on communities with a focus on climate and the planet");
        let industry: String = String::from("Agribusiness");
        let region: String = String::from("South East Asia");
        let publisher: String = String::from("allanokoth.testnet");
        (
            image,
            title,
            summary,
            industry,
            region,
            publisher,
        )
    }

    fn get_stages_params() -> (
        String,
        String,
        String,
        String,
        f64,
        f64,
        String,
        Timestamp,
        Vec<String>,
        u8,
        u8,
        u8,
        u8,
        u8,
    ) {
        let stage_id: String = String::from("Jardine Matheson");
        let title: String = String::from("Jardine Matheson");
        let description: String = String::from("Astra is advancing its sustainability journey, combining our focus on communities with a focus on climate and the planet");
        let location: String = String::from("Sumatra, Indonesia");
        let latitude: f64 = 0.0;
        let longitude: f64 = 0.0;
        let publisher: String = String::from("allanokoth.testnet");
        let date: Timestamp = 0;
        let ingredients: Vec<String> = Vec::new();
        let climate: u8 = 5;
        let community: u8 = 5;
        let nature: u8 = 5;
        let waste: u8 = 4;
        let workers: u8 = 4;
        (
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
        )
    }

    fn get_products_params() -> (
        String,
        String,
        String,
        String,
        String,
        String,
        Timestamp,
        u8,
        Vec<String>,
        Vec<Stage>,
        String,
    ) {
        let product_id: String = String::from("Palm Oil 0164B");
        let image: String = String::from(
            "https://financialtribune.com/sites/default/files/field/image/17january/12_oil.jpg",
        );
        let title: String = String::from("Palm Oil 0164B");
        let summary: String = String::from("Palm Oil Production");
        let category: String = String::from("Food");
        let brand_id: String = String::from("Jardine Matheson");
        let date: Timestamp = 0;
        let rating: u8 = 5;
        let stakeholders: Vec<String> = vec!["allanokoth.testnet".to_string()];
        let stages: Vec<Stage> = Vec::new();
        let publisher: String = String::from("allanokoth.testnet");
        (
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
            publisher,
        )
    }

    #[test]
    fn add_read_brands() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_brand_params();

        //CREATE BRAND OPERATION TESTING
        contract.create_brand(
            params.1.clone(),
            params.0,
            params.1.clone(),
            params.2,
            params.3,
            params.4,
        );

        //READ BRAND OPERATION TESTING
        if let Some(brand) = contract.read_brand(params.1.clone()) {
            let test_params = get_brand_params().1;
            assert_eq!(test_params, brand.title, "Read Operation Successful");
            env::log("Brand Create & Read Test Successful!".as_bytes());
        } else {
            log(b"Error in the code");
        }
    }

    #[test]
    fn add_delete_brands() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_brand_params();

        //CREATE BRAND OPERATION TESTING
        contract.create_brand(
            params.1.clone(),
            params.0,
            params.1.clone(),
            params.2,
            params.3,
            params.4,
        );

        //DELETE BRAND OPERATION TESTING
        contract.delete_brand(params.1.clone());
        log(b"Brand Deletion Test Successful!");

        //READ BRANDS LOOKUP_MAP & CONFIRMING DELETE
        if let Some(brands) = contract.read_brands() {
            let test_params = get_brand_params();
            println!("Brand List: {:?}", brands.get(&test_params.1));
            log(b"Brand Deletion Test Confirmed!");
        } else {
            log(b"Error in the code");
        }
    }

    #[test]
    fn add_read_delete_products() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_products_params();

        //CREATE PRODUCT OPERATION
        contract.create_product(
            params.0, params.1, params.2, params.3, params.4, params.5, params.6, params.7,
            params.8, params.9);

        //READ PRODUCTS OPERATION
        let products = contract.read_products();
        assert_eq!(1, products.len());
        let test_params = get_products_params();
        assert_eq!(&products[0].product_id, &test_params.0);

        //DELETE PRODUCT OPERATION
        contract.delete_product(0);
    }

    #[test]
    fn add_read_delete_stage() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = CircularChain::default();
        let params = get_stages_params();

        contract.create_stage(
            0, params.0, params.1, params.2, params.3, params.4, params.5, params.6, params.7,
            params.8, params.9, params.10, params.11, params.12, params.13,
        );

        //READ STAGES
        if let Some(stages) = contract.read_stages(0, 0, 3) {
            assert_eq!(1, stages.len());
            let test_params = get_stages_params();
            assert_eq!(&stages[0].stage_id, &test_params.0);
        } else {
            log(b"Error reading stages");
        }

        // Remove functionality
        contract.delete_stages(0, 0);
        if let Some(stages) = contract.read_stages(0, 0, 3) {
            assert_eq!(0, stages.len());
        } else {
            log(b"Error reading stages");
        }
    }
}
