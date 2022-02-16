FROM rust:1.57.0-slim-bullseye

RUN rustup component add rustfmt clippy
