#!/bin/bash
cp lunes_bootnode.service  /etc/systemd/system/lunes_bootnode.service
cp lunes_validator.service  /etc/systemd/system/lunes_validator.service
sudo systemctl daemon-reload
sudo systemctl start lunes_bootnode.service
sudo systemctl start lunes_validator.service