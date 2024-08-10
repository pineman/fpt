#![feature(array_chunks)]

use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use clap::{Parser, ValueEnum};
use eframe::Frame;
#[allow(unused_imports)]
use egui::{
    menu, CentralPanel, Color32, ColorImage, Context, Grid, Key, RichText, ScrollArea, SidePanel,
    TextureHandle, TextureOptions, TopBottomPanel, Ui, Vec2, ViewportBuilder, ViewportCommand,
};
use fpt::debug_interface::DebugEvent;
use fpt::memory::Buttons;
use fpt::ppu::tile::Tile;
use fpt::{bw, DebugCmd, DebugInterface, Gameboy};
use log::info;

// TODO: the gameboy doesn't run at exactly 60fps
const SIXTY_FPS_FRAMETIME: f64 = 0.016666666667;
const T_CYCLE: f64 = 0.0000002384185791015625;

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
const TV_TEXTURE_SCALE: f32 = 1.0;

// Debug view Background Map Viewer (BMV)
const BMV_BORDER_SIZE: usize = 1;
const BMV_TILES_PER: usize = 32;
const BMV_X_SIZE: usize = 256 + BMV_BORDER_SIZE * 2;
const BMV_Y_SIZE: usize = 256 + BMV_BORDER_SIZE * 2;
const BMV_TEXTURE_SCALE: f32 = 1.0;

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

#[derive(Default)]
struct DebugConsole {
    console: Vec<String>,
    command: String,
    last_command: String,
    was_focused: bool,
}

pub struct FPT {
    gb: Gameboy,
    cycles_since_last_frame: u32,
    accum_time: f64,
    egui_frame_count: u64,
    gb_frame_count: u64,
    bootrom: Option<BootromToFake>,

    slow_factor: f64,
    // Debug Console (DC)
    debug_console: DebugConsole,

    image: ColorImage,
    texture: Option<TextureHandle>,

    tiles: ColorImage,
    tiles_texture: Option<TextureHandle>,

    bg_map: ColorImage,
    bg_map_texture: Option<TextureHandle>,

    #[allow(dead_code)]
    rom_channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
}

impl Default for FPT {
    fn default() -> Self {
        Self {
            gb: Gameboy::new(),
            cycles_since_last_frame: 0,
            accum_time: 0.0,
            egui_frame_count: 0,
            gb_frame_count: 0,
            bootrom: None,

            slow_factor: 1.0,

            debug_console: DebugConsole::default(),

            image: ColorImage::new([WIDTH, HEIGHT], Color32::TRANSPARENT),
            texture: None,

            tiles: ColorImage::new([TV_X_SIZE, TV_Y_SIZE], Color32::TRANSPARENT),
            tiles_texture: None,

            bg_map: ColorImage::new([BMV_X_SIZE, BMV_Y_SIZE], Color32::TRANSPARENT),
            bg_map_texture: None,

            rom_channel: channel(),
        }
    }
}

impl FPT {
    /// Called once before the first frame.
    #[allow(unused_variables)]
    fn new(_cc: &eframe::CreationContext, bootrom: Option<BootromToFake>, rom_path: &str) -> Self {
        let mut fpt = FPT {
            bootrom: bootrom.clone(),
            ..Default::default()
        };
        if cfg!(target_arch = "wasm32") {
            fpt.gb.cpu_mut().set_paused(true);
        } else {
            if std::env::var("CI").is_err() {
                if let Ok(rom) = std::fs::read(rom_path) {
                    fpt.gb.load_rom(&rom);
                } else {
                    panic!("Unable to open {}", rom_path);
                }
            }
        }
        // XXX duplicated logic from fpt-cli main.rs
        if let Some(BootromToFake::DMG0) = bootrom {
            fpt.gb.boot_fake();
        } else {
            fpt.gb.boot_real();
        }
        fpt
    }

    fn emulator(&mut self, ui: &mut Ui) -> Option<fpt::ppu::Frame> {
        self.egui_frame_count += 1;
        let mut frame: Option<fpt::ppu::Frame> = None;
        let delta_time = ui.input(|i| i.unstable_dt) as f64;
        self.accum_time += delta_time;
        // TODO: should limit to a 60fps frame, taking self.slow_factor into account (so 10 frames at 0.1, or 0.1 frames at 10)
        let cycles_want = self.accum_time.div_euclid(T_CYCLE * self.slow_factor) as u32;
        let mut cycles_ran = 0;
        while cycles_ran < cycles_want && !self.gb.paused() {
            let cycles = self.gb.step() as u32;
            self.cycles_since_last_frame += cycles;
            if self.cycles_since_last_frame >= self.gb.cycles_in_one_frame() {
                frame = Some(*self.gb.get_frame()); // Copies the whole [u8; WIDTH * HEIGHT] into frame
                self.gb_frame_count += 1;
                self.cycles_since_last_frame = 0;
            }
            cycles_ran += cycles;
        }
        self.accum_time -= cycles_ran as f64 * T_CYCLE * self.slow_factor;
        frame
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
        let sleep_time = SIXTY_FPS_FRAMETIME - time_taken;
        info!("sleep_time {:.8}", sleep_time);
        if sleep_time < 0.0 {
            ctx.request_repaint();
        } else {
            // ctx.request_repaint_after(Duration::from_secs_f64(sleep_time - 0.005));
            ctx.request_repaint_after(Duration::from_secs_f64(sleep_time));
        }
    }

    #[allow(dead_code)]
    fn timing_info(&self, ui: &mut Ui) {
        ui.collapsing("Timing", |ui| {
            Grid::new("my_grid").striped(true).show(ui, |ui| {
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
                stat!("time"        : "{:>9.3}" , time);
                stat!("dt"          : "{:>9.3}" , delta_time);
                stat!("accum. time" : "{:>9.3}" , self.accum_time);
                stat!("Ideal count" : "{:>9.3}" , time / SIXTY_FPS_FRAMETIME);
                stat!("Frame count" : "{:>5}"   , self.gb_frame_count);
                stat!("UI updates"  : "{:>5}"   , self.egui_frame_count);
            });
        });
    }

    fn get_tile(&self, tile_i: usize) -> Tile {
        let bus = self.gb.bus();
        let tile_start = 0x8000 + 16 * tile_i;
        bus.with_span(tile_start, Tile::load)
    }

    fn debug_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.collapsing("VRAM", |ui| {
            ui.horizontal_wrapped(|ui| self.vram_viewer(ui));
            ui.horizontal(|ui| self.vram_registers(ui));
        });
        ui.horizontal(|ui| {
            let paused = self.gb.paused();
            if ui
                .button(if paused { "Continue" } else { "Pause" })
                .clicked()
            {
                self.gb.set_paused(!paused);
            }
            ui.horizontal(|ui| {
                ui.monospace("Slow factor:");
                ui.radio_value(&mut self.slow_factor, 0.1f64, "0.1");
                ui.radio_value(&mut self.slow_factor, 1f64, "1");
                ui.radio_value(&mut self.slow_factor, 60f64, "60");
                ui.radio_value(&mut self.slow_factor, 1000f64, "1000");
                ui.radio_value(&mut self.slow_factor, 1e5f64, "1e5");
                ui.radio_value(&mut self.slow_factor, 1e6f64, "1e6");
            });
        });
        ui.horizontal_wrapped(|ui| {
            macro_rules! cpu_register {
                ($ui:expr, $high_label:literal : $high_value:expr, $low_label:literal : $low_value:expr) => {
                    $ui.colored_label(Color32::LIGHT_BLUE, $high_label);
                    $ui.monospace(format!("{:08b}", $high_value));
                    $ui.code(format!("{:04X}", bw::word16($high_value, $low_value)));
                    $ui.monospace(format!("{:08b}", $low_value));
                    $ui.colored_label(Color32::LIGHT_BLUE, $low_label);
                }
            }
            let cpu = self.gb.cpu();
            ui.vertical(|ui| {
                Grid::new("cpu_registers_a-e").num_columns(4).min_col_width(10.0).striped(true).show(ui, |ui| {
                    cpu_register!(ui, "A": cpu.a(), "F": cpu.f()); ui.end_row();
                    cpu_register!(ui, "B": cpu.b(), "C": cpu.c()); ui.end_row();
                    cpu_register!(ui, "D": cpu.d(), "E": cpu.e()); ui.end_row();
                    cpu_register!(ui, "H": cpu.h(), "L": cpu.l()); ui.end_row();
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                Grid::new("flags").num_columns(1).min_col_width(10.0).striped(true).show(ui, |ui| {
                    ui.colored_label(Color32::LIGHT_BLUE, "C");
                    ui.code(if cpu.c_flag() { "1" } else { "0" });
                    ui.colored_label(Color32::LIGHT_BLUE, "H");
                    ui.code(if cpu.h_flag() { "1" } else { "0" });
                    ui.colored_label(Color32::LIGHT_BLUE, "N");
                    ui.code(if cpu.n_flag() { "1" } else { "0" });
                    ui.colored_label(Color32::LIGHT_BLUE, "Z");
                    ui.code(if cpu.z_flag() { "1" } else { "0" });
                });
                ui.horizontal(|ui| {
                    ui.colored_label(Color32::LIGHT_BLUE, "SP");
                    ui.code(format!("{:#06X}", cpu.sp()));
                });
                ui.horizontal(|ui| {
                    ui.colored_label(Color32::LIGHT_BLUE, "PC");
                    ui.code(format!("{:#06X}", cpu.pc()));
                });
            });
        });
        // TODO: scroll into line of current pc (need to find index)
        // TODO: differentiate current pc
        ui.collapsing("Code", |ui| {
            ui.vertical(|ui| {
                let mem = self.gb.bus().memory();
                let code_flat: Vec<&String> = mem.code_listing().iter().flatten().collect();
                if ui.button("Dump").clicked() {
                    println!(
                        "{}",
                        code_flat
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>()
                            .join("\n")
                    );
                }
                ScrollArea::vertical().show_rows(
                    ui,
                    ui.text_style_height(&egui::TextStyle::Body),
                    code_flat.len(),
                    |ui, row_range| {
                        for row in row_range {
                            ui.label(RichText::new(code_flat[row].clone()).monospace());
                        }
                    },
                );
            });
        });
        ui.collapsing("Console", |ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                // TODO: dirty hack to make the console input always stick to the bottom
                .max_height(ui.available_rect_before_wrap().height() - 24.0)
                .show_rows(
                    ui,
                    ui.text_style_height(&egui::TextStyle::Body),
                    self.debug_console.console.len(),
                    |ui, row_range| {
                        for row in row_range {
                            ui.label(
                                RichText::new(self.debug_console.console[row].clone()).monospace(),
                            );
                        }
                    },
                );
            let edit = egui::TextEdit::multiline(&mut self.debug_console.command)
                .desired_rows(1)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY);
            let response = ui.add(edit);
            if self.debug_console.was_focused {
                response.request_focus();
                self.debug_console.was_focused = false;
            }

            let debug_events: &mut VecDeque<DebugEvent> = self.gb.get_debug_events();
            while !debug_events.is_empty() {
                let event = debug_events.pop_front();
                self.debug_console
                    .console
                    .push(format!("{}", event.unwrap()));
            }

            if response.has_focus() && ctx.input(|i| i.key_pressed(Key::Enter)) {
                self.debug_console.was_focused = true;
                self.debug_console.command = self.debug_console.command.trim().to_string();
                if self.debug_console.command.is_empty() {
                    self.debug_console
                        .command
                        .clone_from(&self.debug_console.last_command);
                }
                self.debug_console
                    .console
                    .push(format!("> {}", self.debug_console.command));

                let command = DebugCmd::from_string(&self.debug_console.command);
                let event = match command {
                    Some(command) => self
                        .gb
                        .debug_cmd(&command)
                        .map(|x| format!("{}", x))
                        .unwrap_or("Unrecognized command".to_string()),
                    None => "Cannot parse command".to_string(),
                };
                self.debug_console.console.push(event);

                self.debug_console
                    .last_command
                    .clone_from(&self.debug_console.command);
                self.debug_console.command = String::new();
            }
        });
    }

    fn vram_registers(&mut self, ui: &mut Ui) {
        let bus = self.gb.bus();
        Grid::new("VRAM-registers-parent")
            .striped(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    Grid::new("VRAM-registers-1").striped(true).show(ui, |ui| {
                        ui.monospace("LCDC");
                        ui.monospace(format!("{:08b}", bus.lcdc()));
                        ui.end_row();
                        ui.monospace("STAT");
                        ui.monospace(format!("{:08b}", bus.stat()));
                        ui.end_row();
                    });
                    ui.separator();
                });
                ui.horizontal(|ui| {
                    Grid::new("VRAM-registers-2").striped(true).show(ui, |ui| {
                        ui.monospace("LY");
                        ui.monospace(format!("{:08b}", bus.ly()));
                        ui.end_row();
                        ui.monospace("LYC");
                        ui.monospace(format!("{:08b}", bus.lyc()));
                        ui.end_row();
                    });
                    ui.separator();
                });
                ui.horizontal(|ui| {
                    Grid::new("VRAM-registers-3").striped(true).show(ui, |ui| {
                        ui.monospace("SCX");
                        ui.monospace(format!("{:08b}", bus.scx()));
                        ui.end_row();
                        ui.monospace("SCY");
                        ui.monospace(format!("{:08b}", bus.scy()));
                        ui.end_row();
                    });
                });
            });
    }

    fn vram_viewer(&mut self, ui: &mut Ui) {
        for tile_i in 0..fpt::ppu::tile::NUM_TILES {
            let tile = self.get_tile(tile_i);
            for y in 0..TILE_SIZE {
                let yy =
                    y + (tile_i / TV_COLS + 1) * TV_BORDER_SIZE + (tile_i / TV_COLS) * TILE_SIZE;
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
        let texture: &mut TextureHandle = self.tiles_texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("tile_viewer", self.tiles.clone(), TextureOptions::NEAREST)
        });
        texture.set(self.tiles.clone(), TextureOptions::NEAREST);
        ui.vertical(|ui| {
            ui.label("Tile data");
            ui.image((texture.id(), TV_TEXTURE_SCALE * texture.size_vec2()));
        });

        let lcdc = self.gb.bus().lcdc();
        let bg_map_area = match bw::test_bit8::<3>(lcdc) {
            false => 0x9800..0x9C00,
            true => 0x9C00..0xA000,
        };
        let bg_map_iter = bg_map_area.map(|addr| self.gb.bus().read(addr));

        for (i, tile_i) in bg_map_iter.enumerate() {
            let tile = self.get_tile(tile_i as usize);
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
        let texture: &mut TextureHandle = self.bg_map_texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "bg_map_viewer",
                self.bg_map.clone(),
                TextureOptions::NEAREST,
            )
        });
        texture.set(self.bg_map.clone(), TextureOptions::NEAREST);
        ui.vertical(|ui| {
            ui.label("BG Map");
            ui.image((texture.id(), BMV_TEXTURE_SCALE * texture.size_vec2()));
        });
    }

    fn central_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        if !self.gb.cpu().paused() {
            // TODO: only capture buttons if debug console is not focused
            let buttons = Buttons {
                a: ctx.input(|i| i.key_down(Key::A)),
                b: ctx.input(|i| i.key_down(Key::S)),
                select: ctx.input(|i| i.key_down(Key::D)),
                start: ctx.input(|i| i.key_down(Key::F)),
                up: ctx.input(|i| i.key_down(Key::K)),
                down: ctx.input(|i| i.key_down(Key::J)),
                left: ctx.input(|i| i.key_down(Key::H)),
                right: ctx.input(|i| i.key_down(Key::L)),
            };
            self.gb.set_buttons(&buttons);
            let frame = self.emulator(ui);
            if let Some(frame) = frame {
                for (i, &gb_pixel) in frame.iter().enumerate() {
                    self.image.pixels[i] = PALETTE[gb_pixel as usize];
                }
            }
        }
        // TODO repeated work in 1st repaint
        // TODO: should be in new?
        let texture: &mut TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("my-image", self.image.clone(), TextureOptions::NEAREST)
        });
        texture.set(self.image.clone(), TextureOptions::NEAREST);
        ui.image((texture.id(), TEXTURE_SCALE_FACTOR * texture.size_vec2()));
        // TODO: fix sleep timings for displays > 60hz. til then we burn cpu
        // self.sleep(ctx, frame_start, gb_frame_count_before);
        ctx.request_repaint();
    }

    #[cfg(target_arch = "wasm32")]
    // https://github.com/woelper/egui_pick_file/blob/main/src/app.rs
    fn load_rom(&mut self, ui: &mut Ui) {
        if let Ok(text) = self.rom_channel.1.try_recv() {
            self.gb.load_rom(&text);
            if let Some(BootromToFake::DMG0) = bootrom {
                self..gb.boot_fake();
            } else {
                self.gb.boot_real();
            }
            self.gb.cpu_mut().set_paused(false);
        }
        if ui.button("Load rom").clicked() {
            let sender = self.rom_channel.0.clone();
            let task = rfd::AsyncFileDialog::new().pick_file();
            let ctx = ui.ctx().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let file = task.await;
                if let Some(file) = file {
                    let text = file.read().await;
                    let _ = sender.send(text);
                    ctx.request_repaint();
                }
            });
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_rom(&mut self, ui: &mut Ui) {
        if ui.button("Load rom").clicked() {
            let file = rfd::FileDialog::new().pick_file();
            if let Some(file) = file {
                let text: Box<[u8]> = std::fs::read(file).unwrap().into_boxed_slice();
                self.gb.load_rom(&text);
                if let Some(BootromToFake::DMG0) = self.bootrom {
                    self.gb.boot_fake();
                } else {
                    self.gb.boot_real();
                }
                self.gb.cpu_mut().set_paused(false);
            }
        }
    }
}

impl eframe::App for FPT {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                self.load_rom(ui);
                self.timing_info(ui);
                self.debug_panel(ctx, ui);
            });

        CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ctx, ui);
        });
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Apply known CPU and hardware register values of a well-known bootrom when it
    /// hands off the execution to the cartridge's code. This skips emulating a bootrom.
    #[arg(short, long)]
    fake_bootrom: Option<BootromToFake>,
    /// ROM path
    rom: Option<String>,
}

// XXX duplicated struct from fpt-cli's main.rs
#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum BootromToFake {
    DMG0,
}

/// Desktop entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let cli = Cli::parse();
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::new(950.0, 700.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "FPT",
        native_options,
        Box::new(|cc| {
            Box::new(FPT::new(
                cc,
                cli.fake_bootrom,
                &cli.rom.unwrap_or("roms/Tetris_World_Rev_1.gb".to_string()),
            ))
        }),
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
                Box::new(|cc| Box::new(FPT::new(cc, None, ""))),
            )
            .await
            .expect("failed to start eframe");
    });
}
