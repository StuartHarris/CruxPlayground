#!/usr/bin/env zsh

set -e -x

cargo run --package component --bin codegen --features=cli,facet_typegen
cargo run --package application --bin codegen --features=cli,facet_typegen

cd generated/ApplicationCrux
swift build