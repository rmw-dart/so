#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

flutter_rust_bridge_codegen --rust-input rust/src/lib.rs --dart-output dart/lib/rust.dart
