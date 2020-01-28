build: src/*
	cargo build --release

dev: src/*
	cargo build

.PHONY: build dev
