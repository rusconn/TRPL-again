FROM rust:1.57.0-slim-bullseye

RUN apt-get update \
  && apt-get install --no-install-recommends -y git ca-certificates fish \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* \
  && rustup component add rustfmt clippy
