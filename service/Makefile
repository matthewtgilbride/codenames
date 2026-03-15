include ../help.mk

# Force make to use bash to ensure a consistent shell across machines
SHELL := /bin/bash

##@ Rust

clean: ## cargo clean
	cargo clean

build: ## compile and build the service
	cargo build

# https://github.com/messense/homebrew-macos-cross-toolchains
build-release-x86_64: export CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
build-release-x86_64: export CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
build-release-x86_64: export AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
build-release-x86_64: export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc
build-release-x86_64: ## build a release for 64 bit AMD architectures
	cargo build --release --target x86_64-unknown-linux-musl

# https://github.com/messense/homebrew-macos-cross-toolchains
build-release-aarch64: export CC_aarch64_unknown_linux_musl=aarch64-unknown-linux-musl-gcc
build-release-aarch64: export CXX_aarch64_unknown_linux_musl=aarch64-unknown-linux-musl-g++
build-release-aarch64: export AR_aarch64_unknown_linux_musl=aarch64-unknown-linux-musl-ar
build-release-aarch64: export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-unknown-linux-musl-gcc
build-release-aarch64: ## build a release for 64 bit ARM architectures (e.g. Mac M1)
	cargo build --release --target aarch64-unknown-linux-musl

format: ## format code
	cargo +nightly fmt

test: ## test all subprojects
	cargo test --all

check: format build test ## format, build, and test

##@ Local Development

LOCAL_PID = $(shell lsof -t -i:8080)
kill-local: ## kill whatever is running on port 8080
	-kill $(LOCAL_PID)

LOCAL_IP = $(shell ipconfig getifaddr en0)

run-local: kill-local
run-local: export ALLOWED_ORIGINS=http://localhost:3000
run-local: ## run the http server locally
	cargo build
	cargo run

integration-test-local: ## run integration tests locally
	pipenv run pytest -v -s integration_tests.py

load-test: export TEST_URL=https://codenamesapi.mattgilbride.com
load-test:
	pipenv run pytest -v -s load_test.py

.EXPORT_ALL_VARIABLES:
AWS_ACCESS_KEY_ID = $(shell aws configure get aws_access_key_id)
AWS_SECRET_ACCESS_KEY = $(shell aws configure get aws_secret_access_key)
AWS_ACCOUNT = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_ECR_URL = ${AWS_ACCOUNT}.dkr.ecr.us-east-1.amazonaws.com

run-docker: build-release-x86_64 ## run the dockerized service as it will run on EC2
	docker-compose -f ../docker-compose.yml up --build service

integration-test-docker: check build-release-x86_64 ## check code, build newman test container, and run it
	docker-compose -f ../docker-compose.yml up --build --exit-code-from service-test service-test

integration-test: integration-test-docker ## run docker integration tests and clean up
	docker-compose -f ../docker-compose.yml down

