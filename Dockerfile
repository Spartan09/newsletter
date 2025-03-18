FROM rust:1.85.0

WORKDIR /app

RUN apt-get update && apt-get install lld clang mold -y

COPY . .

RUN cargo build --release

ENTRYPOINT [ "./target/release/newsletter" ]
