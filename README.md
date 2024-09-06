# what-osm-pbf [WOP]
Get the necessary `osm.pbf` files within a bounding box.

## Installation
```bash
cargo install what-osm-pbf
```

## Levels
The levels represent the granularity of the areas that will be returned. The levels are as follows:

- `level=0` -> the most granular. It will find the smallest areas possible. In some countries such as England this is **likely** be at county level.
- `level=1` -> **likely** to be regional areas within a country.
- `level=2` -> **likely** to be the at country level.
- `level=3` -> the least granular. **Likely** to be at continent level.

## Example 1
Basic with `level=2`
```bash
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=2

# Output:
#
# ✅ Found 3 osm.pbf files that intersect
# 
# Name: "Algeria", Link: "https://download.geofabrik.de/africa/algeria-latest.osm.pbf"
# Name: "Portugal", Link: "https://download.geofabrik.de/europe/portugal-latest.osm.pbf"
# Name: "Spain", Link: "https://download.geofabrik.de/europe/spain-latest.osm.pbf"
```

## Example 2
Save results to JSON using `level=3` for least granular.
```bash
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=3 \
    --savejson

# Output:
#
# ✅ Found 2 osm.pbf files that intersect
# 
# Name: "Africa", Link: "https://download.geofabrik.de/africa-latest.osm.pbf"
# Name: "Europe", Link: "https://download.geofabrik.de/europe-latest.osm.pbf"
#
# 🎉 Results saved to what-osm-pbf.json
```

## Example 3
Download the results using `level=0` for most granular.
```bash
wop --bbox=-7.382813,36.129002,-1.702881,38.410558 \
    --level=0 \
    --download

# Output:
#
# ✅ Found 6 osm.pbf files that intersect
# 
# Name: "Algeria", Link: "https://download.geofabrik.de/africa/algeria-latest.osm.pbf"
# Name: "Portugal", Link: "https://download.geofabrik.de/europe/portugal-latest.osm.pbf"
# Name: "Andalucía", Link: "https://download.geofabrik.de/europe/spain/andalucia-latest.osm.pbf"
# Name: "Castilla-La Mancha", Link: "https://download.geofabrik.de/europe/spain/castilla-la-mancha-latest.osm.pbf"
# Name: "Extremadura", Link: "https://download.geofabrik.de/europe/spain/extremadura-latest.osm.pbf"
# Name: "Murcia", Link: "https://download.geofabrik.de/europe/spain/murcia-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/africa/algeria-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/algeria-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/europe/portugal-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/portugal-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/europe/spain/andalucia-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/andalucia-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/europe/spain/castilla-la-mancha-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/castilla-la-mancha-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/europe/spain/extremadura-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/extremadura-latest.osm.pbf"
# 
# Downloading: "https://download.geofabrik.de/europe/spain/murcia-latest.osm.pbf"
# ✓ Downloaded: "/Users/frankjimenez/repositories/what-osm-pbf/target/release/murcia-latest.osm.pbf"

```

## License
See [`LICENSE`](./LICENSE)
