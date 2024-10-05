_default:
    @just -l

# Run cargo test
test:
    @cargo test

# Run cargo fmt across source
fmt:
    @cargo fmt --manifest-path ./Cargo.toml --all --check

# Run cargo fmt across source
run:
    @cargo run -- --pipeline ./examples/many_servers.json

# Run cargo clippy across source
clippy:
    @cargo clippy -- --deny warnings

# Do a release of the project requires version specify
release VERSION:
    @git tag "v{{ VERSION }}"
    @git push origin "v{{ VERSION }}"
