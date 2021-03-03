include ../help.mk

.EXPORT_ALL_VARIABLES:
AWS_ACCOUNT_NUMBER = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_DEFAULT_REGION := us-east-1

synth-registry: export DEPLOY_TYPE=registry ## validate cfn for ECR repos
synth-registry:
	yarn cdk synth

deploy-registry: export DEPLOY_TYPE=registry ## deploy ECR repos
deploy-registry:
	yarn cdk deploy

destroy-registry: export DEPLOY_TYPE=registry ## destroy ECR repos
destroy-registry:
	yarn cdk destroy

synth-cluster: export DEPLOY_TYPE=cluster ## validate cfn for ECS cluster
synth-cluster:
	yarn cdk synth

deploy-cluster: export DEPLOY_TYPE=cluster ## deploy ECS cluster
deploy-cluster:
	yarn cdk deploy

destroy-cluster: export DEPLOY_TYPE=cluster ## destroy ECS cluster
destroy-cluster:
	yarn cdk destroy
