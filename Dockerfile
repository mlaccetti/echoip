FROM rust:1.50

WORKDIR /echoip

RUN echo 'fn main() {}' | tee dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo build --release

CMD ["target/release/echoip"]

FROM rust:1.50-slim as runtime

WORKDIR /echoip

COPY --from=builder /echoip/target/release/echoip /usr/local/bin
ENTRYPOINT ["./usr/local/bin/echoip"]
