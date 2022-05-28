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

build-wasm: ## compile and build wasm subproject
	cargo build -p codenames-wasm --release --target wasm32-unknown-unknown

build-all: build build-wasm ## compile and build all projects

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

##@ Wasmcloud

wasmcloud-deps: ## start the auxiliary services needed to run wash locally
	docker-compose -f ../docker-compose.yml up -d registry nats

rust_log = debug
actor_id = $(shell wash claims inspect ./codenames.wasm -o json | jq -r .module)

wasmcloud-actor-sign: build-wasm ## sign the built wasmcloud actor
	wash claims sign \
		target/wasm32-unknown-unknown/release/codenames_wasm.wasm \
		--http_server \
		--cap wasmcloud:builtin:logging \
		--cap wasmcloud:builtin:numbergen \
		--cap aws:kvdynamodb \
		--name "codenames" \
		--destination ./codenames.wasm


codenames.wasm: wasmcloud-actor-sign
$(wildcard ./wasm/src/**/*): codenames.wasm

.PHONY: wasmcloud-actor-push
wasmcloud-actor-push: codenames.wasm ## push the built wasmcloud module to a local registry
	wash reg push localhost:5000/codenames:0.1.0 codenames.wasm --insecure

wasmcloud-actor-push-gh: codenames.wasm ## push the built wasmcloud module to ghcr
	wash reg push \
	-u matthewtgilbride -p $$GH_PERSONAL_ACCESS_TOKEN \
	ghcr.io/matthewtgilbride/codenames:0.1.0 codenames.wasm

.PHONY: wasmcloud-actor-update
wasmcloud-actor-update: wasmcloud-actor-push ## update the running actor in the wasmcloud host
	wash ctl update actor $(shell wash ctl get hosts -o json | jq -r '.hosts[0].id') $(codenames_actor_id) localhost:5000/codenames:v1

.PHONY: wasmcloud-host-start
wasmcloud-host-start: export RUST_LOG=$(rust_log)
wasmcloud-host-start: export WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000
wasmcloud-host-start: wasmcloud-deps ## start the wasmcloud actor locally
	wasmcloud_host start

.PHONY: wasmcloud-start
wasmcloud-start: wasmcloud-host-start
	wash ctl start provider ghcr.io/matthewtgilbride/kvdynamodb_provider:0.1.0
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.15.0
	wash ctl start actor ghcr.io/matthewtgilbride/codenames:0.1.0

wasmcloud-up: wasmcloud-start
	wash ctl link put \
	  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
	  VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M \
	  wasmcloud:httpserver \
	  PORT=8080 \
	  allowed_origins="http://localhost:3000"
	wash ctl link put \
	  MBBVDZJTLOX3O5XUHZB2BLG3GKGU4AOWVOVKWN5CH4AUPYHDSSABC454 \
	  VBM7ATBGFQ2ZWDECPWUTLRIIWSBPABHBL5UL5CD7LDNR5I4NKFXZJ5EA \
	  aws:kvdynamodb \
	  config_json='{ "table_name": "codenames", "key_attribute": "key", "value_attribute": "game" }'


wasmcloud_process = $(shell ps | grep wasmcloud | awk '{print $$1}')
.PHONY: wasmcloud-down
wasmcloud-down: kill-local ## tear down wasmcloud and it's dependencies
	-docker-compose down
	-wasmcloud_host stop
	-ps -ax | grep -i wasmcloud | awk '{print $$1}' | xargs kill -9

.PHONY: wash-up ## start the wasmcloud actor in a wash REPL
wash-up: export ACTOR_ID=$(shell wash claims inspect ./codenames.wasm -o json | jq -r .module)
wash-up: export RUST_LOG=$(rust_log)
wash-up: codenames.wasm
	wash up -m wasm/manifest.yaml
