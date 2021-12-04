FROM public.ecr.aws/lambda/provided:al2

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.57.0

ARG RUST_TARGET="x86_64-unknown-linux-gnu"
ARG EXTRAS=""
    
RUN mkdir /code; \
    yum groupinstall -y "Development Tools"; \
    yum install -y wget zip openssl-devel ${EXTRAS}; \
    url="https://static.rust-lang.org/rustup/archive/1.24.3/${RUST_TARGET}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${RUST_TARGET};

WORKDIR /code

CMD [ "cargo", "build", "--release" ]