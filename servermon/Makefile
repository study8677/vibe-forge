.PHONY: all build server agent web clean dev

all: build

# Build everything
build: server agent web

# Build Go server
server:
	go build -o bin/server ./cmd/server

# Build Go agent
agent:
	go build -o bin/agent ./cmd/agent

# Build frontend
web:
	cd web && npm install && npm run build

# Run server (dev)
run: server
	./bin/server -addr :8080

# Run frontend dev server (with proxy to Go backend)
dev-web:
	cd web && npm run dev

# Cross compile agent for Linux
agent-linux:
	GOOS=linux GOARCH=amd64 go build -o bin/agent-linux-amd64 ./cmd/agent
	GOOS=linux GOARCH=arm64 go build -o bin/agent-linux-arm64 ./cmd/agent

# Clean build artifacts
clean:
	rm -rf bin/ web/dist/ web/node_modules/
