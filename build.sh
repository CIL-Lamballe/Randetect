#!/bin/bash

cross build --target=aarch64-unknown-linux-musl --release
cp target/aarch64-unknown-linux-musl/release/randetect randetect_aarch64-musl

cross build --target=x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/randetect randetect_x86_64-musl
