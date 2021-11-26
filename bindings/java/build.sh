set -e
WORK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export WORK_DIR
RUST_MANIFEST="./src/main/tokenizers-jna/Cargo.toml"
cargo update --manifest-path $RUST_MANIFEST
cargo build --manifest-path $RUST_MANIFEST

