@try-build:
    cargo build --release
    cd ./target/release/ && ./wop --bbox=-7.382813,36.129002,-1.702881,38.410558 --download

@try:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --download

