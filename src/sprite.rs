use crate::constants::{SPRITE_RES, ZOOM, WINDOW_RES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point2, wgpu};
use nannou::{App, Frame};

pub struct IPoint2 {
    pub x: i32,
    pub y: i32
}

pub struct Sprite {
    pub image: DynamicImage,
    pub location: Point2
}

impl Sprite {
    pub fn new(sprite_sheet_coord: IPoint2, game_location: Point2, sprite_sheet: DynamicImage) -> Sprite {
        return Sprite {
            image: new_sprite_image(sprite_sheet_coord.x as u32, sprite_sheet_coord.y as u32, &sprite_sheet),
            location: game_location
        }
    }

    pub fn draw_sprite(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        draw.texture(&wgpu::Texture::from_image(app, &self.image))
            .x_y(-WINDOW_RES/2.0 + ((self.location.x + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((self.location.y + 0.5) * SPRITE_RES * ZOOM) );
        draw.to_frame(app, frame).unwrap();
    }
}

fn new_sprite_image(x: u32, y: u32, sprite_sheet: &DynamicImage) -> DynamicImage {
    sprite_sheet.crop_imm(x * SPRITE_RES as u32, y * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
        .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
}