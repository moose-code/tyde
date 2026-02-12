use chrono::{DateTime, Local, TimeDelta};

use crate::tide;

/// Braille dot patterns for plotting.
/// Each braille character is a 2×4 grid of dots.
/// We use a simplified approach: one data point per column,
/// mapping to row within a braille cell.

const BRAILLE_BASE: u32 = 0x2800;

// Braille dot positions (col, row) → bit index:
// (0,0)=0, (0,1)=1, (0,2)=2, (1,0)=3, (1,1)=4, (1,2)=5, (0,3)=6, (1,3)=7
const BRAILLE_DOTS: [u32; 8] = [
    0x01, // (0,0)
    0x02, // (0,1)
    0x04, // (0,2)
    0x08, // (1,0)
    0x10, // (1,1)
    0x20, // (1,2)
    0x40, // (0,3)
    0x80, // (1,3)
];

/// Generate a 24-hour tide chart as a vector of lines (strings of braille characters).
///
/// Returns `(lines, now_col)` where `now_col` is the column index of the current time marker.
///
/// `chart_width` is the number of braille characters wide (each spans 2 data columns).
/// `chart_height` is the number of braille characters tall (each spans 4 data rows).
pub fn render_chart(
    now: DateTime<Local>,
    chart_width: usize,
    chart_height: usize,
) -> (Vec<String>, usize) {
    if chart_width < 4 || chart_height < 1 {
        return (vec![], 0);
    }

    let data_cols = chart_width * 2;
    let data_rows = chart_height * 4;

    // Start of day (midnight) — chart spans 00:00 to 24:00
    let today = now.date_naive();
    let start = today
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(now.timezone())
        .unwrap();

    // Sample tide heights across 24 hours
    let mut heights: Vec<f64> = Vec::with_capacity(data_cols);
    for i in 0..data_cols {
        let t_hours = 24.0 * (i as f64) / (data_cols as f64);
        let dt = start + TimeDelta::seconds((t_hours * 3600.0) as i64);
        heights.push(tide::predict(dt));
    }

    // Find min/max for scaling
    let min_h = heights.iter().cloned().fold(f64::MAX, f64::min);
    let max_h = heights.iter().cloned().fold(f64::MIN, f64::max);
    let range = (max_h - min_h).max(0.1);

    // Map heights to row indices (0 = top, data_rows-1 = bottom)
    let row_indices: Vec<usize> = heights
        .iter()
        .map(|h| {
            let frac = (h - min_h) / range;
            let row = ((1.0 - frac) * (data_rows - 1) as f64).round() as usize;
            row.min(data_rows - 1)
        })
        .collect();

    // Build braille grid
    let mut braille_grid = vec![vec![0u32; chart_width]; chart_height];

    for (col_idx, &row_idx) in row_indices.iter().enumerate() {
        let bcol = col_idx / 2; // braille character column
        let brow = row_idx / 4; // braille character row
        let sub_col = col_idx % 2; // 0 or 1 within braille char
        let sub_row = row_idx % 4; // 0..3 within braille char

        if bcol < chart_width && brow < chart_height {
            let dot_idx = match (sub_col, sub_row) {
                (0, r) if r < 3 => r,
                (1, r) if r < 3 => r + 3,
                (0, 3) => 6,
                (1, 3) => 7,
                _ => continue,
            };
            braille_grid[brow][bcol] |= BRAILLE_DOTS[dot_idx];
        }
    }

    // Convert to strings
    let lines: Vec<String> = braille_grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&bits| char::from_u32(BRAILLE_BASE + bits).unwrap_or(' '))
                .collect()
        })
        .collect();

    // Now marker column (braille char index)
    let elapsed_hours = (now - start).num_seconds() as f64 / 3600.0;
    let now_data_col = ((elapsed_hours / 24.0) * data_cols as f64) as usize;
    let now_col = now_data_col / 2;

    (lines, now_col.min(chart_width.saturating_sub(1)))
}

/// Format y-axis labels for the chart.
pub fn y_axis_labels(chart_height: usize, now: DateTime<Local>) -> Vec<String> {
    // Recalculate min/max to produce matching labels
    let today = now.date_naive();
    let start = today
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(now.timezone())
        .unwrap();

    let mut min_h = f64::MAX;
    let mut max_h = f64::MIN;
    for i in 0..100 {
        let t_hours = 24.0 * (i as f64) / 100.0;
        let dt = start + TimeDelta::seconds((t_hours * 3600.0) as i64);
        let h = tide::predict(dt);
        min_h = min_h.min(h);
        max_h = max_h.max(h);
    }

    let mut labels = Vec::with_capacity(chart_height);
    for i in 0..chart_height {
        let frac = 1.0 - (i as f64) / (chart_height.max(1) as f64 - 1.0).max(1.0);
        let h = min_h + frac * (max_h - min_h);
        labels.push(format!("{:.1}m", h));
    }
    labels
}
