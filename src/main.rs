/// Setup
extern crate stopwatch;
use eframe::{
    egui::CentralPanel,
    egui::{vec2, Color32, Frame, ScrollArea},
    epi::App,
    run_native, NativeOptions,
};
use stopwatch::Stopwatch;

static EMPTY_TIME_FORMAT: &str = "00:00:00.000";

/// Stopwatch GUI setup
struct StopwatchGui {
    stopwatch: Stopwatch,
    stopwatch_text: String,
    laps: Vec<String>,
}
impl Default for StopwatchGui {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            stopwatch_text: EMPTY_TIME_FORMAT.to_owned(),
            laps: vec![],
        }
    }
}

/// Stopwatch handler
trait HandleStopwatch {
    fn _get_time_elapsed(&mut self) -> String;
    fn update_stopwatch(&mut self);
    fn lap_time(&mut self);
    fn clear_laps(&mut self);
}
impl HandleStopwatch for StopwatchGui {
    fn _get_time_elapsed(&mut self) -> String {
        let duration = self.stopwatch.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;
        let milliseconds = duration.as_millis() % 1000;
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )
    }

    fn update_stopwatch(&mut self) {
        self.stopwatch_text = self._get_time_elapsed();
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

            Frame::default()
                .fill(Color32::from_rgb(48, 48, 48))
                .show(ui, |ui| {
                    Frame::default().margin(vec2(11.5, 11.5)).show(ui, |ui| {
                        // Buttons
                        ui.horizontal(|ui| {
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
                        });
                        // Display
                        ui.label(format!("{} elapsed", &self.stopwatch_text));
                    });
                });

            ui.separator();

            // Laps
            Frame::default()
                .fill(Color32::from_rgb(48, 48, 48))
                .show(ui, |ui| {
                    ui.label("Laps (press Clear to clear)                 ");
                    ScrollArea::new([false, true]).show(ui, |ui| {
                        for lap in &self.laps {
                            ui.label(format!("{}", lap));
                        }
                    });
                });
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
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(200.0, 200.0)),
        always_on_top: true,
        ..Default::default()
    };
    run_native(Box::new(StopwatchGui::default()), options);
}
