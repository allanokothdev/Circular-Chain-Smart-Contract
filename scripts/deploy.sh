#!/bin/bash 

source ./scripts/setting.conf

near delete $SUB_ACCOUNT $MASTER_ACCOUNT 

near create-account $SUB_ACCOUNT --masterAccount $MASTER_ACCOUNT --initialBalance 50

near deploy $SUB_ACCOUNT --wasmFile=./res/circular_chain.wasm