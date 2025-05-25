use bincode::{
    config,
    config::Configuration,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use geo_types::{Coord, Geometry, LineString, Point, Polygon};
use hilbert_2d::{h2xy_continuous_f64, xy2h_continuous_f64, Variant};

const HILBERT_VARIANT: Variant = Variant::Hilbert;
const PRECISION: i32 = 7;

#[inline(always)]
fn round_decimal(v: f64) -> f64 {
    (v * 10f64.powi(PRECISION)).round() / 10f64.powi(PRECISION)
}

/// Represents a Hilbert-encoded point.
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct HilbertPoint(pub f64);

/// Represents a Hilbert-encoded geometry.
#[derive(Debug, Clone, Decode, Encode)]
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
        x: round_decimal(x),
        y: round_decimal(y),
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

impl From<&Geometry> for HilbertGeometry {
    fn from(geom: &Geometry) -> Self {
        encode_geometry(&geom)
    }
}

impl Into<Geometry> for HilbertGeometry {
    fn into(self) -> Geometry {
        decode_geometry(&self)
    }
}

impl HilbertGeometry {
    pub fn encode_bincode(self, config: &Configuration) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, *config)
    }

    pub fn decode_bincode(
        data: &[u8],
        config: &Configuration,
    ) -> Result<HilbertGeometry, DecodeError> {
        let (decoded, _) = bincode::decode_from_slice(data, *config)?;
        Ok(decoded)
    }
}

// Geometry <-> HWKB
pub struct HilbertSerializer {
    config: Configuration,
}

impl HilbertSerializer {
    pub fn new() -> Self {
        Self {
            config: config::standard(),
        }
    }

    pub fn encode(&self, geom: &Geometry) -> Result<Vec<u8>, EncodeError> {
        let hg = HilbertGeometry::from(geom);
        hg.encode_bincode(&self.config)
    }

    pub fn decode(&self, data: &[u8]) -> Result<Geometry, DecodeError> {
        let hg = HilbertGeometry::decode_bincode(data, &self.config)?;
        Ok(hg.into())
    }
}
