FROM rust:latest

RUN mkdir /src

COPY ./rocked /src

VOLUME /src
RUN cargo build --release

