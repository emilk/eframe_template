/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EguiApp {
    // Example stuff:
    label: String,
    value: f32,
    painting: Painting,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            painting: Default::default(),
        }
    }
}

impl egui::app::App for EguiApp {
    fn name(&self) -> &str {
        "Egui Template"
    }

    /// Called by the framework to load old app state (if any).
    fn load(&mut self, storage: &dyn egui::app::Storage) {
        *self = egui::app::get_value(storage, egui::app::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(&mut self, ctx: &egui::CtxRef, integration_context: &mut egui::app::IntegrationContext) {
        let EguiApp {
            label,
            value,
            painting,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::f32(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("Powered by Egui"),
                );
            });
        });

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked {
                        integration_context.output.quit = true;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Template");
            ui.hyperlink("https://github.com/emilk/egui_template");
            ui.add(egui::github_link_file_line!(
                "https://github.com/emilk/egui_template/blob/master/",
                "Direct link to source code."
            ));
            egui::demos::warn_if_debug_build(ui);

            ui.separator();

            ui.heading("Central Panel");
            ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
            ui.label("It is often a great place for big things, like drawings:");

            ui.heading("Draw with your mouse to paint:");
            painting.ui_control(ui);
            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                painting.ui_content(ui);
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        // Resize the glium window to be just the size we need it to be:
        integration_context.output.window_size = Some(ctx.used_size());
    }
}

// ----------------------------------------------------------------------------

/// Example code for painting on a canvas with your mouse
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::color::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            self.stroke.ui(ui, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked {
                self.lines.clear();
            }
        })
        .1
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());
        let rect = response.rect;

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if response.active {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                let canvas_pos = mouse_pos - rect.min;
                if current_line.last() != Some(&canvas_pos) {
                    current_line.push(canvas_pos);
                }
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::PaintCmd::line(points, self.stroke));
            }
        }

        response
    }
}
