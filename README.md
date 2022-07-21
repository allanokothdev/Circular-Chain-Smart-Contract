# Circular-Chain-Smart-Contract

Loom Link: https://www.loom.com/share/3caab1b14981498d9017f5b1ee84a2c8

The continued effect of Climate Change has ushered in a new brand of Consumers who are climate-conscious. They want to know the impact of the product on the environment, the community as well as climate before they can make a purchase. To attract these consumers, Brands have gone to label their products as eco-friendly, sustainably-produced. Some of these claims are true, but for some are simply greenwashing to fool consumers into buying their products. Unfortunately for consumers, there’s no way to tell the difference between sustainably-produced products and fake ones. 
Ladies and Gentlemen, I present to you Circular Chain, A Blockchain-based platform that promotes supply chain transparency and traceability by aggregating data on respective product impact on the climate, environment as well as community, to come up with a sustainability score. Climate-conscious consumers can simply check and see the product impact on  the climate before purchasing. This way, Greenwashing stops with Circular Chain.
# Future Developments:
Create Frontend for the Dapp, partner with brands in Consumer Goods space.
Consumers will scan the product’s barcode and get the sustainability score of the respective product before making the purchase.


# Code Overview

Objects:

Defining Product Object:
The Circular Chain Smart contract will store product objects in a LookupMap.
In order to properly display product information to a user: This includes 
{Product Image, Title, Summary, Category, (esg) Sustainability Score,
as well as a vector containing the different stages along the supply chain of the respective product}.
![product_struct](https://user-images.githubusercontent.com/19547628/180221114-ebcd1db1-0a48-4b6c-873b-237f141f4a17.png)

Defining Stage Object:
This struct outlines the information captured at every stage of the supply chain, This includes, 
stage title, location, as well as product impact on nature, community and climate.
![stage_struct](https://user-images.githubusercontent.com/19547628/180221231-eb5b46b6-4d60-4d48-bb24-b4fe3ccd64df.png)

Add
Method in product.rs file with the functionality of adding stages to the vector variable in product struct
![product_function_add_stage](https://user-images.githubusercontent.com/19547628/180221852-d11ed2e5-42f3-4b0d-ab58-6e1a00c56ef6.png)

Show
Method in product.rs file with the functionality of showing added stages in the vector variable in product struct
![product_function_show_stage](https://user-images.githubusercontent.com/19547628/180221864-a0ac3bd7-f9df-4f29-9c53-2b354bcdf899.png)

Remove
Method in product.rs file with the functionality of removing stage from product
![product_function_remove_stage](https://user-images.githubusercontent.com/19547628/180221857-b815865a-bdac-403c-bfdf-e3a619a83641.png)

Defining Circular Chain (Entry File)
Inside ./src directory open lib.rs file. This is the entry file to our smart contract; All the 
functions we'll invoke our smart contract will be defined here 
Products will be stored in products: LookupMap
![circular_chain_struct](https://user-images.githubusercontent.com/19547628/180223487-f1045be7-87dd-47a0-b2c6-3333c9fe0cfd.png)

Add Stage
Adds new stage and product, and if there's a product with the same index/id, then add stage
To perform add stage operation, the signer address must be contained in the stakeholders vector in Product.
![function_add_stage_implementation](https://user-images.githubusercontent.com/19547628/180221777-ddea3b14-29ae-4403-b6d7-40da1c011de1.png)

Update ESG Score
Updates the Products ESG Score, by calculating the aggregate score of each stage in along the respective product supply chain stages
![function_update_esg_score_implementation](https://user-images.githubusercontent.com/19547628/180221842-27a1762f-fc8b-43d1-bf0c-7403213dc49b.png)

Read Product
Retrieves product in the LookupMap.
![function_read_product_implementation](https://user-images.githubusercontent.com/19547628/180221808-b002498a-d018-47c0-9e19-646c14a49531.png)


Read Stages
Retrieves all stages associated with the product ID.
![function_read_stages_implementation](https://user-images.githubusercontent.com/19547628/180221815-f824e724-0f3d-4318-a34c-59d76211e568.png)

Deletes Stage
Removes stage associated with the Index.
![function_delete_stage_implementation](https://user-images.githubusercontent.com/19547628/180221803-626d2a3f-8490-4239-94cd-3424bb800dd6.png)

# Storage Staking 
![function_storage_staking](https://user-images.githubusercontent.com/19547628/180221828-a61a7c33-813c-436a-ae0b-4a0e3f04a4c4.png)

# Tests
Parameters
![test_parameters](https://user-images.githubusercontent.com/19547628/180223623-a8daa726-e2eb-490f-9139-fbabf9338113.png)

Add Stage
![function_add_stage_test](https://user-images.githubusercontent.com/19547628/180221790-02402ddb-b601-4ed2-bd1f-bc394035210d.png)

Delete Stage
![function_remove_stage_test](https://user-images.githubusercontent.com/19547628/180221820-4e96d2af-a181-4d28-a124-a5570e488484.png)

Update Score
![function_update_esg_score_test](https://user-images.githubusercontent.com/19547628/180221848-af077b2c-802c-450b-a854-141667b9dcde.png)

Calls different function using NEAR CLI
![call_scripts](https://user-images.githubusercontent.com/19547628/180221884-dc83e64e-be25-4370-a8d2-8f35279ee808.png)



