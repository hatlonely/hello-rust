NAME=hello-rust
VERSION=0.0.1
IMAGE_DEV=$(NAME)-dev

image-dev:
	docker build -t $(IMAGE_DEV):$(VERSION) -f Dockerfile.dev .

dev-env:
	docker run -d --rm \
		--network host \
		-v $(PWD):/app \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-w /app \
		--name $(IMAGE_DEV) \
		$(IMAGE_DEV):$(VERSION) \
		tail -f /dev/null