.PHONY: build test lint bench

build:
	cargo build --verbose

test:
	cargo test --verbose -- --test-threads=1

lint:
	cargo clippy --release --all --tests --verbose -- -D clippy::all -D warnings

bench:
	cargo +nightly bench -- --test-threads=1
