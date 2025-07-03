use eframe::{egui, App};
use std::path::PathBuf;
use std::thread;

pub struct SnaccApp {
    status: String,
    watching: bool,
}

impl Default for SnaccApp {
    fn default() -> Self {
        Self {
            status: String::new(),
            watching: false,
        }
    }
}

impl App for SnaccApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📒 snacc x");

            if ui.button("📋 Copy Latest Notebook").clicked() {
                match copy_once() {
                    Ok(msg) => self.status = msg,
                    Err(e) => self.status = format!("❌ {}", e),
                }
            }

            if !self.watching {
                if ui.button("👀 Start Watching").clicked() {
                    self.status = "🔁 Watch mode running...".to_string();
                    self.watching = true;

                    thread::spawn(|| {
                        let dir = dirs::download_dir().unwrap_or(PathBuf::from("."));
                        let result = snacc_lib::watch_loop(dir, "code".to_string(), true);
                        if let Err(err) = result {
                            // Consider logging to a file or pushing to a channel later
                            eprintln!("❌ Watch loop error: {}", err);
                        }
                    });
                }
            } else {
                ui.label("👀 Watching Downloads folder...");
            }

            ui.separator();
            ui.label(&self.status);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "snacc x",
        options,
        Box::new(|_cc| Box::new(SnaccApp::default())),
    )
}

fn copy_once() -> Result<String, String> {
    let dir = dirs::download_dir().ok_or("Downloads folder not found")?;
    if let Some(nb) = snacc_lib::get_latest_ipynb(&dir) {
        snacc_lib::handle_notebook(nb, "code", true)
    } else {
        Err("No notebook found.".into())
    }
}
