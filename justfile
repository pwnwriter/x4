_default:
    @just -l

alias t := test
alias f := fmt
alias r := run

# List all generations
test:
    @cargo test

# Run cargo fmt across source
fmt:
    @cargo fmt --manifest-path ./Cargo.toml --all --check

# Run cargo fmt across source
run:
    @cargo run -- --pipeline ./examples/sxm.json

# Run cargo clippy across source
clippy:
    @cargo clippy -- --deny warnings
