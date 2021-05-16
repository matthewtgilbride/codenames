include ../help.mk

.EXPORT_ALL_VARIABLES:
AWS_ACCOUNT_NUMBER = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_DEFAULT_REGION := us-east-1

synth-registry: export DEPLOY_TYPE=registry
synth-registry: ## validate cfn for ECR repos
	yarn cdk synth

deploy-registry: export DEPLOY_TYPE=registry
deploy-registry: ## deploy ECR repos
	yarn cdk deploy

destroy-registry: export DEPLOY_TYPE=registry
destroy-registry: ## destroy ECR repos
	yarn cdk destroy

synth-cluster: export DEPLOY_TYPE=cluster
synth-cluster: ## validate cfn for ECS cluster
	yarn cdk synth

deploy-cluster: export DEPLOY_TYPE=cluster
deploy-cluster: ## deploy ECS cluster
	yarn cdk deploy

destroy-cluster: export DEPLOY_TYPE=cluster
destroy-cluster: ## destroy ECS cluster
	yarn cdk destroy

public_ip = $(shell dig +short myip.opendns.com @resolver1.opendns.com)

synth-dev-instance: export DEPLOY_TYPE=app-instance
synth-dev-instance: export PUBLIC_IP=$(public_ip)
synth-dev-instance: ## validate cfn for development EC2 x86 instance
	yarn cdk synth

deploy-dev-instance: export DEPLOY_TYPE=app-instance
deploy-dev-instance: export PUBLIC_IP=$(public_ip)
deploy-dev-instance: ## deploy development EC2 instance
	yarn cdk deploy

destroy-dev-instance: export DEPLOY_TYPE=app-instance
destroy-dev-instance: export PUBLIC_IP=$(public_ip)
destroy-dev-instance: ## destroy development EC2 instance
	yarn cdk destroy

synth-app-instance: export DEPLOY_TYPE=app-instance
synth-app-instance: export PUBLIC_IP=$(public_ip)
synth-app-instance: ## validate cfn for development EC2 x86 instance
	yarn cdk synth

deploy-app-instance: export DEPLOY_TYPE=app-instance
deploy-app-instance: export PUBLIC_IP=$(public_ip)
deploy-app-instance: ## deploy development EC2 instance
	yarn cdk deploy

destroy-app-instance: export DEPLOY_TYPE=app-instance
destroy-app-instance: export PUBLIC_IP=$(public_ip)
destroy-app-instance: ## destroy development EC2 instance
	yarn cdk destroy
