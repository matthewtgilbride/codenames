service-build:
	$(MAKE) -C service build

service-format:
	$(MAKE) -C service format

service-test:
	$(MAKE) -C service test

service-check: service-build service-format service-test
