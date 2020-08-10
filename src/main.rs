mod sprite;
mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES};
use nannou::image::{DynamicImage, open};
use crate::grid::Grid;
use crate::sprite::{Sprite, IPoint2};

struct KeyDownStatus {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}

struct Model {
    player_sprite: Sprite,
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

    let sprite_sheet: DynamicImage = open(app.assets_path().unwrap().join("spritesheet.png")).unwrap();
    let grid = Grid::new(sprite_sheet.clone());

    Model {
        player_sprite: Sprite::new(IPoint2{x: 4, y: 4}, Point2::new(4.0, 4.0), sprite_sheet.clone()),
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
        if model.player_sprite.location.y > 0.0 { model.player_sprite.location.y = model.player_sprite.location.y - 1.0 }
    }
    if model.key_down_status.s {
        if model.player_sprite.location.y < model.grid[0].len() as f32 - 1.0 { model.player_sprite.location.y = model.player_sprite.location.y + 1.0 }
    }
    if model.key_down_status.a {
        if model.player_sprite.location.x > 0.0 { model.player_sprite.location.x = model.player_sprite.location.x - 1.0 }
    }
    if model.key_down_status.d {
        if model.player_sprite.location.x < model.grid[0].len() as f32 - 1.0 { model.player_sprite.location.x = model.player_sprite.location.x + 1.0  }
    }
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



fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    model.grid.draw_background(app, &frame);
    model.player_sprite.draw_sprite(app, &frame)
}