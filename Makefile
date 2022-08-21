ifneq ($(OS),Windows_NT)
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		TAR_OPTS := --wildcards
	endif
endif

DOCKER ?= docker
DOCKER_IMAGE ?= mlaccetti/echoip
SHELL := /bin/bash

all: lint test build

test:
	cargo check --verbose

check-fmt:
	cargo fmt

lint: check-fmt
	cargo clippy

build:
	cargo build --verbose

run:
	cargo run

docker-build:
	docker build . --target build --tag "echoip:build"

docker:
	docker build . --tag "echoip:latest"
	docker tag "echoip:latest" "ghcr.io/mlaccetti/echoip:latest"
	docker tag "ghcr.io/mlaccetti/echoip:latest" "northamerica-northeast2-docker.pkg.dev/laccetti-193216/echoip/echoip:latest"

databases := GeoLite2-City GeoLite2-Country GeoLite2-ASN

$(databases):
ifndef GEOIP_LICENSE_KEY
	$(error GEOIP_LICENSE_KEY must be set. Please see https://blog.maxmind.com/2019/12/18/significant-changes-to-accessing-and-using-geolite2-databases/)
endif
	mkdir -p geoip
	@curl -fsSL -m 30 "https://download.maxmind.com/app/geoip_download?edition_id=$@&license_key=$(GEOIP_LICENSE_KEY)&suffix=tar.gz" | tar $(TAR_OPTS) --strip-components=1 -C $(CURDIR)/geoip -xzf - '*.mmdb'

geoip-download: $(databases)
