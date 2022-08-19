DOCKER ?= docker
DOCKER_IMAGE ?= mlaccetti/echoip
SHELL := /bin/bash

OS := $(shell uname)
ifeq ($(OS),Linux)
	TAR_OPTS := --wildcards
endif

all: lint test build

test:
	cargo check

check-fmt:
	cargo fmt

lint: check-fmt
	cargo clippy

build:
	cargo build

local-release:
	cargo build --release

docker-build:
	docker build . --target build --tag "echoip:build"

docker:
	docker build . --tag "echoip:latest"
	docker tag "echoip:latest" "ghcr.io/mlaccetti/echoip:latest"
	docker tag "ghcr.io/mlaccetti/echoip:latest" "northamerica-northeast2-docker.pkg.dev/laccetti-193216/echoip/echoip:latest"

prep-env:
	rustup target add x86_64-unknown-linux-gnu
	brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu

release: prep-env
	TARGET_CC=x86_64-unknown-linux-gnu-musl cargo build --release --target x86_64-unknown-linux-musl

package: release
	zip echoip.zip -j target/x86_64-unknown-linux-musl/release/echoip
	zip -ur echoip.zip static/ templates/ geoip/

databases := GeoLite2-City GeoLite2-Country GeoLite2-ASN

$(databases):
ifndef GEOIP_LICENSE_KEY
	$(error GEOIP_LICENSE_KEY must be set. Please see https://blog.maxmind.com/2019/12/18/significant-changes-to-accessing-and-using-geolite2-databases/)
endif
	mkdir -p geoip
	@curl -fsSL -m 30 "https://download.maxmind.com/app/geoip_download?edition_id=$@&license_key=$(GEOIP_LICENSE_KEY)&suffix=tar.gz" | tar $(TAR_OPTS) --strip-components=1 -C $(CURDIR)/geoip -xzf - '*.mmdb'

geoip-download: $(databases)

run:
	cargo run
