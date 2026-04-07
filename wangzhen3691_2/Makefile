.PHONY: all build run dev frontend frontend-dev clean test bench

all: frontend build

# ---- Go backend ----
build:
	@echo "==> Building shihuang-guard…"
	go build -o bin/shihuang-guard ./cmd/server

run: build
	./bin/shihuang-guard

dev:
	go run ./cmd/server

test:
	go test ./...

# ---- Frontend ----
frontend:
	cd web && npm install && npm run build

frontend-dev:
	cd web && npm run dev

# ---- Tools ----
bench:
	node tools/benchmark.js

webhook:
	node tools/webhook-receiver.js

# ---- Cleanup ----
clean:
	rm -rf bin/ web/dist/ web/node_modules/ data/
