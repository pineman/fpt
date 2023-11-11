pub struct TemplateApp {
    value: u64,
    last_time: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            value: 0,
            last_time: 0.0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let time = ui.input(|i| i.time);
            ui.add(egui::Label::new(format!("{:.8}", time)));
            ui.add(egui::Label::new(format!("{:.8}", self.last_time)));
            ui.add(egui::Label::new(format!("{:.8}", time - self.last_time)));
            ui.add(egui::Label::new(ui.input(|i| i.unstable_dt).to_string()));
            self.last_time = time;
            // ui.add(egui::Label::new(ui.input(|i| i.stable_dt).to_string()));
            // ui.add(egui::Label::new(ui.input(|i| i.predicted_dt).to_string()));
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("fpt");
            self.value += 1;
            ui.add(egui::Label::new(self.value.to_string()));
        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
