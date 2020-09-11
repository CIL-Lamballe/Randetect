#!/bin/bash

# Add to init.d
cp ~/randetect /etc/init/.
chmod 0755 /etc/init.d/randetect

# Restart deamons
systemctl daemon-reload

# Tests
/etc/init/randetect start
/etc/init/randetect stop

# Add to startup
sudo update-rc.d randetect defaults
/etc/init/randetect start
