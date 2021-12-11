#!/bin/sh -xe
PACKAGE=$1

if [ -z "$RUST_TARGET" ]; then
  RUST_TARGET=x86_64-unknown-linux-gnu
fi

rustup target add $RUST_TARGET

if [ ! -z "$RUST_BUILD_VERSION" ]; then
  rustup update $RUST_BUILD_VERSION
  rustup default $RUST_BUILD_VERSION
  rustup show
fi

cargo build --release --package $PACKAGE --target $RUST_TARGET
cp target/$RUST_TARGET/release/$PACKAGE bootstrap
zip lambda.zip bootstrap