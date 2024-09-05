# what-osm-pbf [WOP]
Get the necessary `osm.pbf` files within a bounding box.

## Installation
```bash
cargo install what-osm-pbf
```

#### Usage examples
```bash
## Print out results
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=0

## Save results to JSON
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=0
    --savejson

## Download the results
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=1 \
    --download
```

## License
See [`LICENSE`](./LICENSE)