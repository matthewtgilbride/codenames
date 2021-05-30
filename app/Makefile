include ../help.mk

LOCAL_IP = $(shell ipconfig getifaddr en0)

##@ Yarn commands

compile: ## run typescript compiler
	yarn tsc

build: ## build the nextjs app
	yarn build

format: ## fix lint errors
	yarn lint --fix

check: format compile build ## lint, compile, build

start: export API_URL=http://${LOCAL_IP}:8080
start: ## start dev server
	yarn dev
