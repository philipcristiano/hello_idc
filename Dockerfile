FROM rust:1.76-bookworm as builder
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release
RUN cargo install --path .


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y procps ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hello_idc /usr/local/bin/hello_idc

ENTRYPOINT ["/usr/local/bin/hello_idc"]
