#!/bin/bash
export $(grep -v '^#' .env | xargs)
cargo test