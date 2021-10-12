DOCKER ?= docker
DOCKER_IMAGE ?= mlaccetti/echoip

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
