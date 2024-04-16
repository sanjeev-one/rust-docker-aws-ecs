# Makefile for building and running a Rust microservice Docker container

# Define variables for docker commands
IMAGE_NAME=rust-microservice
CONTAINER_NAME=my-rust-app

.PHONY: build run stop clean

build:
	@echo "Building the Docker image..."
	cd ..; docker build -t $(IMAGE_NAME) .

run:
	@echo "Running the Docker container..."
	docker run -d -p 8080:8080 --name $(CONTAINER_NAME) $(IMAGE_NAME)

stop:
	@echo "Stopping the Docker container..."
	docker stop $(CONTAINER_NAME)

remove:
	@echo "Removing the Docker container..."
	docker rm $(CONTAINER_NAME)

clean: stop remove
	@echo "Cleaned up the Docker container."

