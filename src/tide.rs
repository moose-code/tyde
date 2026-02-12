use chrono::{DateTime, Local, TimeDelta};

/// A single tidal harmonic constituent.
struct Constituent {
    amplitude: f64, // metres
    speed: f64,     // degrees per hour
    phase: f64,     // degrees (epoch phase for Cape Town)
}

/// Cape Town tidal constituents (semi-diurnal dominant).
const CONSTITUENTS: &[Constituent] = &[
    Constituent { amplitude: 0.47, speed: 28.984, phase: 172.0 }, // M2
    Constituent { amplitude: 0.18, speed: 30.000, phase: 196.0 }, // S2
    Constituent { amplitude: 0.10, speed: 28.440, phase: 148.0 }, // N2
    Constituent { amplitude: 0.06, speed: 15.041, phase: 210.0 }, // K1
    Constituent { amplitude: 0.05, speed: 13.943, phase: 195.0 }, // O1
];

const Z0: f64 = 0.85; // mean sea level (metres above chart datum)

/// Reference epoch: 2000-01-01 00:00 UTC (J2000).
fn hours_since_epoch(dt: DateTime<Local>) -> f64 {
    let epoch = chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(chrono::Utc)
        .unwrap();
    let utc = dt.with_timezone(&chrono::Utc);
    let dur = utc.signed_duration_since(epoch);
    dur.num_seconds() as f64 / 3600.0
}

/// Predict tide height (metres) at a given time.
pub fn predict(dt: DateTime<Local>) -> f64 {
    let t = hours_since_epoch(dt);
    let mut h = Z0;
    for c in CONSTITUENTS {
        let angle = (c.speed * t - c.phase).to_radians();
        h += c.amplitude * angle.cos();
    }
    h
}

/// Rate of change of tide height (m/hr).
pub fn rate(dt: DateTime<Local>) -> f64 {
    let t = hours_since_epoch(dt);
    let mut dhdt = 0.0;
    for c in CONSTITUENTS {
        let angle = (c.speed * t - c.phase).to_radians();
        // derivative: -A * speed_rad * sin(angle)
        let speed_rad = c.speed.to_radians(); // convert deg/hr to rad/hr
        dhdt += -c.amplitude * speed_rad * angle.sin();
    }
    dhdt
}

#[derive(Debug, Clone, Copy)]
pub enum TideDirection {
    Rising,
    Falling,
    Slack,
}

impl TideDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            TideDirection::Rising => "Rising",
            TideDirection::Falling => "Falling",
            TideDirection::Slack => "Slack",
        }
    }
}

pub fn direction(dt: DateTime<Local>) -> TideDirection {
    let r = rate(dt);
    if r > 0.005 {
        TideDirection::Rising
    } else if r < -0.005 {
        TideDirection::Falling
    } else {
        TideDirection::Slack
    }
}

#[derive(Debug, Clone)]
pub struct TideExtreme {
    pub time: DateTime<Local>,
    pub height: f64,
    pub is_high: bool,
}

/// Scan forward from `from` in 10-minute steps to find the next tidal extreme.
pub fn next_extreme(from: DateTime<Local>) -> TideExtreme {
    let step = TimeDelta::minutes(10);
    let mut t = from;
    let mut prev_rate = rate(t);

    // Walk up to 15 hours (more than one full tidal cycle)
    for _ in 0..90 {
        t = t + step;
        let r = rate(t);
        // Sign change = extreme between prev step and this step
        if (prev_rate > 0.0 && r <= 0.0) || (prev_rate < 0.0 && r >= 0.0) {
            // Bisect to refine
            let refined = bisect_extreme(t - step, t);
            let h = predict(refined);
            let is_high = prev_rate > 0.0;
            return TideExtreme {
                time: refined,
                height: h,
                is_high,
            };
        }
        prev_rate = r;
    }

    // Fallback (shouldn't happen with real tidal constituents)
    let h = predict(t);
    TideExtreme {
        time: t,
        height: h,
        is_high: true,
    }
}

fn bisect_extreme(mut lo: DateTime<Local>, mut hi: DateTime<Local>) -> DateTime<Local> {
    for _ in 0..20 {
        let mid = lo + (hi - lo) / 2;
        let r = rate(mid);
        if (rate(lo) > 0.0 && r > 0.0) || (rate(lo) < 0.0 && r < 0.0) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo + (hi - lo) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    #[test]
    fn tide_height_in_range() {
        // Sample 48 hours of predictions — all should be 0.0–2.0m
        let now = Local::now();
        for i in 0..288 {
            let dt = now + TimeDelta::minutes(i * 10);
            let h = predict(dt);
            assert!(h > -0.1, "tide too low: {} at +{}min", h, i * 10);
            assert!(h < 2.5, "tide too high: {} at +{}min", h, i * 10);
        }
    }

    #[test]
    fn finds_next_extreme() {
        let now = Local::now();
        let ext = next_extreme(now);
        // Should be in the future
        assert!(ext.time > now);
        // Should be within 15 hours
        let diff = (ext.time - now).num_hours();
        assert!(diff < 15, "next extreme too far: {}h", diff);
        // Height should be reasonable
        assert!(ext.height > 0.0 && ext.height < 2.5);
    }

    #[test]
    fn direction_changes() {
        // Over 24h, direction should change at least once
        let now = Local::now();
        let mut saw_rising = false;
        let mut saw_falling = false;
        for i in 0..144 {
            let dt = now + TimeDelta::minutes(i * 10);
            match direction(dt) {
                TideDirection::Rising => saw_rising = true,
                TideDirection::Falling => saw_falling = true,
                TideDirection::Slack => {}
            }
        }
        assert!(saw_rising, "never saw rising tide in 24h");
        assert!(saw_falling, "never saw falling tide in 24h");
    }
}
