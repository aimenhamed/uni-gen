#!/bin/bash

cargo build --release
sudo cp target/release/uni-gen /usr/local/bin/uni-gen