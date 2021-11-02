FROM public.ecr.aws/lambda/provided:al2

# Install rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

ARG RUST_VERSION=1.56.1

RUN mkdir /code && \
    yum groupinstall -y "Development Tools" && \ 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain $RUST_VERSION


# Set a volume mount point that we can later copy out of
VOLUME ["/code"]
# Working directory for our code
WORKDIR /code

COPY aws-test /code

RUN cargo build --release && \
    cp ./target/release/aws-test ./bootstrap && \
    zip lambda.zip bootstrap && \
    ls -al ./bootstrap && \
    cp ./bootstrap ${LAMBDA_RUNTIME_DIR} && \
    ls -al /code/target/release

# Set the CMD to your handler (could also be done as a parameter override outside of the Dockerfile)
CMD [ "/bin/bash" ]