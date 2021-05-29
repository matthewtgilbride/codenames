# codenames

Hi.  This is a personal project that exists for two reasons:

1. To learn Rust
2. To make my own version of [codenames](https://codenames.game/) that I can play with friends and family (inspiration: [horsepaste](https://github.com/jbowens/codenames))

## prerequisites

- rust: https://www.rust-lang.org/tools/install
- node: https://nodejs.dev/learn/how-to-install-nodejs
- awscli: 
  - https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-install.html
  - https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html
- jq: https://stedolan.github.io/jq/

See the [Makefile](Makefile) to run the various local development tasks.  `help` is the default goal and will print descriptions of the various commands:

```shell
make

Usage:
  make 

Backend (Rust REST Service)
  build-service    compile the rust REST service project
  format-service   run code formatting on the rust project
  test-service     run unit tests on the rust project
  check-service    check the rust project (format, build, and unit test)
  start-service    start the rust project locally using cargo run
  integration-test-service  run the newman integration test suite on the service
  integration-test  as of now, there are only integration tests for the service

Frontend (Typescript/NextJS Web App)
  build-app        build the web UI
  format-app       lint the web UI
  check-app        build and lint the web UI
  start-app        start the web UI locally

Check, Build, and Start both
  check            check both the service and app projects
  build            build both the service and app projects
  start            start fully functioning stack locally via docker

Infrastructure (AWS CDK)
  deploy-infra     deploy AWS infrastructure
  destroy-infra    tear down AWS infrastructure
  build-service-image  build docker image for the service - CAREFUL - this needs to be done from an x86 powered machine (not Apple M1 silicon)
  build-app-image  build docker image for the web ui
  build-images     build both images
  ecr-login        login into AWS ECR for docker
  push-service-image  push service image
  push-app-image   push app image
  push-images      push both images
  start-aws        start fully functioning stack on EC2

Help
  help             Display this help
```

## subprojects

Each subproject contains its own makefile.  For simplicity and readability, many of those commands are exposed to the top level Makefile, but not all.
You can execute `make` or `make help` in any of the below subdirectories to view descriptions of various commands.

### service
The [service](./service) directory contains the REST service supporting gameplay, written in Rust.

You will notice three subprojects within the service directory.  They are as follows

#### domain
This is a rust library containing the core business logic representing a game of codenames.

#### local
A better name for this would be `actix-web`.  This subproject is an actix-web server implementation of the domain.

#### wasm
The ultimate goal is to run this project as a [wasmCloud actor](https://wasmcloud.dev/).  This subproject is a wasmCloud implementation of the domain.

### app
The [app](./app) directory contains the web based user interface, written in React with NextJS.

### infra
The [infra](./infra) directory contains the AWS deployment for the project, written in Typescript with the AWS CDK.
