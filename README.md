# README for Rust Docker Web Service with CI/CD

## Overview

This repository contains a Rust web service built with Actix-Web, packaged in a Docker container, and deployed using CI/CD workflows via GitHub Actions to AWS Elastic Container Registry (ECR) and AWS App Runner.

The web service is a simple HTTP server responding to GET requests at the root (`/`) endpoint with a greeting message, "Hello, IDS 721!".

## Technology Stack

- **Rust**: Programming language used for building the web service.
- **Actix-Web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **Docker**: Used for packaging the application and its dependencies into a container.
- **GitHub Actions**: Automates the workflow to build, tag, and push the Docker image to AWS ECR upon code pushes to the main branch.
- **AWS ECR (Elastic Container Registry)**: Hosts the Docker images.
- **AWS App Runner**: Automatically deploys the Docker container and runs the web service.

## Code Structure

The main components of the code include:
- `greet`: An asynchronous function that returns an HTTP response.
- `main`: Sets up the Actix-Web HTTP server to listen on all interfaces at port 8080.

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, IDS 721!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

## CI/CD Pipeline

The CI/CD pipeline, defined in the GitHub Actions workflow, automates the process of building, tagging, and pushing the Docker image to AWS ECR upon any code push to the `main` branch. Here’s how the pipeline is structured:

1. **Checkout Code**: Retrieves the latest code from the `main` branch.
2. **Configure AWS Credentials**: Uses GitHub secrets to set up AWS credentials.
3. **Login to Amazon ECR**: Authenticates to the AWS ECR service.
4. **Build, Tag, and Push Image**: Builds the Docker image, tags it with 'latest', and pushes it to a specified repository in ECR.
5. **Print ECR Image URL**: Outputs the URL of the pushed Docker image for verification and tracking.

## Deployment

AWS App Runner pulls the Docker image from AWS ECR and runs the web service. The service automatically scales based on demand and handles all aspects of service deployment, operations, and scaling, including provisioning of resources, load balancing, scaling, and monitoring.

## Usage

After deployment, the service can be accessed via the provided endpoint from AWS App Runner, responding to HTTP GET requests at the root (`/`) endpoint.

This setup exemplifies how modern cloud-native development and deployment paradigms can be implemented using Rust, Docker, and AWS cloud services with robust CI/CD practices using GitHub Actions.