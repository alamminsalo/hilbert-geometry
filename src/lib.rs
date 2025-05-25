use geo_types::{Coord, Geometry, LineString, Point, Polygon};
use hilbert_2d::{h2xy_continuous_f64, xy2h_continuous_f64, Variant};

const HILBERT_VARIANT: Variant = Variant::Hilbert;
const PRECISION: i32 = 7;

/// Represents a Hilbert-encoded point.
#[derive(Debug, Clone, Copy)]
pub struct HilbertPoint(pub f64);

/// Represents a Hilbert-encoded geometry.
#[derive(Debug, Clone)]
pub enum HilbertGeometry {
    Point(HilbertPoint),
    LineString(Vec<HilbertPoint>),
    Polygon(Vec<Vec<HilbertPoint>>), // Outer ring and inner rings
}

/// Encodes a 2D coordinate into a Hilbert index.
fn encode_coord(coord: Coord<f64>) -> HilbertPoint {
    HilbertPoint(xy2h_continuous_f64(coord.x, coord.y, HILBERT_VARIANT))
}

/// Decodes a Hilbert index back into a 2D coordinate.
fn decode_coord(p: HilbertPoint) -> Coord<f64> {
    let (x, y) = h2xy_continuous_f64(p.0, HILBERT_VARIANT);
    Coord {
        x: (x * 10f64.powi(PRECISION)).round() / 10f64.powi(PRECISION),
        y: (y * 10f64.powi(PRECISION)).round() / 10f64.powi(PRECISION),
    }
}

/// Encodes a `geo-types` geometry into a Hilbert-encoded geometry.
pub fn encode_geometry(geom: &Geometry<f64>) -> HilbertGeometry {
    match geom {
        Geometry::Point(pt) => HilbertGeometry::Point(encode_coord(pt.0)),
        Geometry::LineString(ls) => {
            let encoded = ls.points().map(|p| encode_coord(p.0)).collect();
            HilbertGeometry::LineString(encoded)
        }
        Geometry::Polygon(poly) => {
            let exterior = poly
                .exterior()
                .points()
                .map(|p| encode_coord(p.0))
                .collect::<Vec<HilbertPoint>>();
            let interiors = poly
                .interiors()
                .iter()
                .map(|ring| {
                    ring.points()
                        .map(|p| encode_coord(p.0))
                        .collect::<Vec<HilbertPoint>>()
                })
                .collect::<Vec<Vec<HilbertPoint>>>();
            HilbertGeometry::Polygon(vec![vec![exterior], interiors].concat())
        }
        _ => unimplemented!("Geometry type not supported"),
    }
}

/// Decodes a Hilbert-encoded geometry back into a `geo-types` geometry.
pub fn decode_geometry(hgeom: &HilbertGeometry) -> Geometry<f64> {
    match hgeom {
        HilbertGeometry::Point(hp) => {
            let coord = decode_coord(*hp);
            Geometry::Point(Point(coord))
        }
        HilbertGeometry::LineString(hps) => {
            let coords = hps.iter().map(|hp| decode_coord(*hp)).collect();
            Geometry::LineString(LineString(coords))
        }
        HilbertGeometry::Polygon(rings) => {
            if rings.is_empty() {
                return Geometry::Polygon(Polygon::new(LineString::new(vec![]), vec![]));
            }
            let exterior = LineString(rings[0].iter().map(|hp| decode_coord(*hp)).collect());
            let interiors = rings[1..]
                .iter()
                .map(|ring| LineString(ring.iter().map(|hp| decode_coord(*hp)).collect()))
                .collect();
            Geometry::Polygon(Polygon::new(exterior, interiors))
        }
    }
}
