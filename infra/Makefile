include ../help.mk

.EXPORT_ALL_VARIABLES:
AWS_ACCOUNT_NUMBER = $(shell aws sts get-caller-identity | jq -r .Account)
AWS_DEFAULT_REGION = us-east-1

bootstrap: ## bootstrap CDK
	yarn cdk bootstrap

synth: ## synthesize CloudFormation template
	yarn cdk synth

diff: ## show CDK diff
	yarn cdk diff

deploy: ## deploy the stack
	yarn cdk deploy

destroy: ## destroy the stack
	aws s3 rm s3://codenames.mattgilbride.com --recursive
	yarn cdk destroy
