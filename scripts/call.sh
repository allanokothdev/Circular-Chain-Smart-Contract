#!/bin/bash 

source ./scripts/setting.conf

# Add Stage to product list
near call $SUB_ACCOUNT add_stage '{"title": "Production", "summary": "Palm Oil processing plant stage in Sumatra, Indonesia", "location": "Sumatra, Indonesia", "climate": 3, "community": 4, "nature": 5, "product_id": "Nestle Cooking Oil", "product_brand_title": "Nestle", "product_image": "https://financialtribune.com/sites/default/files/field/image/17january/12_oil.jpg", "product_title": "Jardine Cooking Oil", "product_summary": "Nestle Cooking Oil from Palm Oil", "product_category": "Food"}' --accountId circularchain.allanokoth.testnet --amount 2

# Update Product ESG SCORE
near call $SUB_ACCOUNT update_esg_score '{"product_id": "Nestle Cooking Oil"}' --accountId circularchain.allanokoth.testnet

# Show product
near call $SUB_ACCOUNT read_product '{"product_id": "Nestle Cooking Oil"}' --accountId circularchain.allanokoth.testnet

# Show stages
near call $SUB_ACCOUNT read_stages '{"product_id": "Nestle Cooking Oil", "start": 0, "limit": 10}' --accountId circularchain.allanokoth.testnet

# Remove stage from product list
near call $SUB_ACCOUNT delete_stage '{"product_id": "Nestle Cooking Oil","id": 0}' --accountId circularchain.allanokoth.testnet
