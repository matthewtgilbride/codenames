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

synth-instance: export DEPLOY_TYPE=instance
synth-instance: export PUBLIC_IP=$(public_ip)
synth-instance: ## validate cfn for development EC2 x86 instance
	yarn cdk synth

deploy-instance: export DEPLOY_TYPE=instance
deploy-instance: export PUBLIC_IP=$(public_ip)
deploy-instance: ## deploy development EC2 instance
	yarn cdk deploy

destroy-instance: export DEPLOY_TYPE=instance
destroy-instance: export PUBLIC_IP=$(public_ip)
destroy-instance: ## destroy development EC2 instance
	yarn cdk destroy
