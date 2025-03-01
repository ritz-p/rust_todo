FROM rust:1.85

RUN apt-get update && \
    apt-get install -y vim clang cmake libssl-dev build-essential \
    postgresql-client libwebkit2gtk-4.1-dev curl wget \
    file libssl-dev libxdo-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
    x11-apps gnupg ca-certificates fonts-noto-cjk

COPY rust-toolchain.toml .
RUN rustup update && \
    rustup update nightly
RUN rustup component add rls rust-analysis rust-src clippy && \
    rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt && \
    cargo install cargo-edit cargo-watch cargo-make sqlx-cli tauri-cli create-tauri-app && \
    rustup install nightly
RUN cargo install trunk --locked

RUN rustup target add wasm32-unknown-unknown

RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt update && \
    apt install -y yarn
RUN echo 'alias cm="cargo make"' >> ~/.bashrc
RUN echo 'alias tauri="cargo tauri"' >> ~/.bashrc
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1
ENV LANG=ja_JP.UTF-8
ENV LC_CTYPE=ja_JP.UTF-8