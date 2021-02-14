service-build:
	$(MAKE) -C service build

service-format:
	$(MAKE) -C service format

service-test:
	$(MAKE) -C service test

service-check: service-build service-format service-test

service-start:
	${MAKE} -C service run-local

service-integration-test:
	$(MAKE) -C service integration-test

app-tsc:
	cd app; yarn tsc

app-build:
	cd app; yarn build

app-format:
	cd app; yarn lint --fix

app-check: app-tsc app-format app-build

check: service-integration-test app-check

app-start:
	cd app: yarn dev

service-gen-keys:
	$(MAKE) -C service gen-account-key
	$(MAKE) -C service gen-module-key

service-sign-module:
	$(MAKE) -C service sign-module

service-show-module:
	$(MAKE) -C service show-module


