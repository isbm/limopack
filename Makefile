.DEFAULT_GOAL := build

.PHONY:build
build: man
	cargo build -v --release

build-dev:
	cargo build

man:
	pandoc --standalone --to man doc/limopak.8.md -o doc/limopak.8
