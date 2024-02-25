FROM rust:1.76.0-buster AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release


FROM debian:buster-slim
WORKDIR /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/test-api .

CMD ["test-api"]
