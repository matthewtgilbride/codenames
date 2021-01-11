service-build:
	cd service && cargo build

service-format:
	cd service && cargo fmt

service-test:
	cd service && cargo test

service-check: service-build service-format service-test
