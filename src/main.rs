mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES, SPRITE_RES};
use nannou::image::{DynamicImage, open};
use nannou::image::imageops::{FilterType};

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

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let sprite = model.sprite_sheet.crop_imm((2.0 * SPRITE_RES) as u32, (2.0 * SPRITE_RES) as u32, SPRITE_RES as u32, SPRITE_RES as u32).resize( (SPRITE_RES * 2.0) as u32, (SPRITE_RES * 2.0) as u32, FilterType::Nearest);
    draw.texture(&wgpu::Texture::from_image(app, &sprite));

    draw.to_frame(app, &frame).unwrap();
}