use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeoResult {
    pub lat: f64,
    pub lon: f64,
    pub city: Option<String>,
    pub country: Option<String>,
}

/// Detect the user's location via ip-api.com (HTTP, no key required).
/// Returns None on any failure (timeout, parse error, offline, etc.).
pub fn detect_location() -> Option<GeoResult> {
    let resp = ureq::get("http://ip-api.com/json/")
        .timeout(std::time::Duration::from_secs(3))
        .call()
        .ok()?;
    let body = resp.into_string().ok()?;
    let geo: GeoResult = serde_json::from_str(&body).ok()?;
    Some(geo)
}
