include ../help.mk

# Force make to use bash to ensure a consistent shell across machines
SHELL := /bin/bash

##@ Rust

clean: ## cargo clean
	cargo clean

build: ## compile and build non-wasm subprojects
	cargo build

build-wasm: ## compile and build wasm subproject
	cargo build -p codenames-wasm --release --target wasm32-unknown-unknown

build-all: build build-wasm ## compile and build all projects

format: ## format code
	cargo fmt

test: ## test all subprojects
	cargo test --all

check: format build test ## format, build, and test

##@ Local Development

redis-up: ## bring up a local redis container
	docker-compose -f ../docker-compose.yml up -d redis

LOCAL_PID = $(shell lsof -t -i:8080)
kill-local: ## kill whatever is running on port 8080
	-kill $(LOCAL_PID)

LOCAL_IP = $(shell ipconfig getifaddr en0)

run-local: redis-up kill-local
run-local: export REDIS_HOST=0.0.0.0
run-local: export ALLOWED_ORIGINS=http://$(LOCAL_IP):3000
run-local: ## run the http server locally
	cargo build
	cargo run

integration-test-local: ## run newman tests locally
	yarn test

integration-test-docker: check ## check code, build newman test container, and run it
	docker-compose -f ../docker-compose.yml up --build --exit-code-from service-test service-test

integration-test: integration-test-docker ## run docker integration tests and clean up
	docker-compose -f ../docker-compose.yml down

##@ Wasmcloud

wasmcloud-deps: ## start the auxiliary services needed to run wash locally
	docker-compose -f ../docker-compose.yml up -d registry nats redis

rust_log = info
wasmcloud_host_id = $(shell wash ctl get hosts -o json | jq -r '.hosts[0].id')
codenames_actor_id = $(shell wash claims inspect ./codenames.wasm -o json | jq -r .module)

wasmcloud-actor-build: build-wasm ## compile and build wasmcloud actor mosule

wasmcloud-actor-sign: wasmcloud-actor-build ## sign the built wasmcloud actor
	wash claims sign target/wasm32-unknown-unknown/release/codenames_wasm.wasm --keyvalue --http_server --logging --cap wasmcloud:extras --name "codenames" --destination ./codenames.wasm

codenames.wasm: build-wasm wasmcloud-actor-sign
$(wildcard ./wasm/src/**/*): codenames.wasm

.PHONY: wasmcloud-actor-push
wasmcloud-actor-push: codenames.wasm ## push the built wasmcloud module to a local registry
	wash reg push localhost:5000/codenames:v1 codenames.wasm --insecure

.PHONY: wasmcloud-actor-update
wasmcloud-actor-update: wasmcloud-actor-push ## update the running actor in the wasmcloud host
	wash ctl update actor $(wasmcloud_host_id) $(codenames_actor_id) localhost:5000/codenames:v1

.PHONY: wasmcloud-up
wasmcloud-up: export ACTOR_ID=$(codenames_actor_id)
wasmcloud-up: export RUST_LOG=$(rust_log)
wasmcloud-up: wasmcloud-deps codenames.wasm ## start the wasmcloud actor locally
	wasmcloud --allowed-insecure localhost:5000 -m wasm/manifest.yaml

wasmcloud_process = $(shell ps | grep wasmcloud | awk '{print $$1}')
.PHONY: wasmcloud-down
wasmcloud-down: kill-local ## tear down wasmcloud and it's dependencies
	docker-compose down
	-kill $(wasmcloud_process)

.PHONY: wash-up ## start the wasmcloud actor in a wash REPL
wash-up: export ACTOR_ID=$(codenames_actor_id)
wash-up: export RUST_LOG=$(rust_log)
wash-up: codenames.wasm
	wash up -m wasm/manifest.yaml
