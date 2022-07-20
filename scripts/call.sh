#!/bin/bash 

source ./scripts/setting.conf

# Add Stage to product list
near call $SUB_ACCOUNT add_stage '{"title": "Allan", "summary": "Allan Okoth", "location": "Kenya", "climate": 5, "community": 5, "nature": 5, "product_brand_title": "Jardine Matheson", "product_image": "https://financialtribune.com/sites/default/files/field/image/17january/12_oil.jpg", "product_title": "Jardine Cooking Oil", "product_summary": "Jardine Cooking Oil from Palm Oil", "product_category": "Food", "product_esg_score": 0}' --accountId circularchain.allanokoth.testnet --amount 2

# Show product list content
#near call $SUB_ACCOUNT read_wishlist '{"start": 0, "limit": 10}' --accountId allanokoth.testnet

# Remove stage from product list
#near call $SUB_ACCOUNT delete_car '{"id": 0}' --accountId allanokoth.testnet
