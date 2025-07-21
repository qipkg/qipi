alias b := build
alias t := test
alias l := lint
alias f := fmt

build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

check:
	cargo check --workspace

ci:
	just fmt
	just lint
	just test