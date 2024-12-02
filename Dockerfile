FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY wql_queries ./wql_queries

RUN cargo build --release

EXPOSE 29000

CMD ["./target/release/sensex_nexus"]
