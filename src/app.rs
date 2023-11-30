/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TutorialApp {
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    language: String,
    code: String,
}

impl Default for TutorialApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 10.0,
            language: "rs".into(),
            code: "
fn main() {\n\
\tprintln!(\"Hello rust!\");\n\
}\n\
".into(),
        }
    }
}

impl TutorialApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TutorialApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    //     let transparency : u8 = {
    //         if self.value >= 0.0 { 90 + (self.value.round() as u8) * 10 }
    //         else { 90 - (self.value.round() as u8) * 10 }
    //     };
    //     egui::Color32::from_rgba_premultiplied(12, 12, 12, transparency).to_normalized_gamma_f32()
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Tutorial");

            ui.horizontal(|ui| {
                ui.label("Write here! ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            ui.horizontal(|ui| {
                if ui.button("-").clicked() && self.value > 0.0 {
                    self.value -= 1.0;
                }
                ui.label(self.value.to_string());
                if ui.button("+").clicked() && self.value < 10.0 {
                    self.value += 1.0;
                }
            });

            ui.separator();
            
            let mut theme = egui_extras::syntax_highlighting::CodeTheme::dark();
            
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, &self.language);
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
                );
            });

            ui.separator();
            
            ui.add(egui::github_link_file!(
                "https://github.com/ChanJoon/eframe_tutorial/blob/master/",
                "Source code."
            ));
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                copyright_footer(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn copyright_footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Copyright © 2023 ");
        ui.hyperlink_to("ChanJoon", "https://github.com/ChanJoon");
        ui.label(".");
    });
}
