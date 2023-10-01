#!/bin/sh

rustup target add wasm32-unknown-unknown wasm32-wasi
cargo install lalrpop

sudo apt-add-repository -y ppa:mosquitto-dev/mosquitto-ppa
sudo apt-get update
sudo apt-get install -y mosquitto

sudo service mosquitto start
