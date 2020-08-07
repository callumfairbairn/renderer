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

    let sprite = model.sprite_sheet.crop_imm((2.0 * SPRITE_RES) as u32, (2.0 * SPRITE_RES) as u32, SPRITE_RES as u32, SPRITE_RES as u32).resize( (SPRITE_RES * 2.0) as u32, (SPRITE_RES * 2.0) as u32, FilterType::Nearest);
    let sprite2 = model.sprite_sheet.crop_imm((3.0 * SPRITE_RES) as u32, (3.0 * SPRITE_RES) as u32, SPRITE_RES as u32, SPRITE_RES as u32).resize( (SPRITE_RES * 2.0) as u32, (SPRITE_RES * 2.0) as u32, FilterType::Nearest);

    // let draw = app.draw();
    let draw1 = Draw::new();
    draw1.texture(&wgpu::Texture::from_image(app, &sprite)).x_y(-WINDOW_RES/2.0 + SPRITE_RES, WINDOW_RES/2.0 - SPRITE_RES);

    let draw2 = Draw::new();
    draw2.texture(&wgpu::Texture::from_image(app, &sprite2)).x_y(-WINDOW_RES/2.0 + 3.0 * SPRITE_RES, WINDOW_RES/2.0 - SPRITE_RES);

    draw1.to_frame(app, &frame).unwrap();
    draw2.to_frame(app, &frame).unwrap();
}