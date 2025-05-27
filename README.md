# Hilbert Geometry

Encodes `geo-types` geometries using hilbert encoding. Currently only supports lon/lat coordinates. Uses `bincode` for binary serialization.

Note: compression is **lossy**, translating to around 0.1 meter real-world loss of precision.

Usage:

```
use geo_types::{polygon, Geometry};
use hilbert_geometry::HilbertSerializer;

let serializer = HilbertSerializer::new();
let poly = Geometry::Polygon(polygon![
    (x: 0.0, y: 0.0),
    (x: 1.0, y: 0.0),
    (x: 1.0, y: 1.0),
    (x: 0.0, y: 1.0),
    (x: 0.0, y: 0.0)
]);
let encoded: Vec<u8> = serializer.encode(&poly).unwrap();
let decoded: Geometry = serializer.decode(&encoded).unwrap();
```

# Compression examples

| Geometry   | Hilbert Geometry | WKB      |
| ---------- | ---------------- | -------- |
| Point      | 9 bytes          | 21 bytes |
| Linestring | 18 bytes         | 41 bytes |
| Polygon    | 43 bytes         | 93 bytes |
