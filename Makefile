include help.mk

##@ Backend (Rust REST Service)

build-service: ## build the lambda function for deployment
	$(MAKE) -C service build-lambda

format-service: ## run code formatting on the rust project
	$(MAKE) -C service format

test-service: ## run unit tests on the rust project
	$(MAKE) -C service test

check-service: ## check the rust project (format, build, and unit test)
	$(MAKE) -C service check

start-service: ## start the rust service locally using actix
	$(MAKE) -C service run-local

start-lambda: ## start the lambda function locally using cargo-lambda
	$(MAKE) -C service run-lambda-local

##@ Frontend (Typescript/NextJS Web App)

build-app: ## build the web UI
	$(MAKE) -C app build

format-app: ## lint the web UI
	$(MAKE) -C app format

check-app: ## build and lint the web UI
	$(MAKE) -C app check

start-app: ## start the web UI locally
	$(MAKE) -C app start

##@ Check, Build, and Start both

check: check-service check-app ## check both the service and app projects

build: build-service build-app ## build both the service and app projects

##@ Infrastructure (AWS CDK)

deploy-infra: build-service build-app ## build and deploy AWS infrastructure
	${MAKE} -C infra deploy
	${MAKE} deploy-ui

deploy-ui:
	${MAKE} -C app upload

destroy-infra: ## tear down AWS infrastructure
	${MAKE} -C infra destroy
