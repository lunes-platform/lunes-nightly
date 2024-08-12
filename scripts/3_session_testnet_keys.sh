#!/bin/bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey",  "params": [
        "gran",
        "",
        "0x"
    ] }' http://127.0.0.1:9938
printf "\n"
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey", "params": [
        "aura",
        "",
        "0x"
    ]}' http://127.0.0.1:9938

printf "\n User in App extrinsics \n"

curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' http://127.0.0.1:9938