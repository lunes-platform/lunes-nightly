#!/usr/bin/env bash
BOOTNODE=""
BOOTNODE+="./target/release/lunes-node \n"
BOOTNODE+=" --node-key <your-node-key> \n"
BOOTNODE+=" --base-path /tmp/bootnode1 \n"
BOOTNODE+=" --chain lunes-staging-raw.json \n"     
BOOTNODE+=" --name bootnode1"     
printf "BOOTNODE"

VALIDATOR=""
VALIDATOR+="./target/release/lunes-node \n"
VALIDATOR+=" --base-path  /tmp/validator1 \n"
VALIDATOR+=" --chain   lunes-staging-raw.json \n"
VALIDATOR+="  --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \n"
VALIDATOR+="  --port 30336 \n"
VALIDATOR+="   --ws-port 9947 \n"
VALIDATOR+="   --rpc-port 9936 \n"
VALIDATOR+="   --name  validator1 \n"
VALIDATOR+="   --validator"
printf "VALIDATOR"