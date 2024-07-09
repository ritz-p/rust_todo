FROM rust:latest

RUN apt update && \
    apt install -y vim clang cmake libssl-dev build-essential \
    postgresql-client libwebkit2gtk-4.0-dev curl wget \
    file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
    x11-apps

RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch cargo-make sqlx-cli tauri-cli trunk && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown
RUN cargo install create-tauri-app --locked
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1