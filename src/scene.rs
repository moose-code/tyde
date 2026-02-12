use crossterm::style::Color;

use crate::color;
use crate::sun::DaylightInfo;

/// A single cell in the scene buffer.
#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: ' ',
            fg: Color::Reset,
            bg: color::SAND_DRY,
        }
    }
}

/// Deterministic hash for animation — returns 0.0..1.0.
fn cell_hash(row: u16, col: u16, tick: u32) -> f64 {
    // Simple hash mixing row, col, and a time tick
    let mut v: u32 = row as u32 * 7919 + col as u32 * 104729 + tick * 31337;
    v ^= v << 13;
    v ^= v >> 17;
    v ^= v << 5;
    (v % 10000) as f64 / 10000.0
}

/// Build the ocean scene for one frame.
///
/// - `width`, `height`: terminal cell dimensions for the ocean area
/// - `tide_height`: current tide in metres (typically 0.0–2.0)
/// - `time_secs`: monotonic seconds (for animation)
/// - `daylight`: brightness and warmth for day/night cycle
pub fn render_scene(width: u16, height: u16, tide_height: f64, time_secs: f64, daylight: &DaylightInfo) -> Vec<Vec<Cell>> {
    let w = width as usize;
    let h = height as usize;
    let mut grid = vec![vec![Cell::default(); w]; h];

    if w < 10 || h < 3 {
        return grid;
    }

    // Map tide height to wave edge position (fraction of screen width).
    // tide 0.0m → 30% of width, tide 2.0m → 75% of width
    let tide_frac = ((tide_height / 2.0).clamp(0.0, 1.0) * 0.45 + 0.30) as f64;
    let base_edge = (w as f64 * tide_frac) as i32;

    // Animation tick for foam randomness (~4 changes/sec)
    let foam_tick = (time_secs * 4.0) as u32;

    for row in 0..h {
        let row_f = row as f64;
        let row_u16 = row as u16;

        // Per-row wave offset: 3 layered sine waves
        let wave1 = (time_secs * 1.8 + row_f * 0.4).sin() * 2.5;
        let wave2 = (time_secs * 0.7 + row_f * 0.25 + 1.5).sin() * 1.5;
        let wave3 = (time_secs * 2.5 + row_f * 0.6 + 3.0).sin() * 0.8;
        let wave_offset = wave1 + wave2 + wave3;

        let edge = base_edge + wave_offset as i32;

        for col in 0..w {
            let col_i = col as i32;
            let col_u16 = col as u16;
            let dist_from_edge = col_i - edge; // negative = ocean, positive = sand

            let cell = if dist_from_edge < -8 {
                // Deep ocean
                let h = cell_hash(row_u16, col_u16, foam_tick / 3);
                let ch = if h < 0.06 { '~' }
                    else if h < 0.09 { '≈' }
                    else if h < 0.11 { '~' }
                    else { '~' };
                let fg = if dist_from_edge < -20 {
                    color::OCEAN_MID
                } else {
                    color::OCEAN_SHALLOW
                };
                Cell { ch, fg, bg: color::OCEAN_DEEP }

            } else if dist_from_edge < -3 {
                // Shallow water approaching crest
                let h = cell_hash(row_u16, col_u16, foam_tick / 2);
                let ch = if h < 0.15 { '~' }
                    else if h < 0.25 { '~' }
                    else { '~' };
                Cell {
                    ch,
                    fg: color::WAVE_CREST,
                    bg: color::OCEAN_MID,
                }

            } else if dist_from_edge < 0 {
                // Wave crest zone
                let h = cell_hash(row_u16, col_u16, foam_tick);
                let ch = if h < 0.3 { '}' }
                    else if h < 0.5 { ')' }
                    else if h < 0.7 { ']' }
                    else { ')' };
                Cell {
                    ch,
                    fg: color::WAVE_BRIGHT,
                    bg: color::OCEAN_SHALLOW,
                }

            } else if dist_from_edge < 2 {
                // Immediate foam
                let h = cell_hash(row_u16, col_u16, foam_tick);
                let ch = if h < 0.25 { '*' }
                    else if h < 0.5 { ':' }
                    else if h < 0.75 { '\'' }
                    else { '·' };
                Cell {
                    ch,
                    fg: color::FOAM_WHITE,
                    bg: color::OCEAN_SHALLOW,
                }

            } else if dist_from_edge < 6 {
                // Foam / spray zone
                let h = cell_hash(row_u16, col_u16, foam_tick);
                let ch = if h < 0.15 { '.' }
                    else if h < 0.25 { ':' }
                    else if h < 0.30 { '\'' }
                    else { ' ' };
                Cell {
                    ch,
                    fg: color::FOAM_LIGHT,
                    bg: color::SAND_WET,
                }

            } else if dist_from_edge < 12 {
                // Wet sand
                let h = cell_hash(row_u16, col_u16, foam_tick / 4);
                let ch = if h < 0.05 { '.' } else { ' ' };
                Cell {
                    ch,
                    fg: color::FOAM_LIGHT,
                    bg: color::SAND_WET,
                }

            } else if dist_from_edge < 20 {
                // Damp sand
                Cell {
                    ch: ' ',
                    fg: color::SAND_DAMP,
                    bg: color::SAND_DAMP,
                }

            } else if dist_from_edge < 30 {
                // Mid sand
                Cell {
                    ch: ' ',
                    fg: color::SAND_MID,
                    bg: color::SAND_MID,
                }

            } else {
                // Dry sand
                Cell {
                    ch: ' ',
                    fg: color::SAND_DRY,
                    bg: color::SAND_DRY,
                }
            };

            // Apply day/night lighting
            grid[row][col] = Cell {
                ch: cell.ch,
                fg: color::apply_daylight(cell.fg, daylight),
                bg: color::apply_daylight(cell.bg, daylight),
            };
        }
    }

    grid
}
