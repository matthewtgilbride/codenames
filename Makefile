include help.mk

build-service: ## compile the rust REST service project
	$(MAKE) -C service build

format-service: ## run code formatting on the rust project
	$(MAKE) -C service format

test-service: ## run unit tests on the rust project
	$(MAKE) -C service test

check-service: ## check the rust project (format, build, and unit test)
	$(MAKE) -C service check

start-service: ## start the rust project locally using cargo run
	$(MAKE) -C service run-local

integration-test-service: ## run the newman integration test suite on the service
	$(MAKE) -C service integration-test

integration-test: integration-test-service ## as of now, there are only integration tests for the service

build-app: ## build the web UI
	$(MAKE) -C app build

format-app: ## lint the web UI
	$(MAKE) -C app format

check-app: ## build and lint the web UI
	$(MAKE) -C app check

start-app: ## start the web UI locally
	$(MAKE) -C app start

check: check-service check-app ## check both the service and app projects

build: build-service build-app ## build both the service and app projects

.EXPORT_ALL_VARIABLES:

AWS_ACCOUNT = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_ECR_URL = ${AWS_ACCOUNT}.dkr.ecr.us-east-1.amazonaws.com

LOCAL_IP = $(shell ifconfig | grep broadcast | head -n 1 | awk '{print $$2}')
SERVICE_PORT ?= 8080
APP_PORT ?= 3000

API_URL ?= http://$(LOCAL_IP):${SERVICE_PORT}
HOST ?= ${LOCAL_IP}

start: ## start fully functioning stack locally via docker
	docker-compose up -d app

deploy-infra: ## deploy AWS infrastructure
	${MAKE} -C infra deploy-registry
	${MAKE} -C infra deploy-cluster

destroy-infra: ## tear down AWS infrastructure
	${MAKE} -C infra destroy-cluster

build-service-image: ## build docker image for the service - CAREFUL - this needs to be done from an AMD powered machine (not Apple M1 silicon)
	docker-compose build service

build-app-image: ## build docker image for the web ui
	docker-compose build app

build-images: build-service-image build-app-image ## build both images

ecr-login: ## login into AWS ECR for docker
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin ${AWS_ECR_URL}

push-service-image: ## push service image
	docker push ${AWS_ECR_URL}/codenames_service

push-app-image: ## push app image
	docker push ${AWS_ECR_URL}/codenames_app

push-images: push-service-image push-app-image ## push both images

build-wash:
	docker-compose build wash

run-wash:
	docker-compose run --rm wash

