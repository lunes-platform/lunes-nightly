#!/bin/bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey",   "params": [
        "imol",
        "",
        ""
    ] }' http://127.0.0.1:9934
printf "\n"
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey",  "params": [
        "gran",
        "",
        ""
    ] }' http://127.0.0.1:9934
printf "\n"
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey", "params": [
        "babe",
        "",
        ""
    ]}' http://127.0.0.1:9934

printf "\n User in App extrinsics \n"

curl -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' http://127.0.0.1:9934