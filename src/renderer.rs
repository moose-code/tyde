use crossterm::{cursor, queue, style::{self, Color, SetBackgroundColor, SetForegroundColor}};
use std::io::{self, Write};

use crate::chart;
use crate::color;
use crate::scene;
use crate::station::TideStation;
use crate::sun;
use crate::tide::{self, TideDirection};

use chrono::{DateTime, Datelike, Local};

/// Render one complete frame to the terminal.
pub fn render_frame(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    now: DateTime<Local>,
    time_secs: f64,
    station: &TideStation,
) -> io::Result<()> {
    if width < 40 || height < 10 {
        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            style::SetForegroundColor(color::INFO_VALUE),
            style::SetBackgroundColor(Color::Reset),
            style::Print("Terminal too small — resize to at least 40×10"),
        )?;
        stdout.flush()?;
        return Ok(());
    }

    let tide_height = tide::predict(station, now);
    let tide_dir = tide::direction(station, now);
    let extreme = tide::next_extreme(station, now);

    // Compute daylight for the scene
    let station_h = chart::station_hour(station, now);
    let tz = station.timezone();
    let station_now = now.with_timezone(&tz);
    let day_of_year = station_now.ordinal();
    let daylight = sun::compute_daylight(station.lat, station.lon, station.utc_offset_minutes, day_of_year, station_h);

    // Layout: 1 line info bar, then ocean scene, then chart
    let chart_rows: u16 = 7;
    let scene_height = height.saturating_sub(1 + chart_rows);
    let scene_height = scene_height.max(3);

    // --- Info bar (row 0) ---
    render_info_bar(stdout, width, tide_height, tide_dir, &extreme, now, station)?;

    // --- Ocean scene (rows 1..1+scene_height) ---
    let grid = scene::render_scene(width, scene_height, tide_height, time_secs, &daylight);
    for (r, row) in grid.iter().enumerate() {
        queue!(stdout, cursor::MoveTo(0, 1 + r as u16))?;
        let mut prev_fg = Color::Reset;
        let mut prev_bg = Color::Reset;
        for cell in row {
            if cell.fg != prev_fg {
                queue!(stdout, SetForegroundColor(cell.fg))?;
                prev_fg = cell.fg;
            }
            if cell.bg != prev_bg {
                queue!(stdout, SetBackgroundColor(cell.bg))?;
                prev_bg = cell.bg;
            }
            queue!(stdout, style::Print(cell.ch))?;
        }
    }

    // --- Tide chart (bottom rows) ---
    let chart_top = 1 + scene_height;
    render_tide_chart(stdout, width, chart_rows, chart_top, now, station)?;

    // Reset colors at end
    queue!(
        stdout,
        style::SetForegroundColor(Color::Reset),
        style::SetBackgroundColor(Color::Reset),
    )?;

    stdout.flush()?;
    Ok(())
}

fn render_info_bar(
    stdout: &mut io::Stdout,
    width: u16,
    height: f64,
    dir: TideDirection,
    extreme: &tide::TideExtreme,
    now: DateTime<Local>,
    station: &TideStation,
) -> io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        style::SetBackgroundColor(Color::Rgb { r: 15, g: 15, b: 30 }),
    )?;

    // TIDE label
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_LABEL),
        style::Print(" TIDE"),
    )?;

    // Location
    let location = format!("  {}", station.name);
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_DIM),
        style::Print(&location),
    )?;

    // Separator
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_DIM),
        style::Print("  │  "),
    )?;

    // Height
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_VALUE),
        style::Print(format!("Height: {:.2}m", height)),
    )?;

    // Separator + direction
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_DIM),
        style::Print("  │  "),
        style::SetForegroundColor(match dir {
            TideDirection::Rising => Color::Rgb { r: 80, g: 200, b: 120 },
            TideDirection::Falling => Color::Rgb { r: 200, g: 120, b: 80 },
            TideDirection::Slack => color::INFO_DIM,
        }),
        style::Print(dir.as_str()),
    )?;

    // Separator + next extreme
    let label = if extreme.is_high { "Next High" } else { "Next Low" };
    let time_str = extreme.time.with_timezone(&station.timezone()).format("%H:%M").to_string();
    let diff = extreme.time - now;
    let diff_h = diff.num_hours();
    let diff_m = diff.num_minutes() % 60;
    let countdown = format!("{}h {:02}m", diff_h, diff_m);

    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_DIM),
        style::Print("  │  "),
        style::SetForegroundColor(color::INFO_VALUE),
        style::Print(format!("{}: {:.2}m @ {} ({})", label, extreme.height, time_str, countdown)),
    )?;

    // Fill rest of line with background
    // Calculate how much we've written (approximate)
    let written = 5 + location.len() + 5 + 12 + 5 + dir.as_str().len() + 5 + label.len() + time_str.len() + countdown.len() + 5;
    let remaining = (width as usize).saturating_sub(written);
    if remaining > 0 {
        queue!(stdout, style::Print(" ".repeat(remaining)))?;
    }

    Ok(())
}

/// Color category for each x-axis character.
#[derive(Clone, Copy, PartialEq)]
enum XCharKind {
    Axis,
    NowMarker,
    Sunrise,
    Sunset,
}

fn render_tide_chart(
    stdout: &mut io::Stdout,
    width: u16,
    chart_rows: u16,
    top: u16,
    now: DateTime<Local>,
    station: &TideStation,
) -> io::Result<()> {
    let label_width: usize = 6; // "1.5m⡇" = 5 chars + separator
    let chart_width = (width as usize).saturating_sub(label_width + 1);
    let chart_height = (chart_rows as usize).saturating_sub(1); // leave 1 row for x-axis

    if chart_width < 8 || chart_height < 1 {
        return Ok(());
    }

    let (lines, now_col) = chart::render_chart(station, now, chart_width, chart_height);
    let y_labels = chart::y_axis_labels(station, chart_height, now);

    // Compute sunrise/sunset column positions
    let tz = station.timezone();
    let station_now = now.with_timezone(&tz);
    let day_of_year = station_now.ordinal();
    let sun_times = sun::sunrise_sunset(station.lat, station.lon, station.utc_offset_minutes, day_of_year);
    let sunrise_col = sun_times.map(|(sr, _)| ((sr / 24.0) * chart_width as f64) as usize);
    let sunset_col = sun_times.map(|(_, ss)| ((ss / 24.0) * chart_width as f64) as usize);

    let bg = Color::Rgb { r: 8, g: 8, b: 20 };

    for (i, line) in lines.iter().enumerate() {
        let row = top + i as u16;
        queue!(stdout, cursor::MoveTo(0, row))?;

        // Y-axis label
        let label = y_labels.get(i).map(|s| s.as_str()).unwrap_or("");
        queue!(
            stdout,
            style::SetBackgroundColor(bg),
            style::SetForegroundColor(color::CHART_AXIS),
            style::Print(format!("{:>5}", label)),
        )?;

        // Chart content — color by: now marker > sunrise/sunset > day/night
        for (j, ch) in line.chars().enumerate() {
            let fg = if j == now_col {
                color::CHART_MARKER
            } else if sunrise_col == Some(j) {
                color::CHART_SUNRISE
            } else if sunset_col == Some(j) {
                color::CHART_SUNSET
            } else if is_night_col(j, sunrise_col, sunset_col, chart_width) {
                color::CHART_CURVE_NIGHT
            } else {
                color::CHART_CURVE
            };
            queue!(stdout, style::SetForegroundColor(fg), style::Print(ch))?;
        }

        // Fill remaining
        let fill = (width as usize).saturating_sub(label_width + line.len());
        if fill > 0 {
            queue!(stdout, style::Print(" ".repeat(fill)))?;
        }
    }

    // --- X-axis ---
    let xaxis_row = top + chart_height as u16;
    queue!(
        stdout,
        cursor::MoveTo(0, xaxis_row),
        style::SetBackgroundColor(bg),
        style::SetForegroundColor(color::CHART_AXIS),
        style::Print(format!("{:>5}", "")),
    )?;

    // Build x-axis: hour labels, then sunrise/sunset labels, then ▲ now marker (highest priority)
    let mut x_axis: Vec<(char, XCharKind)> = vec![(' ', XCharKind::Axis); chart_width];

    // Layer 1: hour labels (00, 06, 12, 18, 24)
    let spacing = chart_width / 4;
    for i in 0..=4 {
        let hour = i * 6;
        let pos = i * spacing;
        let label = format!("{:02}", hour);
        for (k, ch) in label.chars().enumerate() {
            if pos + k < chart_width {
                x_axis[pos + k] = (ch, XCharKind::Axis);
            }
        }
    }

    // Layer 2: sunrise label "↑HH:MM"
    if let Some((sr, _)) = sun_times {
        let sr_h = sr as u32;
        let sr_m = ((sr - sr_h as f64) * 60.0) as u32;
        let label = format!("↑{:02}:{:02}", sr_h, sr_m);
        let pos = ((sr / 24.0) * chart_width as f64) as usize;
        // center the label on the position
        let start = pos.saturating_sub(label.chars().count() / 2);
        for (k, ch) in label.chars().enumerate() {
            if start + k < chart_width {
                x_axis[start + k] = (ch, XCharKind::Sunrise);
            }
        }
    }

    // Layer 3: sunset label "↓HH:MM"
    if let Some((_, ss)) = sun_times {
        let ss_h = ss as u32;
        let ss_m = ((ss - ss_h as f64) * 60.0) as u32;
        let label = format!("↓{:02}:{:02}", ss_h, ss_m);
        let pos = ((ss / 24.0) * chart_width as f64) as usize;
        let start = pos.saturating_sub(label.chars().count() / 2);
        for (k, ch) in label.chars().enumerate() {
            if start + k < chart_width {
                x_axis[start + k] = (ch, XCharKind::Sunset);
            }
        }
    }

    // Layer 4: ▲ now marker (always wins)
    let now_hour_f = chart::station_hour(station, now);
    let now_marker_pos = (now_hour_f / 24.0 * chart_width as f64) as usize;
    if now_marker_pos < chart_width {
        x_axis[now_marker_pos] = ('▲', XCharKind::NowMarker);
    }

    // Render x-axis
    for &(ch, kind) in &x_axis {
        let fg = match kind {
            XCharKind::Axis => color::CHART_AXIS,
            XCharKind::NowMarker => color::CHART_MARKER,
            XCharKind::Sunrise => color::CHART_SUNRISE,
            XCharKind::Sunset => color::CHART_SUNSET,
        };
        queue!(stdout, style::SetForegroundColor(fg), style::Print(ch))?;
    }

    // Fill remaining
    let remaining_x = (width as usize).saturating_sub(label_width + chart_width);
    if remaining_x > 0 {
        queue!(stdout, style::Print(" ".repeat(remaining_x)))?;
    }

    Ok(())
}

/// Returns true if this chart column falls during nighttime hours.
fn is_night_col(
    col: usize,
    sunrise_col: Option<usize>,
    sunset_col: Option<usize>,
    _chart_width: usize,
) -> bool {
    match (sunrise_col, sunset_col) {
        (Some(sr), Some(ss)) => col < sr || col > ss,
        _ => false, // no sunrise/sunset data — treat as all day
    }
}
