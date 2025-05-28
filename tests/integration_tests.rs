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

        let pt = Geometry::Point(point!(x: -44., y: -22.));
        let encoded = encode_geometry(&pt);
        let decoded = decode_geometry(&encoded);
        assert_eq!(pt, decoded);
    }

    #[test]
    fn test_linestring_encoding() {
        let ls = Geometry::LineString(line_string![
            (x: 1.0, y: 1.0),
            (x: 5.0, y: 5.0)
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

    #[test]
    fn test_serialization() {
        let serializer = HilbertSerializer::new();

        // Point
        let point = Geometry::Point(point![
            x: 1.0, y: 1.0
        ]);
        let encoded = serializer.encode(&point).unwrap();
        let decoded = serializer.decode(&encoded).unwrap();
        println!("Encoded point to {} bytes.", encoded.len(),);
        assert_eq!(point, decoded);

        // Linestring
        let ls = Geometry::LineString(line_string![
            (x: 1.0, y: 1.0),
            (x: 5.0, y: 5.0)
        ]);
        let encoded = serializer.encode(&ls).unwrap();
        let decoded = serializer.decode(&encoded).unwrap();
        println!("Encoded linestring to {} bytes.", encoded.len(),);
        assert_eq!(ls, decoded);

        // Polygon
        let poly = Geometry::Polygon(polygon![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 0.0),
            (x: 1.0, y: 1.0),
            (x: 0.0, y: 1.0),
            (x: 0.0, y: 0.0)
        ]);
        let encoded = serializer.encode(&poly).unwrap();
        let decoded = serializer.decode(&encoded).unwrap();
        println!("Encoded polygon to {} bytes.", encoded.len(),);
        assert_eq!(poly, decoded);
    }
}
