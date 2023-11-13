# Rust Axum to Serverless

This is a simple example of how to deploy a Rust Axum application to AWS Lambda using the [Serverless Framework](https://www.serverless.com/).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Serverless Framework](https://www.serverless.com/framework/docs/getting-started/)
- [Docker](https://docs.docker.com/get-docker/)

## Working locally

```bash
bash scripts/dev
```

## Deploying to AWS

Deployment of the application is done using Docker as the build environment. This is to ensure that the application is built in a Linux environment, which is required for AWS Lambda.

Be sure to fix your `servrless.yml` file to match your AWS account.

```bash
bash scripts/deploy
```