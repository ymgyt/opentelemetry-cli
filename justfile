# List recipes
@default:
    just --list

# Run check
[group("check")]
check: fmt-check typo lint audit test

# Check format
[private]
fmt-check:
    cargo fmt --check

# Run typos
[group("check")]
typo:
    typos .

# Run linter
[group("check")]
lint:
    cargo clippy --workspace --all-targets --all-features --no-deps --locked -- --deny warnings

# Run audit
[group("check")]
audit:
    cargo audit

# Run test
[group("check")]
test *flags:
    cargo nextest run --no-tests=pass --all-features {{flags}}

# Detect unused packages
[group("check")]
machete:
    cargo machete
