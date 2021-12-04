#!/bin/sh -xe
PACKAGE=$1
RUST_TARGET=$2

if [ -z "$RUST_TARGET" ]; then
  RUST_TARGET=x86_64-unknown-linux-gnu
fi

cargo build --release --package $PACKAGE --target $RUST_TARGET
cp target/$RUST_TARGET/release/$PACKAGE bootstrap
zip lambda.zip bootstrap