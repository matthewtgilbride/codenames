.PHONY: $(MAKECMDGOALS)

build:
	cargo build
	cargo build -p codenames-wasm --target wasm32-unknown-unknown

format:
	cargo fmt

test:
	cargo test --all

check: format build test

redis-up:
	docker-compose -f ../docker-compose.yml up -d redis

LOCAL_PID = $(shell lsof -t -i:8080)
kill-local:
	-kill $(LOCAL_PID)

run-local: redis-up kill-local
run-local: export REDIS_HOST=0.0.0.0
run-local: export ALLOWED_ORIGINS=http://localhost:3000
run-local:
	cargo build
	cargo run

integration-test-local:
	yarn test

integration-test-docker: check
	docker-compose -f ../docker-compose.yml up --build --exit-code-from service-test service-test

integration-test: integration-test-docker
	docker-compose -f ../docker-compose.yml down

gen-account-key:
	nk gen account -o json | jq -r .seed > account.nk

gen-module-key:
	nk gen module -o json | jq -r .seed > module.nk

install-wasmcloud:
	cargo install wasmcloud  --git https://github.com/wasmCloud/wasmCloud --branch main

start-wasmcloud:
	wasmcloud

sign-local:
	wash claims sign target/wasm32-unknown-unknown/debug/codenames_wasm.wasm --keyvalue --http_server --name "codenames" --destination ./codenames.wasm

push-local:
	wash reg push localhost:5000/codenames:v1 codenames.wasm --insecure

start-local-providers:
	wash ctl start provider wasmcloud.azurecr.io/redis:0.9.2
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.9.2

start-actor:
	wash ctl start actor localhost:5000/codenames:v1

show-module:
	wascap caps ~/.wash/codenames_wasm_module.nk