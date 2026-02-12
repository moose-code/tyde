mod chart;
mod color;
mod geolocation;
mod renderer;
mod scene;
mod station;
mod sun;
mod terminal;
mod tide;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Local;

use station::{TideStation, STATIONS, DEFAULT_STATION_INDEX};

fn resolve_station() -> &'static TideStation {
    let args: Vec<String> = std::env::args().collect();

    // --list-stations: print and exit
    if args.iter().any(|a| a == "--list-stations") {
        station::list_stations();
        std::process::exit(0);
    }

    // --station <name>: pick by partial name
    if let Some(pos) = args.iter().position(|a| a == "--station") {
        if let Some(query) = args.get(pos + 1) {
            match station::find_by_name(query) {
                Some(idx) => {
                    eprintln!("Using station: {}", STATIONS[idx].name);
                    return &STATIONS[idx];
                }
                None => {
                    eprintln!("No station matching \"{}\". Use --list-stations to see all.", query);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("--station requires a name argument");
            std::process::exit(1);
        }
    }

    // --offline: skip geolocation, use default
    if args.iter().any(|a| a == "--offline") {
        eprintln!("Offline mode — using {}", STATIONS[DEFAULT_STATION_INDEX].name);
        return &STATIONS[DEFAULT_STATION_INDEX];
    }

    // Auto-detect location via IP geolocation
    eprint!("Detecting location... ");
    match geolocation::detect_location() {
        Some(geo) => {
            let idx = station::nearest_station(geo.lat, geo.lon);
            let city = geo.city.as_deref().unwrap_or("unknown");
            let country = geo.country.as_deref().unwrap_or("");
            eprintln!(
                "using {} (nearest to {}, {})",
                STATIONS[idx].name, city, country
            );
            std::thread::sleep(Duration::from_millis(500));
            &STATIONS[idx]
        }
        None => {
            eprintln!("failed, using {}", STATIONS[DEFAULT_STATION_INDEX].name);
            &STATIONS[DEFAULT_STATION_INDEX]
        }
    }
}

fn main() {
    let station = resolve_station();

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
        if renderer::render_frame(&mut stdout, term.width, term.height, now, time_secs, station)
            .is_err()
        {
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
