#!/bin/sh
###############################################################
# Copyright (c) 2020 Silvio Clecio (silvioprog)
###############################################################

set -e

RUSTFLAGS="-C link-arg=-s" cargo build --release
