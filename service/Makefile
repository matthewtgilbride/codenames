include ../help.mk

# Force make to use bash to ensure a consistent shell across machines
SHELL := /bin/bash

##@ Rust

clean: ## cargo clean
	cargo clean

build: ## compile and build the service
	cargo build

format: ## format code
	cargo +nightly fmt

test: ## test all subprojects
	cargo test --all

check: format build test ## format, build, and test

##@ Lambda

build-lambda: ## build the lambda function with cargo-lambda
	cargo lambda build --release --compiler cargo --package codenames-lambda

##@ Local Development

run-lambda-local: ## run the lambda function locally with cargo-lambda watch
	cargo lambda watch --package codenames-lambda

##@ Testing

integration-test: ## run integration tests
	python3 -m pytest integration_tests.py -v
