#[cfg(test)]
mod tests {
    use geo_types::{line_string, point, polygon, Geometry};
    use hilbert_geometry::*;

    #[test]
    fn test_point_encoding() {
        let pt = Geometry::Point(point!(x: 0.5, y: 0.5));
        let encoded = encode_geometry(&pt);
        let decoded = decode_geometry(&encoded);
        assert_eq!(pt, decoded);
    }

    #[test]
    fn test_linestring_encoding() {
        let ls = Geometry::LineString(line_string![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 1.0)
        ]);
        let encoded = encode_geometry(&ls);
        let decoded = decode_geometry(&encoded);
        assert_eq!(ls, decoded);
    }

    #[test]
    fn test_polygon_encoding() {
        let poly = Geometry::Polygon(polygon![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 0.0),
            (x: 1.0, y: 1.0),
            (x: 0.0, y: 1.0),
            (x: 0.0, y: 0.0)
        ]);
        let encoded = encode_geometry(&poly);
        let decoded = decode_geometry(&encoded);
        assert_eq!(poly, decoded);
    }
}
