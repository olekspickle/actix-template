.PHONY: docs test lint build build-release tag run-local

.ONESHELL: # Use one shell per target
SHELL := /bin/bash
# Stop excecution on any error
.SHELLFLAGS = -ec

crate=actix-template

docs:
	cargo docs

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

build:
	cargo fetch
	cargo build --target=x86_64-unknown-linux-musl

build-release:
	cargo fetch
	cargo build --target=x86_64-unknown-linux-musl --release

pack: build
	# TODO: query crate name with
	# cargo pkgid | rev | cut -d'/' -f1 | rev | sed 's/#.*//'
	docker build -t $(crate):local .

tag: pack
	# TODO: user can be sub-d with
	# git config --get user.name | cut -d " " -f 1
	# and version with
	# cargo pkgid | grep -oP '#\K[^#]+$'
	docker tag $(crate):local olekspickle/$(crate):v0.1.0

log_level=RUST_LOG=info,actix_template=trace

run-local:
	$(log_level) cargo run

run-docker-restricted: pack
	docker run -d \
		-p 7777:7777 \
		--hostname $(crate) \
		--cpus="0.25" --memory="0.5g" \
		-e $(log_level) \
		$(crate):local
