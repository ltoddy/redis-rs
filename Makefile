.PHONY: build test lint bench

build:
	cargo build

test:
	cargo test -- --test-threads=1

lint:
	cargo clippy --release --all --tests -- -D clippy::all -D warnings

bench:
	cargo +nightly bench -- --test-threads=1
