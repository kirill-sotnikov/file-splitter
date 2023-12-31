#!/bin/bash

set -e

curl -LO https://github.com/kirill-sotnikov/file-splitter/releases/download/v1.0.0/file-joiner
curl -LO https://github.com/kirill-sotnikov/file-splitter/releases/download/v1.0.0/file-splitter

chmod +x file-joiner
chmod +x file-splitter

sudo mv file-splitter /usr/local/bin
sudo mv file-joiner /usr/local/bin
