.PHONY: build install test clean dev

# Build Rust components
build:
	cd rust && cargo build --release

# Install Python package
install:
	cd python && pip install -e .

# Install Rust CLI
install-cli:
	cd rust/fluxreel-cli && cargo install --path .

# Run tests
test:
	pytest tests/
	cd rust && cargo test

# Development setup
dev:
	cd rust && cargo build
	cd python && pip install -e . --no-build-isolation

# Clean build artifacts
clean:
	cd rust && cargo clean
	rm -rf python/build python/dist python/*.egg-info
	rm -rf studio/node_modules studio/dist

# Format code
fmt:
	cd rust && cargo fmt
	cd python && black fluxreel/ tests/

# Lint code
lint:
	cd rust && cargo clippy
	cd python && flake8 fluxreel/ tests/

