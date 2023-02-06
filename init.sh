#!/bin/bash

# Use this script to initialize your development environment. This will copy the KallistiOS bindings to your
# src directory, if it exists.

[ -d "kos-rust-bindings" ] && cp kos-rust-bindings/kos.rs src/kos.rs