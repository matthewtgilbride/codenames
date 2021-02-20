build-service:
	$(MAKE) -C service build

format-service:
	$(MAKE) -C service format

test-service:
	$(MAKE) -C service test

check-service:
	$(MAKE) -C service check

start-service:
	${MAKE} -C service run-local

integration-test-service:
	$(MAKE) -C service integration-test

build-app:
	cd app; yarn tsc
	cd app; yarn build

format-app:
	cd app; yarn lint --fix

check-app: format-app build-app

start-app:
	cd app; yarn dev

check: check-service check-app

integration-test: integration-test-service

start:
	docker-compose up --build -d app

service-gen-keys:
	$(MAKE) -C service gen-account-key
	$(MAKE) -C service gen-module-key

service-sign-module:
	$(MAKE) -C service sign-module

service-show-module:
	$(MAKE) -C service show-module


