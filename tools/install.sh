#!/bin/bash

# Add to init.d
sudo cp ~/randetect /etc/init.d/.
sudo chmod 0755 /etc/init.d/randetect

# Restart deamons
systemctl daemon-reload

# Tests
sudo /etc/init.d/randetect start
sudo /etc/init.d/randetect stop

# Add to startup
sudo update-rc.d randetect defaults
sudo /etc/init.d/randetect start
