build:
	cargo build
	cargo build -p codenames-wasm --target wasm32-unknown-unknown

format:
	cargo fmt

test:
	cargo test --all

check: format build test

docker-up:
	docker-compose up -d

local-integration-test: docker-up
	yarn test

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

push-local: docker-up
	wash reg push localhost:5000/codenames:v1 codenames.wasm --insecure

start-local-providers: docker-up
	wash ctl start provider wasmcloud.azurecr.io/redis:0.9.2
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.9.2

start-actor: docker-up
	wash ctl start actor localhost:5000/codenames:v1

show-module:
	wascap caps ~/.wash/codenames_wasm_module.nk