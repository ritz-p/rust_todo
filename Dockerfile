FROM rust:1.79

RUN apt-get update && \
    apt-get install -y vim clang cmake libssl-dev build-essential \
    postgresql-client libwebkit2gtk-4.1-dev curl wget \
    file libssl-dev libxdo-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
    x11-apps gnupg ca-certificates fonts-noto-cjk

COPY rust-toolchain.toml .
RUN rustup component add rls rust-analysis rust-src clippy && \
    rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt && \
    cargo install cargo-edit cargo-watch cargo-make sqlx-cli tauri-cli create-tauri-app && \
    rustup install nightly
RUN cargo install trunk --locked

RUN rustup target add wasm32-unknown-unknown

RUN echo 'alias cargo make="cm"' >> ~/.bashrc
RUN echo 'alial cargo tauri="tauri"' >> ~/.bashrc
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1
ENV LANG=ja_JP.UTF-8
ENV LC_CTYPE=ja_JP.UTF-8