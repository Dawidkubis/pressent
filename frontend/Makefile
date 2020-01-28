build: src/*
	cargo build --release
	wasm-pack build --target web

dev: src/*
	cargo build
	wasm-pack build --target web --dev

.PHONY: all build
