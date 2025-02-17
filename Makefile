.PHONY: run
run:
	@echo "Running the application"
	cargo run

.PHONY: watch
watch:
	@echo "Watch application update"
	cargo watch -q -c -w src/ -x "run"

.PHONY: build
build:
	@echo "build the application"
	cargo build

.PHONY: test 
test:
	@echo "Run unit test"
	cargo test

.PHONY: lint 
lint:
	@echo "Lint source code"
	cargo clippy

.PHONY: audit
audit:
	@echo "Audit vulnerability"
	cargo audit

.PHONY: check
check:
	@echo "Check application code"
	cargo check

.PHONY: help
help:
	@echo "Available commands:"
	@echo " make run          - Run the application"
	@echo " make watch        - Watch code update"
	@echo " make build        - Build the application"
	@echo " make test         - Run all test cases"
	@echo " make lint         - Lint the source code"
	@echo " make audit        - Audit application vulnerability"
	@echo " make check        - Check the application without build"
