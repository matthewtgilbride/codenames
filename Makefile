service-build:
	$(MAKE) -C service build

service-format:
	$(MAKE) -C service format

service-test:
	$(MAKE) -C service test
	$(MAKE) -C service local-integration-test

service-check: service-build service-format service-test

service-gen-keys:
	$(MAKE) -C service gen-account-key
	$(MAKE) -C service gen-module-key

service-sign-module:
	$(MAKE) -C service sign-module

service-show-module:
	$(MAKE) -C service show-module


