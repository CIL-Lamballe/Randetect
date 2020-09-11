#!/bin/bash

# Stop randetect
sudo /etc/init.d/randetect stop

# Remove from startup
sudo update-rc.d -f randetect remove

# Delete from init.d
sudo rm -rf /etc/init.d/randetect

# Reload deamons
systemctl daemon-reload
