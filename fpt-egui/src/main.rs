#![feature(lazy_cell)]

use std::sync::Arc;
use std::time::Duration;

use eframe::Frame;
use egui::{Color32, Context, TextureOptions, Ui};
use log::info;

const GB_FRAME_IN_SECONDS: f64 = 0.016666666667;

const PALETTE: [Color32; 4] = [
    Color32::from_rgb(0, 63, 0),
    Color32::from_rgb(46, 115, 32),
    Color32::from_rgb(140, 191, 10),
    Color32::from_rgb(160, 207, 10),
];

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn now() -> f64 {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
        .expect("failed to get performance from global object")
        .unchecked_into::<web_sys::Performance>()
        .now()
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
static APP_START: std::sync::LazyLock<std::time::Instant> =
    std::sync::LazyLock::new(std::time::Instant::now);

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn now() -> f64 {
    APP_START.elapsed().as_secs_f64() * 1000.0
}

pub struct FPT {
    egui_frame_count: u64,
    gb_frame_count: u64,
    accum_time: f64,
    image: Arc<egui::ColorImage>,
    texture: Option<egui::TextureHandle>,
    gb: fpt::Gameboy,
}

impl Default for FPT {
    fn default() -> Self {
        Self {
            egui_frame_count: 0,
            gb_frame_count: 0,
            accum_time: 0.0,
            image: Arc::new(egui::ColorImage::new([160, 144], Color32::TRANSPARENT)),
            texture: None,
            gb: fpt::Gameboy::new(),
        }
    }
}

impl FPT {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = FPT::default();
        if std::env::var("CI").is_err() {
            let rom = std::fs::read("roms/Tetris_World_Rev_1.gb").unwrap();
            app.gb.load_rom(&rom);
        }
        app
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                    }
                });
                ui.menu_button("LCD", |ui| {
                    if ui.button("clear").clicked() {
                        if let Some(image) = Arc::get_mut(&mut self.image) {
                            image.pixels.fill(Color32::TRANSPARENT);
                        }
                    }
                });
                ui.add_space(16.0);
            });
        });
    }

    fn emulator(&mut self, ui: &mut Ui) {
        let delta_time = ui.input(|i| i.unstable_dt) as f64;
        self.accum_time += delta_time;
        self.egui_frame_count += 1;
        if self.accum_time >= GB_FRAME_IN_SECONDS {
            self.gb_frame_count += 1;
            self.accum_time -= GB_FRAME_IN_SECONDS;
            let image = Arc::get_mut(&mut self.image).unwrap();
            let frame = self.gb.frame();
            for z in 0..(160 * 144) {
                let x = z % 160;
                let y = z / 160;
                image[(x, y)] = PALETTE[frame[z] as usize];
            }
        }
        // TODO repeated work in 1st repaint
        // TODO: should be in new?
        let texture: &mut egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("my-image", self.image.clone(), TextureOptions::NEAREST)
        });
        texture.set(self.image.clone(), TextureOptions::NEAREST);
        ui.image((texture.id(), 3. * texture.size_vec2()));
    }

    #[allow(dead_code)]
    fn debug_panel(&self, ui: &mut Ui) {
        ui.separator();
        egui::Grid::new("my_grid").striped(true).show(ui, |ui| {
            macro_rules! stat {
                ($label:literal : $fmt:literal, $value:expr) => {
                    ui.colored_label(Color32::LIGHT_GRAY, $label);
                    ui.monospace(format!($fmt, $value));
                    ui.code(stringify!($value));
                    ui.end_row();
                };
            }
            let time = ui.input(|i| i.time);
            let delta_time = ui.input(|i| i.unstable_dt) as f64;
            stat!("time"        : "{:.8}", time);
            stat!("dt"          : "{:.8}", delta_time);
            stat!("accum. time" : "{:.8}", self.accum_time);
            stat!("Ideal count" : "{:.3}", time / GB_FRAME_IN_SECONDS);
            stat!("Frame count" : "{}"   , self.gb_frame_count);
            stat!("UI updates"  : "{}"   , self.egui_frame_count);
        });
    }

    #[allow(dead_code)]
    fn sleep(&mut self, ctx: &Context, frame_start: f64, gb_frame_count_before: u64) {
        let mut _ccc = false;
        if self.gb_frame_count - gb_frame_count_before > 1 {
            info!("more than one gb_frame");
            _ccc = true;
        }
        let b = now();
        info!("a {:.8}", frame_start);
        info!("b {:.8}", b);
        let time_taken = (b - frame_start) / 1000.0;
        info!("time_taken {:.8}", time_taken);
        let time_taken = (time_taken * 1000.0).ceil() / 1000.0;
        if _ccc {
            info!("time_taken2 {:.8}", time_taken);
        }
        let sleep_time = GB_FRAME_IN_SECONDS - time_taken;
        info!("sleep_time {:.8}", sleep_time);
        if sleep_time < 0.0 {
            ctx.request_repaint();
        } else {
            // ctx.request_repaint_after(Duration::from_secs_f64(sleep_time - 0.005));
            ctx.request_repaint_after(Duration::from_secs_f64(sleep_time));
        }
    }
}

impl eframe::App for FPT {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        self.top_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("fpt");
            ui.label(self.egui_frame_count.to_string());
            ui.separator();
            // let frame_start = now();
            // let gb_frame_count_before = self.gb_frame_count;
            self.emulator(ui);
            // self.debug_panel(ui);
            // TODO: fix sleep timings for displays > 60hz. til then we burn cpu
            // self.sleep(ctx, frame_start, gb_frame_count_before);
            ctx.request_repaint();
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(egui::Vec2::new(500.0, 700.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(FPT::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(FPT::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
