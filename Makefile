.PHONY: install install-frontend install-rust dev dev-rust demo docs release test-rust test-frontend test-e2e validate bench-rust

install: install-frontend

install-frontend:
	cd frontend && npm install
	cd frontend && npx playwright install chromium --with-deps

dev:
	cargo run --manifest-path common/cli/Cargo.toml -- dev --surface web --host 127.0.0.1 --port 5173

# --- Rust backend ---
install-rust:
	rustup target add x86_64-unknown-linux-musl
	cargo install cargo-zigbuild
	brew install zig 2>/dev/null || true

dev-rust:
	cd server && cargo run --bin revi

build-macos:
	cd server && cargo build --release --target aarch64-apple-darwin
	mkdir -p dist
	cp server/target/aarch64-apple-darwin/release/revi dist/revi-macos-aarch64

build-linux:
	cd server && cargo zigbuild --release --target x86_64-unknown-linux-musl
	mkdir -p dist
	cp server/target/x86_64-unknown-linux-musl/release/revi dist/revi-linux-x86_64

demo:
	cargo run --manifest-path common/cli/Cargo.toml -- demo

docs:
	cargo run --manifest-path common/cli/Cargo.toml -- docs

release:
	bash scripts/build-release.sh

test-rust:
	cd server && cargo test

test-frontend:
	cd frontend && npm test

test-e2e:
	cd frontend && npm run test:e2e

validate:
	bash scripts/validate-rust-vue.sh

bench-rust:
	cd server && cargo bench
