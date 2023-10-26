cp lunes_testnet_bootnode.service  /etc/systemd/system/lunes_testnet_bootnode.service
cp lunes_testnet_validator.service  /etc/systemd/system/llunes_testnet_validator.service
sudo systemctl daemon-reload
sudo systemctl start lunes_testnet_bootnode.service
sudo systemctl start llunes_testnet_validator.service

curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey", "params":["babe", "<your_seed>", "<your_public_key_has>"] }' http://127.0.0.1:9933

curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey", "params":["gran", "<your_seed>", "<your_public_key_has>"] }' http://127.0.0.1:9933

curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", 
"method": "author_insertKey", "params":["imol","<your_seed>", "<your_public_key_has>"] }' http://127.0.0.1:9933

