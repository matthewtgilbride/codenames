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
	cargo lambda build --release --package codenames-lambda

##@ Local Development

LOCAL_PID = $(shell lsof -t -i:8080)
kill-local: ## kill whatever is running on port 8080
	-kill $(LOCAL_PID)

run-local: kill-local
run-local: export ALLOWED_ORIGINS=http://localhost:3000
run-local: ## run the actix http server locally
	cargo build
	cargo run --package codenames-actix

run-lambda-local: ## run the lambda function locally with cargo-lambda watch
	cargo lambda watch --package codenames-lambda

.EXPORT_ALL_VARIABLES:
AWS_ACCESS_KEY_ID = $(shell aws configure get aws_access_key_id)
AWS_SECRET_ACCESS_KEY = $(shell aws configure get aws_secret_access_key)
