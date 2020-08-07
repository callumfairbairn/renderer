mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::image::{DynamicImage, open};
use nannou::image::imageops::{FilterType};
use crate::grid::Grid;

struct KeyDownStatus {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}

struct Model {
    background_sprite: DynamicImage,
    player_sprite: DynamicImage,
    player_location: Point2,
    grid: Grid,
    key_down_status: KeyDownStatus
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES as u32, WINDOW_RES as u32).event(event).view(view).build().unwrap();
    let img_path = app.assets_path().unwrap().join("spritesheet.png");
    let sprite_sheet = open(img_path).unwrap();

    let background_sprite = new_sprite(4, 16, &sprite_sheet);
    let grid = Grid::new(&background_sprite);

    Model {
        background_sprite,
        player_sprite: new_sprite(6, 2, &sprite_sheet),
        player_location: Point2::new(4.0, 4.0),
        grid,
        key_down_status: KeyDownStatus {
            w: false,
            s: false,
            a: false,
            d: false,
    },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.key_down_status.w {
        if model.player_location.y > 0.0 { model.player_location.y = model.player_location.y - 1.0 }
    }
    if model.key_down_status.s {
        if model.player_location.y < model.grid[0].len() as f32 - 1.0 { model.player_location.y = model.player_location.y + 1.0 }
    }
    if model.key_down_status.a {
        if model.player_location.x > 0.0 { model.player_location.x = model.player_location.x - 1.0 }
    }
    if model.key_down_status.d {
        if model.player_location.x < model.grid[0].len() as f32 - 1.0 { model.player_location.x = model.player_location.x + 1.0  }
    }
    model.grid = Grid::update(&model.background_sprite, &model.player_sprite, model.player_location);
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            update_key_down_status(key, model, true)
        }
        KeyReleased(key) => {
            update_key_down_status(key, model, false)
        }
        _ => {}
    }
}

fn update_key_down_status(key: Key, model: &mut Model, key_down: bool) {
    match key {
        Key::W => model.key_down_status.w = key_down,
        Key::S => model.key_down_status.s = key_down,
        Key::A => model.key_down_status.a = key_down,
        Key::D => model.key_down_status.d = key_down,
        _ => {}
    }
}

fn new_sprite(x: u32, y: u32, sprite_sheet: &DynamicImage) -> DynamicImage {
    sprite_sheet.crop_imm(x * SPRITE_RES as u32, y * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
    .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    model.grid.draw(app, &frame);
}