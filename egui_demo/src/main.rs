#![windows_subsystem = "windows"]

use std::time::{SystemTime, UNIX_EPOCH};

struct EguiDemo {
    start_time_ms: u128,
    title_set: bool,
}

impl EguiDemo {
    fn new(start_time_ms: u128) -> Self {
        Self {
            start_time_ms,
            title_set: false,
        }
    }
}

impl eframe::App for EguiDemo {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.title_set {
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let ms = now_ms - self.start_time_ms;
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Title(format!(
                "egui Demo - Startup: {} ms",
                ms
            )));
            self.title_set = true;
        }

        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let ms = now_ms - self.start_time_ms;
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui Demo");
            ui.separator();
            ui.label(
                eframe::egui::RichText::new(format!("Startup Time: {ms} ms"))
                    .size(24.0)
                    .strong(),
            );
        });
    }
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let mut start_time_ms: u128 = 0;
    for i in 1..args.len().saturating_sub(1) {
        if args[i] == "--start-time" {
            start_time_ms = args[i + 1].parse().unwrap_or(0);
            break;
        }
    }
    if start_time_ms == 0 {
        start_time_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }

    eframe::run_native(
        "egui Demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(EguiDemo::new(start_time_ms)))),
    )
}
