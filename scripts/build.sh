#!/bin/sh

set -e

RUSTFLAGS="-C link-arg=-s" cargo build --release
