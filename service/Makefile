build:
	cargo build
	cargo build -p codenames-wasm --target wasm32-unknown-unknown

format:
	cargo fmt

test:
	cargo test --all

check: build format test

gen-account-key:
	nk gen account -o json | jq -r .seed > account.nk

gen-module-key:
	nk gen module -o json | jq -r .seed > module.nk

sign-module:
	wascap sign target/wasm32-wasi/debug/service.wasm service.wasm -u ./module.nk -i ./account.nk -n "codenames-service" -s -k

show-module:
	wascap caps ./service.wasm