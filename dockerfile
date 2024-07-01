FROM rust:latest

RUN apt update && \
    apt install vim clang cmake libssl-dev build-essential postgresql-client -y
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch cargo-make sqlx-cli && \
    rustup install nightly
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1