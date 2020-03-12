GENERAL_ARGS = --release
FRONTEND_ARGS = $(GENERAL_ARGS) -p webapp-frontend --target wasm32-unknown-unknown
BACKEND_ARGS = $(GENERAL_ARGS) -p webapp-backend

.PHONY: \
	build-backend \
	build-frontend \
	lint-rustfmt \
	lint-clippy \
	run-backend \
	run-frontend

all: build-backend build-frontend

build-backend:
	cargo build $(BACKEND_ARGS)

build-frontend:
	cargo web build $(FRONTEND_ARGS)

lint-clippy:
	cargo clippy --all -- -D warnings

lint-rustfmt:
	cargo fmt
	git diff --exit-code

run-backend:
	cargo run $(BACKEND_ARGS)

run-frontend:
	cargo web start $(FRONTEND_ARGS) --auto-reload --host 0.0.0.0
