include ../help.mk

.EXPORT_ALL_VARIABLES:
AWS_ACCOUNT_NUMBER = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_DEFAULT_REGION = us-east-1
PUBLIC_IP=$(shell dig +short myip.opendns.com @resolver1.opendns.com)

bootstrap: export DEPLOY_TYPE=app
bootstrap:
	yarn cdk bootstrap

##@ App Deployment

synth-app: export DEPLOY_TYPE=app
synth-app: ## validate cfn for development EC2 x86 instance
	yarn cdk synth

deploy-app: export DEPLOY_TYPE=app
deploy-app: ## deploy development EC2 instance
	yarn cdk deploy

destroy-app: export DEPLOY_TYPE=app
destroy-app: ## destroy development EC2 instance
	yarn cdk destroy

##@ ECR Registry

synth-registry: export DEPLOY_TYPE=registry
synth-registry: ## validate cfn for ECR repos
	yarn cdk synth

deploy-registry: export DEPLOY_TYPE=registry
deploy-registry: ## deploy ECR repos
	yarn cdk deploy

destroy-registry: export DEPLOY_TYPE=registry
destroy-registry: ## destroy ECR repos
	yarn cdk destroy

##@ x86 Development Instance (for when Apple M1 is a pain)

synth-dev: export DEPLOY_TYPE=dev
synth-dev: ## validate cfn for development EC2 x86 instance
	yarn cdk synth

deploy-dev: export DEPLOY_TYPE=dev
deploy-dev: ## deploy development EC2 instance
	yarn cdk deploy

destroy-dev: export DEPLOY_TYPE=dev
destroy-dev: ## destroy development EC2 instance
	yarn cdk destroy

##@ LEGACY ECS Cluster: Why is the ALB so damn expensive?

synth-cluster: export DEPLOY_TYPE=cluster
synth-cluster: ## validate cfn for ECS cluster
	yarn cdk synth

deploy-cluster: export DEPLOY_TYPE=cluster
deploy-cluster: ## deploy ECS cluster
	yarn cdk deploy

destroy-cluster: export DEPLOY_TYPE=cluster
destroy-cluster: ## destroy ECS cluster
	yarn cdk destroy