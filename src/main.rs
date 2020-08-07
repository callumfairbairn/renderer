mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::image::{DynamicImage, open};
use nannou::image::imageops::{FilterType};
use crate::grid::Grid;

struct Model {
    player_sprite: DynamicImage,
    player_location: Point2,
    grid: Grid
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES as u32, WINDOW_RES as u32).view(view).build().unwrap();
    let img_path = app.assets_path().unwrap().join("spritesheet.png");
    let sprite_sheet = open(img_path).unwrap();

    let background_sprite = new_sprite(4, 16, &sprite_sheet);

    Model {
        player_sprite: new_sprite(6, 2, &sprite_sheet),
        player_location: Point2::new(4.0, 4.0),
        grid: Grid::new(&background_sprite)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid.update(&model.player_sprite, model.player_location);
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn new_sprite(x: u32, y: u32, sprite_sheet: &DynamicImage) -> DynamicImage {
    sprite_sheet.crop_imm(x * SPRITE_RES as u32, y * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
    .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    model.grid.draw(app, &frame);
}