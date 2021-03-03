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

integration-test: integration-test-service

build-app:
	${MAKE} -C app build

format-app:
	${MAKE} -C app format

check-app:
	${MAKE} -C app check

start-app:
	${MAKE} -C app format

check: check-service check-app

build: build-service build-app

deploy-infra:
	${MAKE} -C infra deploy-registry
	${MAKE} -C infra deploy-cluster

destroy-infra:
	${MAKE} -C infra destroy-cluster

.EXPORT_ALL_VARIABLES:

AWS_ACCOUNT := $(shell aws sts get-caller-identity | jq -r .Account)
AWS_ECR_URL := ${AWS_ACCOUNT}.dkr.ecr.us-east-1.amazonaws.com

LOCAL_IP = $(shell ifconfig en0 | grep -i mask | awk '{print $2}')
SERVICE_PORT := 8080
APP_PORT := 3000

API_URL := http://$(LOCAL_IP):${SERVICE_PORT}
HOST := ${LOCAL_IP}

build-service-image:
	docker-compose build service

build-app-image:
	docker-compose build app

start:
	docker-compose up -d app

build-images: build-service-image build-app-image

ecr-login:
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin ${AWS_ECR_URL}

push-service-image:
	docker push ${AWS_ECR_URL}/codenames_service

push-app-image:
	docker push ${AWS_ECR_URL}/codenames_app

push-images: push-service-image push-app-image

