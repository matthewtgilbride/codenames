include ../help.mk

LOCAL_IP = $(shell ipconfig getifaddr en0)

##@ Yarn commands

compile: ## run typescript compiler
	yarn tsc

build: ## build the nextjs app
	yarn build
	yarn export

format: ## fix lint errors
	yarn lint --fix

check: format compile build ## lint, compile, build

start: ## start dev server
	yarn dev

start-build: ## start dev server
	yarn serve -s out

upload:
	aws s3 sync out s3://codenames.mattgilbride.com --cache-control max-age=86400
