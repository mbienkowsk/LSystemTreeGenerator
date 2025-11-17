# Lint the code
lint:
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic

# Build the project (Release mode)
build:
    cargo build --release

# Run tests
test:
    cargo test --all-features --verbose

# Run the app
run:
    RUST_LOG=info cargo run


# Generate HTML test coverage report in target/coverage/html
# Requires cargo-binutils, grcov and rustup llvm-tools to be installed
cov:
    [ -d target/coverage/html ] && rm -r target/coverage/html || true
    CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
    rm *profraw
