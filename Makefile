install:
	rustup update &&\
		cargo build --release

test:
	cargo test

format:	
	cargo fmt

lint:
	cargo clippy

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

run:
# 	cargo run

deploy:
	cargo run square 5
		
all: install lint test format refactor
