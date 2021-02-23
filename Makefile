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

LOCAL_IP = $(shell ipconfig getifaddr en0)

start-app: export API_URL=http://$(LOCAL_IP):8080
start-app: export HOST=$(LOCAL_IP)
start-app:
	cd app; yarn dev

check: check-service check-app

integration-test: integration-test-service

start: export HOST=$(LOCAL_IP)
start:
	docker-compose up --build -d app

AWS_ACCOUNT = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_ECS_URL = ${AWS_ACCOUNT}.dkr.ecr.us-east-1.amazonaws.com

build-images: export IMAGE_URL=${AWS_ECS_URL}
build-images:
	docker-compose build service app

push-images:
	docker push ${AWS_ECS_URL}/codenames_service
	docker push ${AWS_ECS_URL}/codenames_app


