FROM rust:1.79

RUN apt update && \
    apt install -y vim clang cmake libssl-dev build-essential \
    postgresql-client libwebkit2gtk-4.1-dev curl wget \
    file libssl-dev libxdo-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
    x11-apps gnupg ca-certificates

COPY rust-toolchain.toml .
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch cargo-make sqlx-cli tauri-cli create-tauri-app && \
    rustup install nightly
RUN cargo install trunk --locked

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
    && apt-get update && apt-get install -y yarn
RUN rustup target add wasm32-unknown-unknown
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1
