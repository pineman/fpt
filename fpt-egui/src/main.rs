#![feature(lazy_cell)]
#![feature(array_chunks)]

use std::time::Duration;

use eframe::Frame;
use egui::{Color32, Context, TextureOptions, Ui};
use log::info;

use fpt::ppu::tile::Tile;

const GB_FRAME_IN_SECONDS: f64 = 0.016666666667;

const TEXTURE_SCALE_FACTOR: f32 = 3.0;

const GREY: Color32 = Color32::from_rgb(120, 120, 120);

const WIDTH: usize = fpt::ppu::WIDTH;
const HEIGHT: usize = fpt::ppu::HEIGHT;

const PALETTE: [Color32; 4] = [
    Color32::from_rgb(0, 63, 0),
    Color32::from_rgb(46, 115, 32),
    Color32::from_rgb(140, 191, 10),
    Color32::from_rgb(160, 207, 10),
];

// Debug view Tile Viewer (TV)
const TILE_SIZE: usize = fpt::ppu::tile::TILE_PIXEL_SIZE;
const TV_BORDER_SIZE: usize = 1;
const TV_COLS: usize = 16;
const TV_NUM_VBORDERS: usize = TV_COLS + 1;
const TV_ROWS: usize = 24;
const TV_NUM_HBORDERS: usize = TV_ROWS + 1;
const TV_X_SIZE: usize = TILE_SIZE * TV_COLS + TV_BORDER_SIZE * TV_NUM_VBORDERS;
const TV_Y_SIZE: usize = TILE_SIZE * TV_ROWS + TV_BORDER_SIZE * TV_NUM_HBORDERS;

// Debug view Background Map Viewer (BMV)
const BMV_BORDER_SIZE: usize = 1;
const BMV_TILES_PER: usize = 32;
const BMV_X_SIZE: usize = 256 + BMV_BORDER_SIZE * 2;
const BMV_Y_SIZE: usize = 256 + BMV_BORDER_SIZE * 2;

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

    bg_map: egui::ColorImage,
    bg_map_texture: Option<egui::TextureHandle>,

    gb: fpt::Gameboy,
    paused: bool,
}

impl Default for FPT {
    fn default() -> Self {
        Self {
            egui_frame_count: 0,
            gb_frame_count: 0,
            accum_time: 0.0,
            image: egui::ColorImage::new([WIDTH, HEIGHT], Color32::TRANSPARENT),
            texture: None,
            tiles: egui::ColorImage::new([TV_X_SIZE, TV_Y_SIZE], Color32::TRANSPARENT),
            tiles_texture: None,
            bg_map: egui::ColorImage::new([BMV_X_SIZE, BMV_Y_SIZE], Color32::TRANSPARENT),
            bg_map_texture: None,
            gb: fpt::Gameboy::new(),
            paused: true,
        }
    }
}

impl FPT {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = FPT::default();
        #[cfg(not(target_arch = "wasm32"))]
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
            for z in 0..(WIDTH * HEIGHT) {
                let x = z % WIDTH;
                let y = z / WIDTH;
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

    fn get_tile(&self, tile_i: usize) -> Tile {
        let start = 0x8000 + tile_i * 16;
        let end = 0x8000 + (tile_i + 1) * 16;
        let tile_vec = self.gb.bus().slice(start..end);
        let tile_slice: [u8; 16] = tile_vec.try_into().unwrap();
        Tile::load(&tile_slice)
    }

    fn debug_panel(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .id_source("debug_panel")
            .show(ui, |ui| {
                ui.heading("Debug");
                self.debug_info(ui);
                ui.checkbox(&mut self.paused, "Paused");
                for tile_i in 0..fpt::ppu::tile::NUM_TILES {
                    let tile = self.get_tile(tile_i);
                    for y in 0..TILE_SIZE {
                        let yy = y
                            + (tile_i / TV_COLS + 1) * TV_BORDER_SIZE
                            + (tile_i / TV_COLS) * TILE_SIZE;
                        for x in 0..TILE_SIZE {
                            let pixel = tile.get_pixel(y, x);
                            let xx = x
                                + (tile_i % TV_COLS + 1) * TV_BORDER_SIZE
                                + (tile_i % TV_COLS) * TILE_SIZE;
                            self.tiles[(xx, yy)] = PALETTE[pixel as usize];
                        }
                    }
                }
                for b in 0..TV_NUM_HBORDERS {
                    for y in 0..TV_BORDER_SIZE {
                        for x in 0..TV_X_SIZE {
                            self.tiles[(x, y + b * (TILE_SIZE + TV_BORDER_SIZE))] = GREY;
                        }
                    }
                }
                for b in 0..TV_NUM_VBORDERS {
                    for x in 0..TV_BORDER_SIZE {
                        for y in 0..TV_Y_SIZE {
                            self.tiles[(x + b * (TILE_SIZE + TV_BORDER_SIZE), y)] = GREY;
                        }
                    }
                }
                let texture: &mut egui::TextureHandle =
                    self.tiles_texture.get_or_insert_with(|| {
                        ui.ctx().load_texture(
                            "tile_viewer",
                            self.tiles.clone(),
                            TextureOptions::NEAREST,
                        )
                    });
                texture.set(self.tiles.clone(), TextureOptions::NEAREST);
                ui.image((texture.id(), 2. * texture.size_vec2()));

                // TODO we're assuming that the background map is the first one (LCDC.3 == 0)
                let bg_map = self.gb.bus().slice(0x9800..0x9C00);
                for (i, tile_address) in bg_map.iter().enumerate() {
                    let tile = self.get_tile(*tile_address as usize);
                    for y in 0..TILE_SIZE {
                        let yy = y + (i / BMV_TILES_PER) * TILE_SIZE + BMV_BORDER_SIZE;
                        for x in 0..TILE_SIZE {
                            let pixel = tile.get_pixel(y, x);
                            let xx = x + (i % BMV_TILES_PER) * TILE_SIZE + BMV_BORDER_SIZE;
                            self.bg_map[(xx, yy)] = PALETTE[pixel as usize];
                        }
                    }
                }
                // clear edges of bg_map viewer
                for x in 0..BMV_X_SIZE {
                    self.bg_map[(x, 0)] = Color32::TRANSPARENT;
                    self.bg_map[(x, BMV_Y_SIZE - 1)] = Color32::TRANSPARENT;
                }
                for y in 0..BMV_Y_SIZE {
                    self.bg_map[(0, y)] = Color32::TRANSPARENT;
                    self.bg_map[(BMV_X_SIZE - 1, y)] = Color32::TRANSPARENT;
                }
                let top = self.gb.bus().scy() as usize;
                let left = self.gb.bus().scx() as usize;
                let bottom = ((self.gb.bus().scy() as u16 + 143u16) % 256u16) as usize;
                let right = ((self.gb.bus().scx() as u16 + 159u16) % 256u16) as usize;
                let btop = top;
                let bleft = left;
                let bbottom = bottom + 2 * BMV_BORDER_SIZE;
                let bright = right + 2 * BMV_BORDER_SIZE;
                for x in bleft..(bright + 1) {
                    self.bg_map[(x, btop)] = GREY;
                    self.bg_map[(x, bbottom)] = GREY;
                }
                for y in btop..(bbottom + 1) {
                    self.bg_map[(bleft, y)] = GREY;
                    self.bg_map[(bright, y)] = GREY;
                }
                let texture: &mut egui::TextureHandle =
                    self.bg_map_texture.get_or_insert_with(|| {
                        ui.ctx().load_texture(
                            "bg_map_viewer",
                            self.bg_map.clone(),
                            TextureOptions::NEAREST,
                        )
                    });
                texture.set(self.bg_map.clone(), TextureOptions::NEAREST);
                ui.image((texture.id(), 1. * texture.size_vec2()));
            });
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
