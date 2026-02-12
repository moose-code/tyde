/// Tidal harmonic constituent data for a station.
pub struct Constituent {
    pub amplitude: f64, // metres
    pub phase: f64,     // degrees (epoch phase)
}

/// A coastal tide station with its harmonic constituents.
pub struct TideStation {
    pub name: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub z0: f64, // mean sea level above chart datum (metres)
    pub utc_offset_minutes: i32, // standard-time offset from UTC in minutes
    /// Constituents in order: M2, S2, N2, K1, O1
    pub constituents: [Constituent; 5],
}

impl TideStation {
    /// Return a chrono FixedOffset for this station's standard timezone.
    pub fn timezone(&self) -> chrono::FixedOffset {
        chrono::FixedOffset::east_opt(self.utc_offset_minutes * 60).unwrap()
    }
}

/// Universal astronomical speeds (degrees per hour) for the 5 major constituents.
/// Order: M2, S2, N2, K1, O1
pub const SPEEDS: [f64; 5] = [
    28.984, // M2
    30.000, // S2
    28.440, // N2
    15.041, // K1
    13.943, // O1
];

/// Bundled database of ~25 coastal tide stations worldwide.
pub const STATIONS: &[TideStation] = &[
    // Africa
    TideStation {
        name: "Cape Town, SA",
        lat: -33.9,
        lon: 18.4,
        z0: 0.85,
        utc_offset_minutes: 120, // UTC+2
        constituents: [
            Constituent { amplitude: 0.47, phase: 172.0 },
            Constituent { amplitude: 0.18, phase: 196.0 },
            Constituent { amplitude: 0.10, phase: 148.0 },
            Constituent { amplitude: 0.06, phase: 210.0 },
            Constituent { amplitude: 0.05, phase: 195.0 },
        ],
    },
    // North America — East Coast
    TideStation {
        name: "New York, US",
        lat: 40.7,
        lon: -74.0,
        z0: 0.72,
        utc_offset_minutes: -300, // UTC-5
        constituents: [
            Constituent { amplitude: 0.66, phase: 6.0 },
            Constituent { amplitude: 0.15, phase: 30.0 },
            Constituent { amplitude: 0.15, phase: 348.0 },
            Constituent { amplitude: 0.10, phase: 107.0 },
            Constituent { amplitude: 0.06, phase: 99.0 },
        ],
    },
    TideStation {
        name: "Miami, US",
        lat: 25.8,
        lon: -80.1,
        z0: 0.40,
        utc_offset_minutes: -300, // UTC-5
        constituents: [
            Constituent { amplitude: 0.19, phase: 350.0 },
            Constituent { amplitude: 0.07, phase: 14.0 },
            Constituent { amplitude: 0.05, phase: 330.0 },
            Constituent { amplitude: 0.09, phase: 93.0 },
            Constituent { amplitude: 0.06, phase: 79.0 },
        ],
    },
    TideStation {
        name: "Boston, US",
        lat: 42.4,
        lon: -71.0,
        z0: 1.51,
        utc_offset_minutes: -300, // UTC-5
        constituents: [
            Constituent { amplitude: 1.36, phase: 352.0 },
            Constituent { amplitude: 0.21, phase: 14.0 },
            Constituent { amplitude: 0.30, phase: 330.0 },
            Constituent { amplitude: 0.14, phase: 100.0 },
            Constituent { amplitude: 0.10, phase: 93.0 },
        ],
    },
    // North America — West Coast
    TideStation {
        name: "San Francisco, US",
        lat: 37.8,
        lon: -122.5,
        z0: 0.91,
        utc_offset_minutes: -480, // UTC-8
        constituents: [
            Constituent { amplitude: 0.57, phase: 186.0 },
            Constituent { amplitude: 0.13, phase: 195.0 },
            Constituent { amplitude: 0.12, phase: 162.0 },
            Constituent { amplitude: 0.37, phase: 220.0 },
            Constituent { amplitude: 0.23, phase: 206.0 },
        ],
    },
    TideStation {
        name: "Seattle, US",
        lat: 47.6,
        lon: -122.3,
        z0: 1.83,
        utc_offset_minutes: -480, // UTC-8
        constituents: [
            Constituent { amplitude: 1.08, phase: 182.0 },
            Constituent { amplitude: 0.30, phase: 196.0 },
            Constituent { amplitude: 0.23, phase: 160.0 },
            Constituent { amplitude: 0.84, phase: 276.0 },
            Constituent { amplitude: 0.50, phase: 256.0 },
        ],
    },
    TideStation {
        name: "Los Angeles, US",
        lat: 33.7,
        lon: -118.3,
        z0: 0.85,
        utc_offset_minutes: -480, // UTC-8
        constituents: [
            Constituent { amplitude: 0.50, phase: 178.0 },
            Constituent { amplitude: 0.15, phase: 190.0 },
            Constituent { amplitude: 0.11, phase: 156.0 },
            Constituent { amplitude: 0.33, phase: 214.0 },
            Constituent { amplitude: 0.22, phase: 200.0 },
        ],
    },
    // Europe
    TideStation {
        name: "London, UK",
        lat: 51.5,
        lon: 0.0,
        z0: 3.50,
        utc_offset_minutes: 0, // UTC+0
        constituents: [
            Constituent { amplitude: 2.19, phase: 333.0 },
            Constituent { amplitude: 0.64, phase: 10.0 },
            Constituent { amplitude: 0.44, phase: 308.0 },
            Constituent { amplitude: 0.12, phase: 57.0 },
            Constituent { amplitude: 0.10, phase: 335.0 },
        ],
    },
    TideStation {
        name: "Lisbon, PT",
        lat: 38.7,
        lon: -9.1,
        z0: 1.90,
        utc_offset_minutes: 0, // UTC+0
        constituents: [
            Constituent { amplitude: 1.00, phase: 56.0 },
            Constituent { amplitude: 0.36, phase: 80.0 },
            Constituent { amplitude: 0.22, phase: 36.0 },
            Constituent { amplitude: 0.07, phase: 55.0 },
            Constituent { amplitude: 0.06, phase: 335.0 },
        ],
    },
    TideStation {
        name: "Amsterdam, NL",
        lat: 52.4,
        lon: 4.9,
        z0: 0.83,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.64, phase: 5.0 },
            Constituent { amplitude: 0.17, phase: 50.0 },
            Constituent { amplitude: 0.11, phase: 340.0 },
            Constituent { amplitude: 0.09, phase: 110.0 },
            Constituent { amplitude: 0.07, phase: 16.0 },
        ],
    },
    TideStation {
        name: "Reykjavik, IS",
        lat: 64.2,
        lon: -21.9,
        z0: 2.00,
        utc_offset_minutes: 0, // UTC+0
        constituents: [
            Constituent { amplitude: 1.33, phase: 162.0 },
            Constituent { amplitude: 0.42, phase: 195.0 },
            Constituent { amplitude: 0.27, phase: 140.0 },
            Constituent { amplitude: 0.10, phase: 170.0 },
            Constituent { amplitude: 0.06, phase: 145.0 },
        ],
    },
    // Asia
    TideStation {
        name: "Tokyo, JP",
        lat: 35.7,
        lon: 139.7,
        z0: 0.84,
        utc_offset_minutes: 540, // UTC+9
        constituents: [
            Constituent { amplitude: 0.47, phase: 290.0 },
            Constituent { amplitude: 0.20, phase: 320.0 },
            Constituent { amplitude: 0.10, phase: 268.0 },
            Constituent { amplitude: 0.24, phase: 192.0 },
            Constituent { amplitude: 0.17, phase: 170.0 },
        ],
    },
    TideStation {
        name: "Shanghai, CN",
        lat: 31.2,
        lon: 121.5,
        z0: 1.75,
        utc_offset_minutes: 480, // UTC+8
        constituents: [
            Constituent { amplitude: 1.27, phase: 320.0 },
            Constituent { amplitude: 0.57, phase: 350.0 },
            Constituent { amplitude: 0.25, phase: 300.0 },
            Constituent { amplitude: 0.29, phase: 210.0 },
            Constituent { amplitude: 0.19, phase: 190.0 },
        ],
    },
    TideStation {
        name: "Mumbai, IN",
        lat: 19.0,
        lon: 72.8,
        z0: 2.20,
        utc_offset_minutes: 330, // UTC+5:30
        constituents: [
            Constituent { amplitude: 1.47, phase: 340.0 },
            Constituent { amplitude: 0.58, phase: 10.0 },
            Constituent { amplitude: 0.30, phase: 318.0 },
            Constituent { amplitude: 0.44, phase: 50.0 },
            Constituent { amplitude: 0.24, phase: 35.0 },
        ],
    },
    TideStation {
        name: "Singapore, SG",
        lat: 1.3,
        lon: 103.8,
        z0: 1.50,
        utc_offset_minutes: 480, // UTC+8
        constituents: [
            Constituent { amplitude: 0.75, phase: 330.0 },
            Constituent { amplitude: 0.32, phase: 0.0 },
            Constituent { amplitude: 0.15, phase: 310.0 },
            Constituent { amplitude: 0.30, phase: 175.0 },
            Constituent { amplitude: 0.22, phase: 155.0 },
        ],
    },
    TideStation {
        name: "Hong Kong, CN",
        lat: 22.3,
        lon: 114.2,
        z0: 0.80,
        utc_offset_minutes: 480, // UTC+8
        constituents: [
            Constituent { amplitude: 0.36, phase: 310.0 },
            Constituent { amplitude: 0.14, phase: 340.0 },
            Constituent { amplitude: 0.08, phase: 288.0 },
            Constituent { amplitude: 0.31, phase: 190.0 },
            Constituent { amplitude: 0.22, phase: 170.0 },
        ],
    },
    // Oceania
    TideStation {
        name: "Sydney, AU",
        lat: -33.9,
        lon: 151.2,
        z0: 0.90,
        utc_offset_minutes: 600, // UTC+10
        constituents: [
            Constituent { amplitude: 0.50, phase: 220.0 },
            Constituent { amplitude: 0.12, phase: 250.0 },
            Constituent { amplitude: 0.11, phase: 198.0 },
            Constituent { amplitude: 0.16, phase: 310.0 },
            Constituent { amplitude: 0.10, phase: 290.0 },
        ],
    },
    TideStation {
        name: "Auckland, NZ",
        lat: -36.8,
        lon: 174.8,
        z0: 1.50,
        utc_offset_minutes: 720, // UTC+12
        constituents: [
            Constituent { amplitude: 1.07, phase: 248.0 },
            Constituent { amplitude: 0.20, phase: 276.0 },
            Constituent { amplitude: 0.22, phase: 225.0 },
            Constituent { amplitude: 0.12, phase: 22.0 },
            Constituent { amplitude: 0.07, phase: 358.0 },
        ],
    },
    TideStation {
        name: "Honolulu, US",
        lat: 21.3,
        lon: -157.9,
        z0: 0.30,
        utc_offset_minutes: -600, // UTC-10
        constituents: [
            Constituent { amplitude: 0.16, phase: 50.0 },
            Constituent { amplitude: 0.06, phase: 60.0 },
            Constituent { amplitude: 0.04, phase: 28.0 },
            Constituent { amplitude: 0.17, phase: 100.0 },
            Constituent { amplitude: 0.09, phase: 80.0 },
        ],
    },
    // South America
    TideStation {
        name: "Buenos Aires, AR",
        lat: -34.6,
        lon: -58.4,
        z0: 0.50,
        utc_offset_minutes: -180, // UTC-3
        constituents: [
            Constituent { amplitude: 0.31, phase: 84.0 },
            Constituent { amplitude: 0.14, phase: 108.0 },
            Constituent { amplitude: 0.06, phase: 62.0 },
            Constituent { amplitude: 0.08, phase: 100.0 },
            Constituent { amplitude: 0.05, phase: 82.0 },
        ],
    },
    TideStation {
        name: "Rio de Janeiro, BR",
        lat: -22.9,
        lon: -43.2,
        z0: 0.60,
        utc_offset_minutes: -180, // UTC-3
        constituents: [
            Constituent { amplitude: 0.32, phase: 65.0 },
            Constituent { amplitude: 0.13, phase: 85.0 },
            Constituent { amplitude: 0.07, phase: 43.0 },
            Constituent { amplitude: 0.06, phase: 75.0 },
            Constituent { amplitude: 0.05, phase: 55.0 },
        ],
    },
    TideStation {
        name: "Valparaiso, CL",
        lat: -33.0,
        lon: -71.6,
        z0: 0.80,
        utc_offset_minutes: -240, // UTC-4
        constituents: [
            Constituent { amplitude: 0.44, phase: 160.0 },
            Constituent { amplitude: 0.15, phase: 178.0 },
            Constituent { amplitude: 0.09, phase: 140.0 },
            Constituent { amplitude: 0.16, phase: 210.0 },
            Constituent { amplitude: 0.11, phase: 195.0 },
        ],
    },
    // Middle East
    TideStation {
        name: "Dubai, AE",
        lat: 25.3,
        lon: 55.3,
        z0: 0.80,
        utc_offset_minutes: 240, // UTC+4
        constituents: [
            Constituent { amplitude: 0.42, phase: 295.0 },
            Constituent { amplitude: 0.18, phase: 320.0 },
            Constituent { amplitude: 0.09, phase: 273.0 },
            Constituent { amplitude: 0.24, phase: 40.0 },
            Constituent { amplitude: 0.16, phase: 20.0 },
        ],
    },
    // Canada
    TideStation {
        name: "Vancouver, CA",
        lat: 49.3,
        lon: -123.1,
        z0: 2.10,
        utc_offset_minutes: -480, // UTC-8
        constituents: [
            Constituent { amplitude: 0.94, phase: 184.0 },
            Constituent { amplitude: 0.27, phase: 200.0 },
            Constituent { amplitude: 0.20, phase: 162.0 },
            Constituent { amplitude: 0.80, phase: 278.0 },
            Constituent { amplitude: 0.47, phase: 258.0 },
        ],
    },
    TideStation {
        name: "Halifax, CA",
        lat: 44.6,
        lon: -63.6,
        z0: 0.80,
        utc_offset_minutes: -240, // UTC-4
        constituents: [
            Constituent { amplitude: 0.63, phase: 10.0 },
            Constituent { amplitude: 0.14, phase: 30.0 },
            Constituent { amplitude: 0.14, phase: 350.0 },
            Constituent { amplitude: 0.08, phase: 113.0 },
            Constituent { amplitude: 0.05, phase: 97.0 },
        ],
    },
];

/// Default station index (Cape Town).
pub const DEFAULT_STATION_INDEX: usize = 0;

/// Haversine distance in kilometres between two lat/lon points.
fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth radius in km
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

/// Find the nearest station to the given coordinates.
/// Returns the index into STATIONS.
pub fn nearest_station(lat: f64, lon: f64) -> usize {
    let mut best_idx = 0;
    let mut best_dist = f64::MAX;
    for (i, s) in STATIONS.iter().enumerate() {
        let d = haversine_km(lat, lon, s.lat, s.lon);
        if d < best_dist {
            best_dist = d;
            best_idx = i;
        }
    }
    best_idx
}

/// Find a station by partial name match (case-insensitive).
/// Returns the index into STATIONS, or None.
pub fn find_by_name(query: &str) -> Option<usize> {
    let q = query.to_lowercase();
    STATIONS
        .iter()
        .position(|s| s.name.to_lowercase().contains(&q))
}

/// Print all stations to stdout (for --list-stations).
pub fn list_stations() {
    println!("{:<25} {:>8} {:>8}", "Station", "Lat", "Lon");
    println!("{}", "-".repeat(43));
    for s in STATIONS {
        println!("{:<25} {:>8.1} {:>8.1}", s.name, s.lat, s.lon);
    }
}
