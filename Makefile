NAME=hello-rust
VERSION=0.0.1
IMAGE_DEV=$(NAME)-dev

image-dev:
	docker build -t $(IMAGE_DEV):$(VERSION) -f Dockerfile.dev .

dev-env:
	docker run -d --rm \
		--name $(IMAGE_DEV) \
		-v $(PWD):/app \
		-w /app \
		$(IMAGE_DEV):$(VERSION) \
		tail -f /dev/null