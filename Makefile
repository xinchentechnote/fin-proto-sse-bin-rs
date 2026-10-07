# Makefile for fin-protoc project

# Variables
PROTO_DSL := proto/sse_bin_v0.57.pdsl
OUTPUT_DIR := src/
BIN_DIR := ~/workspace/fin-protoc/bin/

.PHONY: all build compile fmt fix test help

all: compile test

build: compile

compile:
	@echo "Compiling protocol..."
	$(BIN_DIR)/fin-protoc -f $(PROTO_DSL) -r $(OUTPUT_DIR)
	cargo fix --allow-dirty --allow-staged --all-targets
	cargo fmt

fmt:
	@echo "Formatting code..."
	cargo fmt

fix:
	@echo "Applying fixes..."
	cargo fix --allow-dirty --allow-staged --all-targets

test:
	@echo "Running tests..."
	cargo nextest run

# Help target
help:
	@echo "Available targets:"
	@echo "  all       - Run compile, format and test (default)"
	@echo "  compile   - Regenerate src/ from the DSL in proto/ via fin-protoc"
	@echo "  fmt       - Format the code using cargo fmt"
	@echo "  fix       - Apply automatic fixes using cargo fix"
	@echo "  test      - Run tests using cargo nextest"
	@echo "  help      - Show this help message"
