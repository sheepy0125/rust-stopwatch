/// Setup
extern crate stopwatch;
use eframe::{egui::CentralPanel, epi::App, run_native, NativeOptions};
use stopwatch::Stopwatch;

/// Stopwatch GUI setup
struct StopwatchGui {
    stopwatch: Stopwatch,
    stopwatch_text: String,
    laps: Vec<f64>,
}

impl Default for StopwatchGui {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            stopwatch_text: "0".to_owned(),
            laps: vec![],
        }
    }
}

/// Stopwatch handler
trait HandleStopwatch {
    fn _get_time_elapsed(&mut self) -> f64;
    fn update_stopwatch(&mut self);
    fn lap_time(&mut self);
    fn clear_laps(&mut self);
}
impl HandleStopwatch for StopwatchGui {
    fn _get_time_elapsed(&mut self) -> f64 {
        (self.stopwatch.elapsed().as_millis() as f64) / (1_000 as f64)
    }

    fn update_stopwatch(&mut self) {
        // Update stopwatch text
        self.stopwatch_text = format!("{}", self._get_time_elapsed());
    }

    fn lap_time(&mut self) {
        let time_elapsed = self._get_time_elapsed();
        self.laps.push(time_elapsed);
    }

    fn clear_laps(&mut self) {
        self.laps.clear();
    }
}

/// Stopwatch GUI
impl App for StopwatchGui {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        // Update the stopwatch if needed
        if self.stopwatch.is_running() {
            self.update_stopwatch();
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Sheepy's Amazing Stopwatch made in Rust!");

            // Buttons
            if ui.button("Start").clicked() {
                self.stopwatch.start();
            }
            if ui.button("Stop").clicked() {
                self.stopwatch.stop();
            }
            if ui.button("Clear").clicked() {
                self.stopwatch.reset();
                self.update_stopwatch();
                self.clear_laps();
            }
            if ui.button("Lap").clicked() {
                self.lap_time();
            }

            // Display
            ui.label(&self.stopwatch_text);

            // Laps
            ui.label("Laps");
            for lap in &self.laps {
                ui.label(format!("{}", lap));
            }
        });

        // Constant refresh mode
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Stopwatch"
    }
}

/// Main
fn main() {
    let options = NativeOptions::default();
    run_native(Box::new(StopwatchGui::default()), options);
}
