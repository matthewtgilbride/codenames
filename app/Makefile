include ../help.mk

##@ Yarn commands

compile: ## run typescript compiler
	yarn tsc

build: ## build the nextjs app
	yarn build

format: ## fix lint errors
	yarn lint --fix

check: format compile build ## lint, compile, build

start: ## start dev server
	yarn dev
