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

publish-core:
    cargo publish --package oysters_core --allow-dirty

publish-server:
    cargo publish --package oysters --allow-dirty

publish-client:
    cargo publish --package oysters_client --allow-dirty

publish-cli:
    cargo publish --package oysters_cli --allow-dirty

publish-all:
    just publish-core
    just publish-server
    just publish-client
    just publish-cli
