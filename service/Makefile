include ../help.mk

build: ## compile and build all subprojects
	cargo build
	cargo build -p codenames-wasm --target wasm32-unknown-unknown

format: ## format code
	cargo fmt

test: ## test all subprojects
	cargo test --all

check: format build test ## format, build, and test

redis-up: ## bring up a local redis container
	docker-compose -f ../docker-compose.yml up -d redis

LOCAL_PID = $(shell lsof -t -i:8080)
kill-local: ## kill whatever is running on port 8080
	-kill $(LOCAL_PID)

LOCAL_IP = $(shell ipconfig getifaddr en0)

run-local: redis-up kill-local ## run the http server locally
run-local: export REDIS_HOST=0.0.0.0
run-local: export ALLOWED_ORIGINS=http://$(LOCAL_IP):3000
run-local:
	cargo build
	cargo run

integration-test-local: ## run newman tests locally
	yarn test

integration-test-docker: check ## check code, build newman test container, and run it
	docker-compose -f ../docker-compose.yml up --build --exit-code-from service-test service-test

integration-test: integration-test-docker ## run integration tests and clean up
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