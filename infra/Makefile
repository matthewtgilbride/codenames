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
