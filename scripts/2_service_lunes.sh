#!/bin/bash
cp lunes_validator.service  /etc/systemd/system/lunes_validator.service
sudo systemctl daemon-reload
sudo systemctl start lunes_validator.service
