@try-build:
    cargo build --release
    cd ./target/release/ && ./wop --bbox=-7.382813,36.129002,-1.702881,38.410558 --download

@try-andalucia-level3:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=3 --download

@try-andalucia-level2:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=2 --download

@try-andalucia-level1:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=1 --download

@try-andalucia-level0:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level=0
