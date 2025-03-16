test:
    cargo run --bin oysters

clean-deps:
    cargo upgrade -i
    cargo machete

fix:
    cargo fix --allow-dirty
    cargo clippy --fix --allow-dirty

build:
    cargo build --bin oysters -r
    cargo build --bin oysters-cli -r
