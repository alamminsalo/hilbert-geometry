/// Normalizes longitude and latitude to the range [0.0, 1.0].
/// - Longitude wraps around [-180, 180]
/// - Latitude wraps around [-90, 90]
pub fn normalize_lon_lat(lon: f64, lat: f64) -> (f64, f64) {
    // Normalize longitude to [0.0, 1.0]
    let lon_wrapped = (lon + 180.0) % 360.0;
    let lon_norm = lon_wrapped / 360.0;

    // Normalize latitude to [0.0, 1.0]
    let lat_wrapped = (lat + 90.0) % 180.0;
    let lat_norm = lat_wrapped / 180.0;

    (lon_norm, lat_norm)
}

/// Denormalizes normalized longitude and latitude back to original ranges.
/// - Longitude in [-180.0, 180.0)
/// - Latitude in [-90.0, 90.0]
pub fn denormalize_lon_lat(lon_norm: f64, lat_norm: f64) -> (f64, f64) {
    // Denormalize longitude from [0.0, 1.0] to [-180.0, 180.0)
    let lon = lon_norm * 360.0 - 180.0;

    // Denormalize latitude from [0.0, 1.0] to [-90.0, 90.0]
    let lat = lat_norm * 180.0 - 90.;

    (lon, lat)
}
