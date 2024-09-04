@try-build:
    cargo build --release
    cd ./target/release/ && ./wop --bbox=-7.382813,36.129002,-1.702881,38.410558 --download

@try-level3:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=3 --download

@try-level2:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=2 --download

@try-level1:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=1 --download

@try-level0:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=0 --download
