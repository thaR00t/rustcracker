#/bin/bash

sudo apt install -y cargo 

sudo cargo build --release

sudo cp ./target/release/rustcracker /usr/bin

echo "Now you can run rustcracker from everywhere you want"

rustcracker