.DEFAULT_GOAL := build

.PHONY:build
release: man
	cargo build -v --release

build:
	cargo build

man:
	pandoc --standalone --to man doc/limopack.8.md -o doc/limopack.8