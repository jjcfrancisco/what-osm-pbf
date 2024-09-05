@try-release-andalucia level download savejson:
    cargo build --release
    cd ./target/release/ && ./wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level={{ level }} \
    --download={{ download }} \
    --savejson={{ savejson }}

@try-release-uk level download savejson:
    cargo build --release
    cd ./target/release/ && ./wop --bbox=0.417480,49.979488,3.526611,51.781436 \ 
    --level={{ level }} \
    --download={{ download }} \
    --savejson={{ savejson }}

@try-debug-andalucia level:
    cargo run -- --bbox=-7.382813,36.129002,-1.702881,38.410558 --level={{ level }}

@try-debug-uk level:
    cargo run -- --bbox=-0.417480,49.979488,3.526611,51.781436 --level={{ level }}

