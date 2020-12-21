.PHONY: build test lint

build:
	cargo build

test:
	cargo test -- --test-threads=1

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --release --all --tests -- -D clippy::all -D warnings
