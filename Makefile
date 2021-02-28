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

check: check-service check-app

integration-test: integration-test-service

start-app:
	cd app; yarn dev

build: build-service build-app

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

