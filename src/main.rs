mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::image::{DynamicImage, open};
use nannou::image::imageops::{FilterType};
use crate::grid::Grid;

struct Model {
    sprite_sheet: DynamicImage
}

fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES as u32, WINDOW_RES as u32).view(view).build().unwrap();
    let img_path = app.assets_path().unwrap().join("spritesheet.png");
    let image = open(img_path).unwrap();
    Model { sprite_sheet: { image } }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn new_sprite(x: u32, y: u32, sprite_sheet: &DynamicImage) -> DynamicImage {
    sprite_sheet.crop_imm(x * SPRITE_RES as u32, y * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
    .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let sprite1= new_sprite(4, 16, &model.sprite_sheet);

    let grid = Grid::new(sprite1);
    grid.draw(app, &frame);
}