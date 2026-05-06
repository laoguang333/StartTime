#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() {
    let mut start_time_ms: u128 = 0;
    let args: Vec<String> = std::env::args().collect();
    for i in 1..args.len().saturating_sub(1) {
        if args[i] == "--start-time" {
            start_time_ms = args[i + 1].parse().unwrap_or(0);
            break;
        }
    }

    if start_time_ms == 0 {
        start_time_ms = get_now_ms();
    }

    nwg::init().expect("Failed to init Native Windows GUI");

    let mut window: nwg::Window = Default::default();
    let mut label: nwg::Label = Default::default();
    let mut font: nwg::Font = Default::default();

    nwg::Font::builder()
        .family("Segoe UI")
        .size(16)
        .weight(700)
        .build(&mut font)
        .expect("Failed to build font");

    nwg::Window::builder()
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .size((800, 450))
        .position((300, 200))
        .title("NWG Demo")
        .build(&mut window)
        .expect("Failed to build window");

    nwg::Label::builder()
        .text("Starting...")
        .parent(&window)
        .position((250, 180))
        .size((300, 60))
        .font(Some(&font))
        .build(&mut label)
        .expect("Failed to build label");

    let elapsed_ms = get_now_ms() - start_time_ms;
    let title = format!("NWG Demo - Startup: {} ms", elapsed_ms);
    window.set_text(&title);
    label.set_text(&format!("Startup Time: {} ms", elapsed_ms));

    nwg::dispatch_thread_events();
}
