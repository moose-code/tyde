use crossterm::{cursor, queue, style::{self, Color, SetBackgroundColor, SetForegroundColor}};
use std::io::{self, Write};

use crate::chart;
use crate::color;
use crate::scene;
use crate::tide::{self, TideDirection};

use chrono::{DateTime, Local};

/// Render one complete frame to the terminal.
pub fn render_frame(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    now: DateTime<Local>,
    time_secs: f64,
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

    let tide_height = tide::predict(now);
    let tide_dir = tide::direction(now);
    let extreme = tide::next_extreme(now);

    // Layout: 1 line info bar, then ocean scene, then chart (4 rows + 1 x-axis)
    let chart_rows: u16 = 5;
    let scene_height = height.saturating_sub(1 + chart_rows);
    let scene_height = scene_height.max(3);

    // --- Info bar (row 0) ---
    render_info_bar(stdout, width, tide_height, tide_dir, &extreme, now)?;

    // --- Ocean scene (rows 1..1+scene_height) ---
    let grid = scene::render_scene(width, scene_height, tide_height, time_secs);
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
    render_tide_chart(stdout, width, chart_rows, chart_top, now)?;

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
    queue!(
        stdout,
        style::SetForegroundColor(color::INFO_DIM),
        style::Print("  Cape Town, SA"),
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
    let time_str = extreme.time.format("%H:%M").to_string();
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
    let written = 5 + 15 + 5 + 12 + 5 + dir.as_str().len() + 5 + label.len() + time_str.len() + countdown.len() + 5;
    let remaining = (width as usize).saturating_sub(written);
    if remaining > 0 {
        queue!(stdout, style::Print(" ".repeat(remaining)))?;
    }

    Ok(())
}

fn render_tide_chart(
    stdout: &mut io::Stdout,
    width: u16,
    chart_rows: u16,
    top: u16,
    now: DateTime<Local>,
) -> io::Result<()> {
    let label_width: usize = 6; // "1.5m⡇" = 5 chars + separator
    let chart_width = (width as usize).saturating_sub(label_width + 1);
    let chart_height = (chart_rows as usize).saturating_sub(1); // leave 1 row for x-axis

    if chart_width < 8 || chart_height < 1 {
        return Ok(());
    }

    let (lines, now_col) = chart::render_chart(now, chart_width, chart_height);
    let y_labels = chart::y_axis_labels(chart_height, now);

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

        // Chart content — highlight the "now" column
        for (j, ch) in line.chars().enumerate() {
            if j == now_col {
                queue!(
                    stdout,
                    style::SetForegroundColor(color::CHART_MARKER),
                    style::Print(ch),
                )?;
            } else {
                queue!(
                    stdout,
                    style::SetForegroundColor(color::CHART_CURVE),
                    style::Print(ch),
                )?;
            }
        }

        // Fill remaining
        let fill = (width as usize).saturating_sub(label_width + line.len());
        if fill > 0 {
            queue!(stdout, style::Print(" ".repeat(fill)))?;
        }
    }

    // X-axis labels
    let xaxis_row = top + chart_height as u16;
    queue!(
        stdout,
        cursor::MoveTo(0, xaxis_row),
        style::SetBackgroundColor(bg),
        style::SetForegroundColor(color::CHART_AXIS),
        style::Print(format!("{:>5}", "")),
    )?;

    // Build x-axis string with hour labels and ▲ now marker
    let mut x_chars: Vec<(char, bool)> = vec![(' ', false); chart_width]; // (char, is_now_marker)
    let spacing = chart_width / 4;
    for i in 0..=4 {
        let hour = i * 6;
        let pos = i * spacing;
        let label = format!("{:02}", hour);
        for (k, ch) in label.chars().enumerate() {
            if pos + k < chart_width {
                x_chars[pos + k] = (ch, false);
            }
        }
    }

    // Place ▲ at the current time position
    let now_hour_f = now.format("%H").to_string().parse::<f64>().unwrap_or(0.0)
        + now.format("%M").to_string().parse::<f64>().unwrap_or(0.0) / 60.0;
    let now_marker_pos = (now_hour_f / 24.0 * chart_width as f64) as usize;
    if now_marker_pos < chart_width {
        x_chars[now_marker_pos] = ('▲', true);
    }

    // Render x-axis
    for &(ch, is_marker) in &x_chars {
        if is_marker {
            queue!(stdout, style::SetForegroundColor(color::CHART_MARKER), style::Print(ch))?;
        } else {
            queue!(stdout, style::SetForegroundColor(color::CHART_AXIS), style::Print(ch))?;
        }
    }

    // Fill remaining
    let remaining_x = (width as usize).saturating_sub(label_width + chart_width);
    if remaining_x > 0 {
        queue!(stdout, style::Print(" ".repeat(remaining_x)))?;
    }

    Ok(())
}
