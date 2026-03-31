.PHONY: install install-backend install-frontend install-rust dev dev-rust build-macos build-linux test-rust bench-rust

install: install-backend install-frontend

install-backend:
	cd backend && pip install -e ".[dev]"

install-frontend:
	cd frontend && npm install
	cd frontend && npx playwright install chromium --with-deps

dev:
	@echo "Start backend:  uvicorn backend.main:app --reload --port 8000"
	@echo "Start frontend: cd frontend && npm run dev"

# --- Rust backend ---
install-rust:
	rustup target add x86_64-unknown-linux-musl
	cargo install cargo-zigbuild
	brew install zig 2>/dev/null || true

dev-rust:
	cd server && cargo run --bin revi -- --workspace ../backend/workspace --data ../backend/data

build-macos:
	cd server && cargo build --release --target aarch64-apple-darwin
	mkdir -p dist
	cp server/target/aarch64-apple-darwin/release/revi dist/revi-macos-aarch64

build-linux:
	cd server && cargo zigbuild --release --target x86_64-unknown-linux-musl
	mkdir -p dist
	cp server/target/x86_64-unknown-linux-musl/release/revi dist/revi-linux-x86_64

test-rust:
	cd server && cargo test

bench-rust:
	cd server && cargo bench
