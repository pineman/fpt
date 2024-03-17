#![feature(lazy_cell)]
#![feature(array_chunks)]

use std::time::Duration;

use eframe::Frame;
use egui::{Color32, Context, TextureOptions, Ui};
use log::info;

use fpt::ppu::tile::Tile;

const GB_FRAME_IN_SECONDS: f64 = 0.016666666667;

const TEXTURE_SCALE_FACTOR: f32 = 3.0;

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

    image: egui::ColorImage,
    texture: Option<egui::TextureHandle>,

    tiles: egui::ColorImage,
    tiles_texture: Option<egui::TextureHandle>,

    gb: fpt::Gameboy,
    paused: bool,
}

impl Default for FPT {
    fn default() -> Self {
        Self {
            egui_frame_count: 0,
            gb_frame_count: 0,
            accum_time: 0.0,
            image: egui::ColorImage::new([160, 144], Color32::TRANSPARENT),
            texture: None,
            tiles: egui::ColorImage::new([8 * 16, 8 * 24], Color32::TRANSPARENT),
            tiles_texture: None,
            gb: fpt::Gameboy::new(),
            paused: true,
        }
    }
}

impl FPT {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = FPT::default();
        if std::env::var("CI").is_err() {
            const ROM_PATH: &str = "roms/Tetris_World_Rev_1.gb";
            if let Ok(rom) = std::fs::read(ROM_PATH) {
                app.gb.load_rom(&rom);
            } else {
                panic!("Unable to open {}", ROM_PATH);
            }
        }
        app
    }

    fn top_panel(&mut self, ctx: &Context) {
        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
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
            let frame = self.gb.frame();
            for z in 0..(160 * 144) {
                let x = z % 160;
                let y = z / 160;
                self.image[(x, y)] = PALETTE[frame[z] as usize];
            }
        }
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

    #[allow(dead_code)]
    fn debug_info(&self, ui: &mut Ui) {
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

    fn debug_panel(&mut self, ui: &mut Ui) {
        ui.heading("Debug");
        self.debug_info(ui);
        ui.checkbox(&mut self.paused, "Paused");

        // TODO: convert to one big texture so we can draw borders (and not use grid)
        egui::ScrollArea::vertical()
            .id_source("tile_viewer")
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);
                    for tile_i in 0..384 {
                        let start = 0x8000 + tile_i * 16;
                        let end = 0x8000 + (tile_i + 1) * 16;
                        let tile_vec = self.gb.bus().slice(start..end);
                        let tile_slice: [u8; 16] = tile_vec.try_into().unwrap();
                        let tile = Tile::load(&tile_slice);
                        for y in 0..8 {
                            for x in 0..8 {
                                let pixel = tile.get_pixel(y, x);
                                let xx = x + (tile_i % 16) * 8;
                                let yy = y + (tile_i / 16) * 8;
                                self.tiles[(xx, yy)] = PALETTE[pixel as usize];
                            }
                        }
                    }
                    let texture: &mut egui::TextureHandle =
                        self.tiles_texture.get_or_insert_with(|| {
                            ui.ctx().load_texture(
                                "tiles",
                                self.tiles.clone(),
                                TextureOptions::NEAREST,
                            )
                        });
                    texture.set(self.tiles.clone(), TextureOptions::NEAREST);
                    ui.image((texture.id(), 2. * texture.size_vec2()));
                });
            });

        ui.separator();

        // egui::ScrollArea::vertical()
        //     .id_source("bg_map_viewer")
        //     .show(ui, |ui| {
        //         ui.horizontal_wrapped(|ui| {
        //             ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);
        //             // TODO we're assuming that the background map is the first one (LCDC.3 == 0)
        //             let bg_map = self.gb.bus().slice(0x9800..0x9C00);
        //             for (i, tile_address) in bg_map.iter().enumerate() {
        //                 let texture = self.tiles_textures[*tile_address as usize]
        //                     .as_ref()
        //                     .unwrap();
        //                 ui.image((texture.id(), 2. * texture.size_vec2()));
        //                 if (i + 1) % 32 == 0 {
        //                     ui.end_row();
        //                 }
        //             }
        //         });
        //     });
    }

    fn central_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        // Emulator + screen
        // let frame_start = now();
        // let gb_frame_count_before = self.gb_frame_count;
        if !self.paused {
            self.emulator(ui);
        }
        // TODO repeated work in 1st repaint
        // TODO: should be in new?
        let texture: &mut egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("my-image", self.image.clone(), TextureOptions::NEAREST)
        });
        texture.set(self.image.clone(), TextureOptions::NEAREST);
        ui.image((texture.id(), TEXTURE_SCALE_FACTOR * texture.size_vec2()));
        ui.label(self.egui_frame_count.to_string());
        // TODO: fix sleep timings for displays > 60hz. til then we burn cpu
        // self.sleep(ctx, frame_start, gb_frame_count_before);
        ctx.request_repaint();
    }
}

impl eframe::App for FPT {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.top_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ctx, ui);
        });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                self.debug_panel(ui);
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
