FROM rust:1-buster

RUN set eux; \
    apt-get update -yqq; \
    apt-get install -yqq \
        librust-pango-dev \
        librust-atk-sys-dev \
        librust-gdk-dev

WORKDIR /usr/src/library-loader
