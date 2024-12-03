FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY wql_queries ./wql_queries
COPY wql_templates ./wql_templates
COPY .env ./.env

RUN cargo build --release

EXPOSE 29000

CMD ["./target/release/sensex_nexus"]
