#!/usr/bin/env bash

BOOTNODE=""
BOOTNODE+=".\\targe\\release\\lunes-node "
BOOTNODE+="  --ws-external "
BOOTNODE+="  --rpc-external "
BOOTNODE+="  ---rpc-cors all "
BOOTNODE+="  ---rpc-methods=unsafe "
BOOTNODE+=" --node-key 0d3fe51599e66fe706caeb2a886698f095112288f6f72d4b773b67785bc76516 "
BOOTNODE+=" --base-path \\tmp\\bootnode1 "
BOOTNODE+=" --chain lunes-staging-raw.json "
BOOTNODE+=" --name bootnode1"

VALIDATOR=""
VALIDATOR+=".\\target\\release\\lunes-node "
VALIDATOR+=" --base-path  /tmp/validator1 "
VALIDATOR+=" --chain   lunes-staging-raw.json "
VALIDATOR+="  --bootnodes  /ip4/31.220.50.80/tcp/30333/p2p/12D3KooWAQaWuv8Ng7jxydotEkS7jX69i1vyhi9JU418RasVCZv5 "
VALIDATOR+="  --ws-external "
VALIDATOR+="  --rpc-external "
VALIDATOR+="  ---rpc-cors all "
VALIDATOR+="  ---rpc-methods=unsafe "
VALIDATOR+="  --port 30336 "
VALIDATOR+="   --ws-port 9947 "
VALIDATOR+="   --rpc-port 9936 "
VALIDATOR+="   --name  validator1 "
VALIDATOR+="   --validator"

# Execute o BOOTNODE em um processo separado
nohup   "$BOOTNODE" &

# Execute o VALIDATOR em um processo separado
nohup   "$VALIDATOR" &