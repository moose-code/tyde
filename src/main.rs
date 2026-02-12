mod chart;
mod color;
mod renderer;
mod scene;
mod terminal;
mod tide;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Local;

fn main() {
    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set Ctrl+C handler");

    // Initialize terminal
    let mut term = match terminal::Terminal::init() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to initialize terminal: {}", e);
            return;
        }
    };

    let start = Instant::now();
    let mut stdout = io::stdout();
    let frame_duration = Duration::from_millis(66); // ~15fps

    // Main animation loop
    while running.load(Ordering::SeqCst) {
        let frame_start = Instant::now();
        let now = Local::now();
        let time_secs = start.elapsed().as_secs_f64();

        // Render the frame
        if renderer::render_frame(&mut stdout, term.width, term.height, now, time_secs).is_err() {
            break;
        }

        // Calculate remaining time in this frame for event polling
        let elapsed = frame_start.elapsed();
        let poll_time = frame_duration.saturating_sub(elapsed);

        // Poll for events
        if let Some(action) = term.poll_event(poll_time) {
            match action {
                terminal::Action::Quit => break,
                terminal::Action::Resize => {
                    // Size already updated in poll_event; next frame will redraw
                }
            }
        }
    }

    // Clean up terminal
    terminal::Terminal::cleanup();
}
