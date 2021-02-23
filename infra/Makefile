AWS_ACCOUNT = $(shell aws sts get-caller-identity | jq -r .Account)

synth: export AWS_ACCOUNT_NUMBER=${AWS_ACCOUNT}
synth: export AWS_DEFAULT_REGION=us-east-1
synth:
	yarn cdk synth

deploy: export AWS_ACCOUNT_NUMBER=${AWS_ACCOUNT}
deploy: export AWS_DEFAULT_REGION=us-east-1
deploy:
	yarn deploy
