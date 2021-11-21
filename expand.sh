#!/bin/bash

set -e
set pipefail

mkdir -p target

cargo +nightly rustc --lib --profile=test -- -Zunpretty=expanded > target/expanded.rs
