use eframe::egui;
use eframe::egui::CentralPanel;

mod audio_handle;
mod command_receiver;
use command_receiver::commander::{AudioCommand, Commander};

const APP_NAME: &str = "sound-board-gui";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    commander: Commander,
}

impl Default for MyApp {
    fn default() -> Self {
        let commander = Commander::new();

        Self { commander }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            corner_radius: 10.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            ..Default::default()
        };

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(32.0, egui::FontFamily::Proportional),
                );
            });

            egui::Grid::new("grid").show(ui, |ui| {
                ui.horizontal(|ui| {
                    let sound_name = "Add entry";
                    if ui.button(sound_name).clicked() {
                        self.commander
                            .send(AudioCommand::AddAudio(String::from("audio")));
                    }
                    let increment_name = "Increment cutoff";
                    if ui.button(increment_name).clicked() {
                        self.commander
                            .send(AudioCommand::ChangeCutoff(String::from("cutoff")));
                    }
                });
            });
        });
    }
}
