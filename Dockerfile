FROM rust:1.57-slim-bullseye

LABEL org.opencontainers.image.authors="michael@laccetti.com"

EXPOSE 8088

WORKDIR /echoip

COPY target/x86_64-unknown-linux-musl/release/echoip .
COPY static static
COPY templates templates
COPY geoip geoip

CMD ["./echoip"]
