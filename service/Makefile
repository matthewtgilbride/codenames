include ../help.mk

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

integration-test: integration-test-docker ## run integration tests and clean up
	docker-compose -f ../docker-compose.yml down

sign-wasmcloud: ## sign the built wasmcloud module
	wash claims sign target/wasm32-unknown-unknown/release/codenames_wasm.wasm --keyvalue --http_server --name "codenames" --destination ./codenames.wasm

push-wasmcloud: ## push the built wasmcloud module to a local registry
	wash reg push localhost:5000/codenames:v1 codenames.wasm --insecure

start-wasmcloud-providers: ## start providers on the local machine
	wash ctl start provider wasmcloud.azurecr.io/redis:0.11.2
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.12.1

start-wasmcloud-actor: ## start the codenames wasmcloud actor
	wash ctl start actor localhost:5000/codenames:v1

start-wasmcloud: ## start a wasmcloud host
	wasmcloud --allowed-insecure localhost:5000

link-actor-to-providers: ## link actor to capability providers - need to provide ACTOR_PUBLIC_KEY in the environment
	wash ctl link ${ACTOR_PUBLIC_KEY} VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB wasmcloud:keyvalue
	wash ctl link ${ACTOR_PUBLIC_KEY} VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080
	wash ctl link ${ACTOR_PUBLIC_KEY} VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP wasmcloud:extras
