GENERAL_ARGS = --release
BACKEND_ARGS = $(GENERAL_ARGS) -p webapp-backend

.PHONY: \
	build-backend \
	build-frontend \
	lint-rustfmt \
	lint-clippy \
	run-backend \
	run-frontend \
	clean

all: build-frontend build-backend

build-backend:
	cargo build $(BACKEND_ARGS)

build-frontend:
	wasm-pack build --target web frontend
	rollup ./frontend/main.js --format iife --file ./frontend/pkg/bundle.js
	cp ./frontend/pkg/bundle.js ./frontend/pkg/p2pc_bg.wasm ./frontend/static/app/
	brotli ./frontend/static/app/*

lint-clippy:
	cargo clippy --all -- -D warnings

lint-rustfmt:
	cargo fmt
	git diff --exit-code

run-backend:
	cargo run $(BACKEND_ARGS)

run-frontend: build-frontend
	python -m http.server -d ./frontend/static 8000

clean:
	cargo clean
	rm -rd ./frontend/pkg/* ./frontend/static/app/*
