/// Daylight properties for scene rendering.
pub struct DaylightInfo {
    pub brightness: f64, // 0.18 (night) to 1.0 (full day)
    pub warmth: f64,     // 0.0 (neutral) to 1.0 (golden hour)
}

/// Solar declination in degrees for a given day of year (1–365).
fn solar_declination(day_of_year: u32) -> f64 {
    -23.44 * ((360.0 / 365.0) * (day_of_year as f64 + 10.0)).to_radians().cos()
}

/// Equation of Time in hours — difference between apparent and mean solar time.
fn equation_of_time(day_of_year: u32) -> f64 {
    let b = 2.0 * std::f64::consts::PI * (day_of_year as f64 - 81.0) / 365.0;
    // Result in minutes, convert to hours
    (9.87 * (2.0 * b).sin() - 7.53 * b.cos() - 1.5 * b.sin()) / 60.0
}

/// Correction from clock time to solar time (in hours).
/// Solar noon occurs at `12.0 + correction` clock time.
/// `lon` is station longitude, `utc_offset_minutes` is the timezone offset.
fn solar_noon_offset(lon: f64, utc_offset_minutes: i32) -> f64 {
    let tz_center_lon = (utc_offset_minutes as f64 / 60.0) * 15.0;
    (tz_center_lon - lon) / 15.0
}

/// Solar altitude angle in degrees.
/// `lat`/`lon` in degrees, `utc_offset_minutes` is the timezone, `hour` is clock time (0.0–24.0).
fn solar_altitude(lat: f64, lon: f64, utc_offset_minutes: i32, day_of_year: u32, hour: f64) -> f64 {
    let dec = solar_declination(day_of_year);
    let eot = equation_of_time(day_of_year);
    let lon_corr = solar_noon_offset(lon, utc_offset_minutes);

    // Convert clock hour to solar hour angle
    let solar_hour = hour - lon_corr + eot;
    let hour_angle = (solar_hour - 12.0) * 15.0; // degrees

    let lat_r = lat.to_radians();
    let dec_r = dec.to_radians();
    let ha_r = hour_angle.to_radians();

    let sin_alt = lat_r.sin() * dec_r.sin() + lat_r.cos() * dec_r.cos() * ha_r.cos();
    sin_alt.clamp(-1.0, 1.0).asin().to_degrees()
}

/// Compute approximate sunrise and sunset in clock hours (station-local, 0.0–24.0).
/// Accounts for longitude within timezone and equation of time.
/// Returns None for polar day/night (sun never rises or never sets).
pub fn sunrise_sunset(lat: f64, lon: f64, utc_offset_minutes: i32, day_of_year: u32) -> Option<(f64, f64)> {
    let dec = solar_declination(day_of_year);
    let lat_r = lat.to_radians();
    let dec_r = dec.to_radians();

    let cos_omega = -(lat_r.tan() * dec_r.tan());
    if cos_omega < -1.0 || cos_omega > 1.0 {
        return None; // polar day or polar night
    }

    let omega = cos_omega.acos().to_degrees();
    let eot = equation_of_time(day_of_year);
    let lon_corr = solar_noon_offset(lon, utc_offset_minutes);

    // Solar times corrected to clock time
    let sunrise = 12.0 - omega / 15.0 + lon_corr - eot;
    let sunset = 12.0 + omega / 15.0 + lon_corr - eot;
    Some((sunrise, sunset))
}

/// Compute daylight info from station parameters and local clock hour.
pub fn compute_daylight(lat: f64, lon: f64, utc_offset_minutes: i32, day_of_year: u32, hour: f64) -> DaylightInfo {
    let alt = solar_altitude(lat, lon, utc_offset_minutes, day_of_year, hour);

    let brightness = if alt > 10.0 {
        1.0
    } else if alt > 0.0 {
        // Low sun — still fairly bright
        0.6 + 0.4 * (alt / 10.0)
    } else if alt > -6.0 {
        // Civil twilight
        0.2 + 0.4 * ((alt + 6.0) / 6.0)
    } else if alt > -12.0 {
        // Nautical twilight
        0.2 * ((alt + 12.0) / 6.0)
    } else {
        0.0
    };
    // ensure a minimum so the scene is always slightly visible
    let brightness = brightness.max(0.18);

    let warmth = if alt > 0.0 && alt < 15.0 {
        // Golden hour: warmth peaks when sun is ~5° above horizon
        let peak = 5.0;
        let spread = 10.0;
        (1.0 - ((alt - peak) / spread).abs()).max(0.0)
    } else if alt > -4.0 && alt <= 0.0 {
        // Fading warmth during twilight
        ((alt + 4.0) / 4.0).max(0.0)
    } else {
        0.0
    };

    DaylightInfo { brightness, warmth }
}
