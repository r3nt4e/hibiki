use std::path::PathBuf;

use board::Board;
use eframe::egui::{self, RichText, Widget};
use knob::Knob;
use rodio::OutputStream;
use sound::SoundKind;

mod board;
mod knob;
mod scene;
mod sound;
mod trigger;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Hibiki")
            .with_inner_size([1280., 720.]),
        ..Default::default()
    };
    eframe::run_native("Hibiki", options, Box::new(|cc| Box::new(Hibiki::new(cc))))
}

struct Hibiki {
    board: Board,
    _stream: OutputStream,
}

impl Hibiki {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MACCHIATO);
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "DelaGothicOne".to_owned(),
            egui::FontData::from_static(include_bytes!("DelaGothicOne-Regular.ttf")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Name("DelaGothicOne".into()))
            .or_default()
            .push("DelaGothicOne".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let board = Board::new(PathBuf::from("scene.hibiki.ron"), stream_handle);

        Self { board, _stream }
    }
}

impl eframe::App for Hibiki {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(
                    RichText::new("響")
                        .family(egui::FontFamily::Name("DelaGothicOne".into()))
                        .size(200.)
                        .color(catppuccin_egui::MACCHIATO.surface0),
                );
            });
            self.board.ui(ui);
            egui::Window::new("Controller").show(ctx, |ui| {
                if let Some(controller) = self.board.selected_controller_mut() {
                    ui.horizontal(|ui| {
                        ui.label("Kind: ");
                        egui::ComboBox::from_id_source("SoundKind")
                            .selected_text(format!("{}", controller.kind))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::Trigger,
                                    format!("{}", SoundKind::Trigger),
                                );
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::CutItself,
                                    format!("{}", SoundKind::CutItself),
                                );
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::Hold,
                                    format!("{}", SoundKind::Hold),
                                );
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::HoldRepeat,
                                    format!("{}", SoundKind::HoldRepeat),
                                );
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::Toggle,
                                    format!("{}", SoundKind::Toggle),
                                );
                                ui.selectable_value(
                                    &mut controller.kind,
                                    SoundKind::ToggleRepeat,
                                    format!("{}", SoundKind::ToggleRepeat),
                                );
                            });
                    });
                    ui.add_space(2.);
                    ui.horizontal(|ui| {
                        let volume = Knob {
                            hint_color: catppuccin_egui::MACCHIATO.yellow,
                            val: &mut controller.volume,
                        };
                        volume.ui(ui);
                        let pan = Knob {
                            hint_color: catppuccin_egui::MACCHIATO.blue,
                            val: &mut controller.pan,
                        };
                        pan.ui(ui);
                    });
                } else {
                    ui.label(RichText::new("Right-click on a sound to inspect").italics());
                }
            });
        });
    }
}
