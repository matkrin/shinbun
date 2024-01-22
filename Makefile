.PHONY: dev
dev:
	cargo tauri dev

.PHONY: build
build:
	cargo tauri build

.PHONY: install
install:
	cargo tauri build && cd src-tauri && cargo install --path . --features custom-protocol
