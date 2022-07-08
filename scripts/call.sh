#!/bin/bash 

source ./scripts/setting.conf

# Create New Brand and add to Brands LookupMap
near call $SUB_ACCOUNT create_brand '{"brand_id": "Jardine Matheson", "image": "https://valueinvestasia.com/wp-content/uploads/2017/07/1200px-Jardine_Matheson_Holdings_logo.jpg", "title": "Jardine Matheson", "summary": "Astra is advancing its sustainability journey, combining our focus on communities with a focus on climate and the planet", "industry": "Agribusiness", "region":  "South East Asia"}' --accountId $MASTER_ACCOUNT --amount 1

# Read Brands LookupMap
# near call $SUB_ACCOUNT read_brands '{}' --accountId allanokoth.testnet

# Update Brand

# Delete Brand from Brands LookupMap

# Create New Product and add to Products Vector

# Read Products Vector

# Update Product

# Delete Product from Products Vector

# Create New Stage and add to Product's Stages variable
# near call add_car '{"image": "https://www.ccarprice.com/products/Toyota_RAV4_Hybrid_LE_2022.jpg", "name": "Toyota", "model": "RAV4", "mileage": 1000, "year": "2022", "price": 5000000}' --accountId yto.testnet --amount 1

# Read Stages Vector

# Update Stages

# Delete Stages from Product's Stages variable

# Cross contract call for save_name in circularchain.allanokoth.testnet
# near call $SUB_ACCOUNT xcc_counter '{"name": "Allan"}' --accountId yto.testnet --gas 140000000000000
