FROM rust:1.88-bookworm as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock /usr/src/app/
RUN \
    mkdir /usr/src/app/src && \
    echo 'fn main() {}' > /usr/src/app/src/main.rs && \
    cargo build --release && \
    rm -Rvf /usr/src/app/src

COPY . .
RUN touch src/main.rs
RUN cargo build --release -v
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y procps ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hello_idc /usr/local/bin/hello_idc

ENTRYPOINT ["/usr/local/bin/hello_idc"]
