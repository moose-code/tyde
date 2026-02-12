use crossterm::style::Color;

use crate::sun::DaylightInfo;

/// Apply daylight brightness and warmth to a color.
/// At night: dims toward dark blue. At golden hour: shifts toward warm orange.
pub fn apply_daylight(base: Color, daylight: &DaylightInfo) -> Color {
    if let Color::Rgb { r, g, b } = base {
        let br = daylight.brightness;
        let w = daylight.warmth;

        // Apply brightness
        let mut rf = r as f64 * br;
        let mut gf = g as f64 * br;
        let mut bf = b as f64 * br;

        // Warmth: shift toward orange/gold
        rf += w * 35.0;
        gf += w * 12.0;
        bf *= 1.0 - w * 0.4;

        // Night blue tint
        let night = (1.0 - br).max(0.0);
        bf += night * 18.0;

        Color::Rgb {
            r: rf.clamp(0.0, 255.0) as u8,
            g: gf.clamp(0.0, 255.0) as u8,
            b: bf.clamp(0.0, 255.0) as u8,
        }
    } else {
        base
    }
}

// Deep ocean
pub const OCEAN_DEEP: Color = Color::Rgb { r: 10, g: 20, b: 60 };
pub const OCEAN_MID: Color = Color::Rgb { r: 15, g: 40, b: 90 };
pub const OCEAN_SHALLOW: Color = Color::Rgb { r: 20, g: 60, b: 120 };

// Wave crest
pub const WAVE_CREST: Color = Color::Rgb { r: 40, g: 140, b: 180 };
pub const WAVE_BRIGHT: Color = Color::Rgb { r: 80, g: 200, b: 220 };

// Foam / spray
pub const FOAM_WHITE: Color = Color::Rgb { r: 220, g: 230, b: 240 };
pub const FOAM_LIGHT: Color = Color::Rgb { r: 180, g: 200, b: 210 };

// Sand gradient (wet → dry)
pub const SAND_WET: Color = Color::Rgb { r: 120, g: 100, b: 60 };
pub const SAND_DAMP: Color = Color::Rgb { r: 160, g: 140, b: 80 };
pub const SAND_MID: Color = Color::Rgb { r: 194, g: 170, b: 100 };
pub const SAND_DRY: Color = Color::Rgb { r: 220, g: 200, b: 130 };

// Info bar
pub const INFO_LABEL: Color = Color::Rgb { r: 80, g: 180, b: 220 };
pub const INFO_VALUE: Color = Color::Rgb { r: 220, g: 220, b: 220 };
pub const INFO_DIM: Color = Color::Rgb { r: 100, g: 100, b: 120 };

// Chart
pub const CHART_AXIS: Color = Color::Rgb { r: 80, g: 80, b: 100 };
pub const CHART_CURVE: Color = Color::Rgb { r: 60, g: 160, b: 200 };
pub const CHART_CURVE_NIGHT: Color = Color::Rgb { r: 35, g: 90, b: 120 };
pub const CHART_MARKER: Color = Color::Rgb { r: 255, g: 100, b: 80 };
pub const CHART_SUNRISE: Color = Color::Rgb { r: 255, g: 200, b: 80 };
pub const CHART_SUNSET: Color = Color::Rgb { r: 255, g: 130, b: 60 };
