default:
    just --list

gen-css:
    deno -A ./gen-css.ts

dev:
    cargo tauri dev

build:
    cargo tauri build

install:
    cargo tauri build && cd src-tauri && cargo install --path . --features custom-protocol
