FROM rust:1.63-alpine3.16 as chef

WORKDIR /echoip

RUN apk update && apk add musl-dev && \
    cargo install cargo-chef

FROM chef AS plan

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build

RUN addgroup -g 10001 echoip && \
    adduser -u 10001 -G echoip -h /echoip -D echoip

COPY --from=plan /echoip/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM scratch as release
LABEL org.opencontainers.image.authors="michael@laccetti.com"

USER echoip
ENV RUST_LOG="error,echoip=info"
EXPOSE 8088

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /echoip/target/release/echoip /echoip/echoip

WORKDIR /echoip

COPY static static
COPY templates templates
COPY geoip geoip

CMD ["/echoip/echoip"]
