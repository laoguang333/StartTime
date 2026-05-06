#![windows_subsystem = "windows"]

use gpui::*;
use gpui_component::{button::*, *};
use std::time::{SystemTime, UNIX_EPOCH};

struct GpuiDemo {
    startup_ms: u128,
}

impl Render for GpuiDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("GPUI Component Demo")
            .child(
                Button::new("ok")
                    .primary()
                    .label(format!("Startup Time: {} ms", self.startup_ms)),
            )
    }
}

fn get_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut start_time_ms: u128 = 0;
    for i in 1..args.len().saturating_sub(1) {
        if args[i] == "--start-time" {
            start_time_ms = args[i + 1].parse().unwrap_or(0);
            break;
        }
    }

    if start_time_ms == 0 {
        start_time_ms = get_now_ms();
    }

    Application::new().run(move |cx| {
        gpui_component::init(cx);
        let elapsed_ms = get_now_ms() - start_time_ms;

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::from(format!("GPUI Demo - Startup: {} ms", elapsed_ms))),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|_| GpuiDemo { startup_ms: elapsed_ms });
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Failed to open window");
        })
        .detach();
    });
}
