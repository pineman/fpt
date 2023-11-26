#![feature(lazy_cell)]

use eframe::Frame;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use egui::{Color32, Context, Pos2, TextureOptions, Ui};
use log::info;

const GB_FRAME_IN_SECONDS: f64 = 0.016666666667;

#[cfg(target_arch = "wasm32")]
fn now() -> f64 {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
        .expect("failed to get performance from global object")
        .unchecked_into::<web_sys::Performance>()
        .now()
}

#[cfg(not(target_arch = "wasm32"))]
static APP_START: LazyLock<Instant> = LazyLock::new(Instant::now);

#[cfg(not(target_arch = "wasm32"))]
fn now() -> f64 {
    APP_START.elapsed().as_secs_f64() * 1000.0
}

pub struct TemplateApp {
    egui_frame_count: u64,
    gb_frame_count: u64,
    accum_time: f64,
    image: Arc<egui::ColorImage>,
    texture: Option<egui::TextureHandle>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            egui_frame_count: 0,
            gb_frame_count: 0,
            accum_time: 0.0,
            image: Arc::new(egui::ColorImage::new([160, 144], Color32::TRANSPARENT)),
            texture: None,
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

    fn top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
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

    fn game(&mut self, ui: &mut Ui) {
        let delta_time = ui.input(|i| i.unstable_dt) as f64;
        self.accum_time += delta_time;
        self.egui_frame_count += 1;
        // I hate this while, I much prefer the if
        // while self.accum_time >= GB_FRAME_IN_SECONDS {
        if self.accum_time >= GB_FRAME_IN_SECONDS {
            self.gb_frame_count += 1;
            self.accum_time -= GB_FRAME_IN_SECONDS;

            if let Some(image) = Arc::get_mut(&mut self.image) {
                // It all starts with this...
                static mut CHAOS_GAME: Pos2 = Pos2::new(80., 143.9);
                const STEPS: u64 = 5;
                for i in 0..STEPS {
                    let t = (self.gb_frame_count * STEPS + i) as f64 / 60.
                        * 0.33
                        * 2.
                        * std::f64::consts::PI;
                    let r = (200. + (t * 1.01 + 0.).sin() * 40.) as u8;
                    let g = (180. + (t * 0.08 + 1.).sin() * 70.) as u8;
                    let b = (40.0 + (t * 0.57 + 2.).sin() * 20.) as u8;
                    let (x, y) = unsafe {
                        CHAOS_GAME = CHAOS_GAME.lerp(
                            match ((r as u32) + (g as u32) + (b as u32)) % 3 {
                                0 => Pos2::new(0., 0.),
                                1 => Pos2::new(0., 143.9),
                                _ => Pos2::new(159.9, 143.9),
                            },
                            0.5,
                        );
                        (CHAOS_GAME.x.floor() as usize, CHAOS_GAME.y.floor() as usize)
                    };
                    image[(x, y)] = Color32::from_rgb(r, g, b);
                    let (x, y) = (159 - x, 143 - y);
                    image[(x, y)] = Color32::from_rgb(b, g, r);
                }
            }
        }
        let texture: &mut egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx()
                .load_texture("my-image", self.image.clone(), TextureOptions::NEAREST)
        });
        // TODO repeated work in 1st repaint
        texture.set(self.image.clone(), TextureOptions::NEAREST);
        ui.image((texture.id(), 3. * texture.size_vec2()));
    }

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
            ctx.request_repaint_after(Duration::from_secs_f64(sleep_time - 0.005));
            // ctx.request_repaint_after(Duration::from_secs_f64(sleep_time));
        }
    }

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
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.top_panel(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("fpt");
            ui.add(egui::Label::new(self.egui_frame_count.to_string()));
            ui.separator();
            // vars for sleep
            // let frame_start = now();
            // let gb_frame_count_before = self.gb_frame_count;
            self.game(ui);
            self.debug_panel(ui);
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
        initial_window_size: Some([500.0, 700.0].into()),
        min_window_size: Some([500.0, 700.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
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
                Box::new(|cc| Box::new(TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
