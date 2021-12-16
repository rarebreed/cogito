#!/bin/sh -xe
PACKAGE=""
RUST_TARGET="x86_64-unknown-linux-gnu"
RUST_BUILD_VERSION=""
ZIP_FILE="lambda.zip"

POSITIONAL=()
while [[ $# -gt 0 ]]; do
  key="$1"

  case $key in
    -t |--target)
      RUST_TARGET="$2"
      shift
      shift
      ;;
    -v |--version)
      RUST_BUILD_VERSION="$2"
      shift # past argument
      shift # past value
      ;;
    --toolchain)
      RUST_TOOLCHAIN="$2"
      shift # past argument
      shift # past value
      ;;
    -p |--package)
      PACKAGE="$2"
      shift # past argument
      shift
      ;;
    -z |--zip)
      ZIP_FILE="$2"
      shift
      shift
      ;;
    *)
      POSITIONAL+=("$1") # save it in an array for later
      shift # past argument
      ;;
  esac
done

rustup target add $RUST_TARGET

if [ ! -z "$RUST_BUILD_VERSION" ]; then
  rustup update $RUST_BUILD_VERSION
  rustup default $RUST_BUILD_VERSION
  rustup show
fi

if [ "$RUST_TOOLCHAIN" != "stable" ]; then
  rustup toolchain install $RUST_TOOLCHAIN
  rustup default $RUST_TOOLCHAIN
  rustup show
fi

cargo clean
cargo build --release --package $PACKAGE --target $RUST_TARGET
cp target/$RUST_TARGET/release/$PACKAGE bootstrap
zip "$ZIP_FILE" bootstrap
ls -al