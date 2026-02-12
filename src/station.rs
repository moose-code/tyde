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

/// Bundled database of ~50 coastal tide stations worldwide.
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
    TideStation {
        name: "Lagos, NG",
        lat: 6.4,
        lon: 3.4,
        z0: 0.60,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.38, phase: 30.0 },
            Constituent { amplitude: 0.14, phase: 55.0 },
            Constituent { amplitude: 0.08, phase: 10.0 },
            Constituent { amplitude: 0.05, phase: 110.0 },
            Constituent { amplitude: 0.04, phase: 90.0 },
        ],
    },
    TideStation {
        name: "Mombasa, KE",
        lat: -4.0,
        lon: 39.7,
        z0: 1.60,
        utc_offset_minutes: 180, // UTC+3
        constituents: [
            Constituent { amplitude: 1.06, phase: 338.0 },
            Constituent { amplitude: 0.50, phase: 5.0 },
            Constituent { amplitude: 0.22, phase: 315.0 },
            Constituent { amplitude: 0.22, phase: 45.0 },
            Constituent { amplitude: 0.14, phase: 25.0 },
        ],
    },
    TideStation {
        name: "Dar es Salaam, TZ",
        lat: -6.8,
        lon: 39.3,
        z0: 1.80,
        utc_offset_minutes: 180, // UTC+3
        constituents: [
            Constituent { amplitude: 1.15, phase: 340.0 },
            Constituent { amplitude: 0.52, phase: 8.0 },
            Constituent { amplitude: 0.24, phase: 318.0 },
            Constituent { amplitude: 0.20, phase: 48.0 },
            Constituent { amplitude: 0.13, phase: 28.0 },
        ],
    },
    TideStation {
        name: "Accra, GH",
        lat: 5.5,
        lon: -0.2,
        z0: 0.55,
        utc_offset_minutes: 0, // UTC+0
        constituents: [
            Constituent { amplitude: 0.34, phase: 35.0 },
            Constituent { amplitude: 0.12, phase: 58.0 },
            Constituent { amplitude: 0.07, phase: 12.0 },
            Constituent { amplitude: 0.04, phase: 105.0 },
            Constituent { amplitude: 0.03, phase: 85.0 },
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
    // Europe — Mediterranean
    TideStation {
        name: "Barcelona, ES",
        lat: 41.4,
        lon: 2.2,
        z0: 0.15,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.06, phase: 240.0 },
            Constituent { amplitude: 0.03, phase: 270.0 },
            Constituent { amplitude: 0.01, phase: 218.0 },
            Constituent { amplitude: 0.04, phase: 80.0 },
            Constituent { amplitude: 0.03, phase: 60.0 },
        ],
    },
    TideStation {
        name: "Marseille, FR",
        lat: 43.3,
        lon: 5.4,
        z0: 0.18,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.07, phase: 250.0 },
            Constituent { amplitude: 0.03, phase: 280.0 },
            Constituent { amplitude: 0.02, phase: 228.0 },
            Constituent { amplitude: 0.04, phase: 85.0 },
            Constituent { amplitude: 0.03, phase: 65.0 },
        ],
    },
    TideStation {
        name: "Naples, IT",
        lat: 40.8,
        lon: 14.3,
        z0: 0.20,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.09, phase: 260.0 },
            Constituent { amplitude: 0.04, phase: 290.0 },
            Constituent { amplitude: 0.02, phase: 238.0 },
            Constituent { amplitude: 0.03, phase: 90.0 },
            Constituent { amplitude: 0.02, phase: 70.0 },
        ],
    },
    TideStation {
        name: "Athens, GR",
        lat: 37.9,
        lon: 23.7,
        z0: 0.12,
        utc_offset_minutes: 120, // UTC+2
        constituents: [
            Constituent { amplitude: 0.05, phase: 280.0 },
            Constituent { amplitude: 0.02, phase: 310.0 },
            Constituent { amplitude: 0.01, phase: 258.0 },
            Constituent { amplitude: 0.02, phase: 100.0 },
            Constituent { amplitude: 0.02, phase: 80.0 },
        ],
    },
    TideStation {
        name: "Istanbul, TR",
        lat: 41.0,
        lon: 29.0,
        z0: 0.10,
        utc_offset_minutes: 180, // UTC+3
        constituents: [
            Constituent { amplitude: 0.03, phase: 290.0 },
            Constituent { amplitude: 0.01, phase: 320.0 },
            Constituent { amplitude: 0.01, phase: 268.0 },
            Constituent { amplitude: 0.02, phase: 110.0 },
            Constituent { amplitude: 0.01, phase: 90.0 },
        ],
    },
    // Europe — Scandinavia / Baltic
    TideStation {
        name: "Oslo, NO",
        lat: 59.9,
        lon: 10.7,
        z0: 0.14,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.10, phase: 35.0 },
            Constituent { amplitude: 0.03, phase: 70.0 },
            Constituent { amplitude: 0.02, phase: 12.0 },
            Constituent { amplitude: 0.03, phase: 130.0 },
            Constituent { amplitude: 0.02, phase: 110.0 },
        ],
    },
    TideStation {
        name: "Copenhagen, DK",
        lat: 55.7,
        lon: 12.6,
        z0: 0.10,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.07, phase: 45.0 },
            Constituent { amplitude: 0.02, phase: 78.0 },
            Constituent { amplitude: 0.01, phase: 22.0 },
            Constituent { amplitude: 0.02, phase: 140.0 },
            Constituent { amplitude: 0.02, phase: 120.0 },
        ],
    },
    TideStation {
        name: "Stockholm, SE",
        lat: 59.3,
        lon: 18.1,
        z0: 0.05,
        utc_offset_minutes: 60, // UTC+1
        constituents: [
            Constituent { amplitude: 0.02, phase: 50.0 },
            Constituent { amplitude: 0.01, phase: 82.0 },
            Constituent { amplitude: 0.01, phase: 28.0 },
            Constituent { amplitude: 0.01, phase: 145.0 },
            Constituent { amplitude: 0.01, phase: 125.0 },
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
    TideStation {
        name: "Bangkok, TH",
        lat: 13.5,
        lon: 100.6,
        z0: 1.10,
        utc_offset_minutes: 420, // UTC+7
        constituents: [
            Constituent { amplitude: 0.40, phase: 310.0 },
            Constituent { amplitude: 0.18, phase: 340.0 },
            Constituent { amplitude: 0.08, phase: 288.0 },
            Constituent { amplitude: 0.35, phase: 170.0 },
            Constituent { amplitude: 0.25, phase: 150.0 },
        ],
    },
    TideStation {
        name: "Manila, PH",
        lat: 14.6,
        lon: 120.9,
        z0: 0.60,
        utc_offset_minutes: 480, // UTC+8
        constituents: [
            Constituent { amplitude: 0.22, phase: 300.0 },
            Constituent { amplitude: 0.10, phase: 330.0 },
            Constituent { amplitude: 0.05, phase: 278.0 },
            Constituent { amplitude: 0.28, phase: 185.0 },
            Constituent { amplitude: 0.20, phase: 165.0 },
        ],
    },
    TideStation {
        name: "Jakarta, ID",
        lat: -6.1,
        lon: 106.8,
        z0: 0.45,
        utc_offset_minutes: 420, // UTC+7
        constituents: [
            Constituent { amplitude: 0.15, phase: 315.0 },
            Constituent { amplitude: 0.06, phase: 345.0 },
            Constituent { amplitude: 0.03, phase: 293.0 },
            Constituent { amplitude: 0.25, phase: 180.0 },
            Constituent { amplitude: 0.18, phase: 160.0 },
        ],
    },
    TideStation {
        name: "Ho Chi Minh City, VN",
        lat: 10.8,
        lon: 106.7,
        z0: 1.60,
        utc_offset_minutes: 420, // UTC+7
        constituents: [
            Constituent { amplitude: 0.30, phase: 305.0 },
            Constituent { amplitude: 0.12, phase: 335.0 },
            Constituent { amplitude: 0.06, phase: 283.0 },
            Constituent { amplitude: 0.55, phase: 175.0 },
            Constituent { amplitude: 0.35, phase: 155.0 },
        ],
    },
    // South Asia
    TideStation {
        name: "Colombo, LK",
        lat: 6.9,
        lon: 79.9,
        z0: 0.35,
        utc_offset_minutes: 330, // UTC+5:30
        constituents: [
            Constituent { amplitude: 0.19, phase: 350.0 },
            Constituent { amplitude: 0.08, phase: 18.0 },
            Constituent { amplitude: 0.04, phase: 328.0 },
            Constituent { amplitude: 0.10, phase: 60.0 },
            Constituent { amplitude: 0.06, phase: 42.0 },
        ],
    },
    TideStation {
        name: "Karachi, PK",
        lat: 24.9,
        lon: 67.0,
        z0: 1.30,
        utc_offset_minutes: 300, // UTC+5
        constituents: [
            Constituent { amplitude: 0.80, phase: 345.0 },
            Constituent { amplitude: 0.36, phase: 12.0 },
            Constituent { amplitude: 0.17, phase: 322.0 },
            Constituent { amplitude: 0.40, phase: 52.0 },
            Constituent { amplitude: 0.22, phase: 35.0 },
        ],
    },
    TideStation {
        name: "Chennai, IN",
        lat: 13.1,
        lon: 80.3,
        z0: 0.40,
        utc_offset_minutes: 330, // UTC+5:30
        constituents: [
            Constituent { amplitude: 0.28, phase: 342.0 },
            Constituent { amplitude: 0.12, phase: 8.0 },
            Constituent { amplitude: 0.06, phase: 320.0 },
            Constituent { amplitude: 0.06, phase: 55.0 },
            Constituent { amplitude: 0.04, phase: 38.0 },
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
    TideStation {
        name: "Perth, AU",
        lat: -31.9,
        lon: 115.9,
        z0: 0.45,
        utc_offset_minutes: 480, // UTC+8
        constituents: [
            Constituent { amplitude: 0.17, phase: 200.0 },
            Constituent { amplitude: 0.06, phase: 230.0 },
            Constituent { amplitude: 0.04, phase: 178.0 },
            Constituent { amplitude: 0.18, phase: 300.0 },
            Constituent { amplitude: 0.11, phase: 280.0 },
        ],
    },
    TideStation {
        name: "Darwin, AU",
        lat: -12.5,
        lon: 130.8,
        z0: 3.70,
        utc_offset_minutes: 570, // UTC+9:30
        constituents: [
            Constituent { amplitude: 1.82, phase: 245.0 },
            Constituent { amplitude: 0.85, phase: 275.0 },
            Constituent { amplitude: 0.37, phase: 222.0 },
            Constituent { amplitude: 0.58, phase: 330.0 },
            Constituent { amplitude: 0.32, phase: 310.0 },
        ],
    },
    TideStation {
        name: "Melbourne, AU",
        lat: -37.8,
        lon: 144.9,
        z0: 0.45,
        utc_offset_minutes: 600, // UTC+10
        constituents: [
            Constituent { amplitude: 0.22, phase: 230.0 },
            Constituent { amplitude: 0.07, phase: 260.0 },
            Constituent { amplitude: 0.05, phase: 208.0 },
            Constituent { amplitude: 0.10, phase: 315.0 },
            Constituent { amplitude: 0.06, phase: 295.0 },
        ],
    },
    TideStation {
        name: "Suva, FJ",
        lat: -18.1,
        lon: 178.4,
        z0: 0.80,
        utc_offset_minutes: 720, // UTC+12
        constituents: [
            Constituent { amplitude: 0.45, phase: 260.0 },
            Constituent { amplitude: 0.14, phase: 290.0 },
            Constituent { amplitude: 0.09, phase: 238.0 },
            Constituent { amplitude: 0.10, phase: 30.0 },
            Constituent { amplitude: 0.06, phase: 10.0 },
        ],
    },
    // Caribbean
    TideStation {
        name: "San Juan, PR",
        lat: 18.5,
        lon: -66.1,
        z0: 0.25,
        utc_offset_minutes: -240, // UTC-4
        constituents: [
            Constituent { amplitude: 0.12, phase: 355.0 },
            Constituent { amplitude: 0.04, phase: 15.0 },
            Constituent { amplitude: 0.03, phase: 333.0 },
            Constituent { amplitude: 0.08, phase: 95.0 },
            Constituent { amplitude: 0.05, phase: 78.0 },
        ],
    },
    TideStation {
        name: "Havana, CU",
        lat: 23.1,
        lon: -82.3,
        z0: 0.25,
        utc_offset_minutes: -300, // UTC-5
        constituents: [
            Constituent { amplitude: 0.11, phase: 345.0 },
            Constituent { amplitude: 0.04, phase: 8.0 },
            Constituent { amplitude: 0.03, phase: 325.0 },
            Constituent { amplitude: 0.07, phase: 90.0 },
            Constituent { amplitude: 0.05, phase: 72.0 },
        ],
    },
    // Central America
    TideStation {
        name: "Panama City, PA",
        lat: 9.0,
        lon: -79.5,
        z0: 2.50,
        utc_offset_minutes: -300, // UTC-5
        constituents: [
            Constituent { amplitude: 1.55, phase: 130.0 },
            Constituent { amplitude: 0.55, phase: 155.0 },
            Constituent { amplitude: 0.32, phase: 108.0 },
            Constituent { amplitude: 0.28, phase: 210.0 },
            Constituent { amplitude: 0.18, phase: 195.0 },
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
    // North America — Alaska
    TideStation {
        name: "Anchorage, US",
        lat: 61.2,
        lon: -149.9,
        z0: 4.60,
        utc_offset_minutes: -540, // UTC-9
        constituents: [
            Constituent { amplitude: 3.50, phase: 178.0 },
            Constituent { amplitude: 0.80, phase: 198.0 },
            Constituent { amplitude: 0.72, phase: 155.0 },
            Constituent { amplitude: 0.65, phase: 280.0 },
            Constituent { amplitude: 0.38, phase: 260.0 },
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
