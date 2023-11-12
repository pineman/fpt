use egui::Color32;
use sha2::Digest;

const GB_FRAME_IN_SECONDS: f64 = 0.016666666667;

pub struct TemplateApp {
    value: u64,
    frame_count: u64,
    last_time: f64,
    accum_time: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            value: 0,
            frame_count: 0,
            last_time: 0.0,
            accum_time: 0.0,
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

#[cfg(target_arch = "wasm32")]
fn now() -> f64 {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
        .expect("failed to get performance from global object")
        .unchecked_into::<web_sys::Performance>()
        .now()
}

fn calc_sha256(input: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
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
            let unstable_dt = ui.input(|i| i.unstable_dt);
            // let delta_time = time - self.last_time;
            let delta_time = unstable_dt as f64;
            self.accum_time += delta_time;
            while self.accum_time >= GB_FRAME_IN_SECONDS {
                self.frame_count += 1;
                // ... RENDER GAME BOY SCREEN ...
                for _ in 0..1000 {
                    calc_sha256("hello world");
                }
                self.accum_time -= GB_FRAME_IN_SECONDS;
            }
            // self.last_time = now() / 1000.0;
            self.last_time = time;

            egui::Grid::new("my_grid").striped(true).show(ui, |ui| {
                macro_rules! stat {
                    ($label:literal : $value:expr) => {
                        ui.colored_label(Color32::LIGHT_GRAY, $label);
                        ui.monospace(stringify!($value));
                        ui.monospace($value);
                        ui.end_row();
                    };
                }
                stat!("time"        : format!("{:.8}", time));
                stat!("dt"          : format!("{:.8}", delta_time));
                stat!("unstable_dt" : format!("{:.8}", unstable_dt));
                stat!("accum. time" : format!("{:.8}", self.accum_time));
                stat!("last time"   : format!("{:.8}", self.last_time));
                stat!("Ideal count" : format!("{}"   , time / GB_FRAME_IN_SECONDS));
                stat!("Frame count" : format!("{}"   , self.frame_count));
                stat!("UI updates"  : format!("{}"   , self.value));
            });

            ui.separator();

            ui.heading("fpt");
            self.value += 1;
            ui.add(egui::Label::new(self.value.to_string()));
        });

        ctx.request_repaint();
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

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
