# cargo install cargo-atcoder が Rust 2021 (1.56.0~) へ依存するため、1.42.0 ではなく新しいバージョンを使う
FROM rust:1.66.0-slim-bullseye

RUN apt-get update \
  && apt-get install --no-install-recommends -y \
  # 後続の Rust 関連コマンドに必要だった
  openssl pkg-config libssl-dev \
  # cargo atcoder new は git の設定を利用する（Cargo の仕様？）
  git ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* \
  && cargo install cargo-atcoder \
  # rust-analyzer の動作には 1.48.0 以上が必要らしい
  && rustup toolchain add 1.48.0 \
  && rustup default 1.48.0 \
  && rustup component add rustfmt clippy

COPY ./cargo-atcoder.toml /root/.config/cargo-atcoder.toml
