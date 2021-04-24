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

wasmcloud-deps: ## start the auxiliary services needed to run wash locally
	docker-compose -f ../docker-compose.yml up -d registry nats redis

rust_log := debug

wasmcloud-start: wasmcloud-deps ## start a wasmcloud host
wasmcloud-start: export RUST_LOG=$(rust_log)
wasmcloud-start:
	-wash drain all
	wasmcloud --allowed-insecure localhost:5000

.PHONY: wasmcloud-start-background
wasmcloud-start-background: ## start and detach from the wasmcloud host process
	$(MAKE) wasmcloud-start &

wasmcloud-actor-build: build-wasm ## compile and build wasmcloud actor mosule

wasmcloud-actor-sign: wasmcloud-actor-build ## sign the built wasmcloud actor
	wash claims sign target/wasm32-unknown-unknown/release/codenames_wasm.wasm --keyvalue --http_server --logging --cap wasmcloud:extras --name "codenames" --destination ./codenames.wasm

codenames.wasm: build-wasm wasmcloud-actor-sign

.PHONY: wasmcloud-actor-push
wasmcloud-actor-push: codenames.wasm ## push the built wasmcloud module to a local registry
	wash reg push localhost:5000/codenames:v1 codenames.wasm --insecure

.PHONY: wasmcloud-providers-start
wasmcloud-providers-start: ## start providers on the local machine
	wash ctl start provider wasmcloud.azurecr.io/redis:0.11.2
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.12.1
	wash ctl start provider wasmcloud.azurecr.io/logging:0.9.3

.PHONY: wasmcloud-actor-start
wasmcloud-actor-start: wasmcloud-actor-push ## start the codenames wasmcloud actor
	wash ctl start actor localhost:5000/codenames:v1

.PHONY: wasmcloud-up
wasmcloud-up: wasmcloud-actor-start wasmcloud-providers-start ## start the wasmcloud actor locally
	wash ctl link MCW7EHKF5Y4GZ2SDSJTNLFO7U6RZFQ5ZC4ZAXHSUPTKJPHKLPPDR3HEI VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB wasmcloud:keyvalue
	wash ctl link MCW7EHKF5Y4GZ2SDSJTNLFO7U6RZFQ5ZC4ZAXHSUPTKJPHKLPPDR3HEI VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080
	wash ctl link MCW7EHKF5Y4GZ2SDSJTNLFO7U6RZFQ5ZC4ZAXHSUPTKJPHKLPPDR3HEI VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP wasmcloud:extras
	wash ctl link MCW7EHKF5Y4GZ2SDSJTNLFO7U6RZFQ5ZC4ZAXHSUPTKJPHKLPPDR3HEI VCCANMDC7KONJK435W6T7JFEEL7S3ZG6GUZMZ3FHTBZ32OZHJQR5MJWZ wasmcloud:logging

wasmcloud_process = $(shell ps | grep wasmcloud | awk '{print $$1}')

.PHONY: wasmcloud-down
wasmcloud-down: kill-local## tear down wasmcloud and it's dependencies
	docker-compose down
	-kill $(wasmcloud_process)
