#!/usr/bin/env bash

DAY=$(printf %02d "$1")

echo "$DAY"

cp ./src/template/xx.rs "./src/bin/$DAY.rs"

touch "./data/examples/$DAY.txt"
touch "./data/inputs/$DAY.txt"
