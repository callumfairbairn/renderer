use crate::constants::{SPRITE_RES, ZOOM, WINDOW_RES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point2, wgpu};
use nannou::{App, Frame};

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone)]
pub struct IPoint2 {
    pub x: i32,
    pub y: i32
}

#[derive(Clone)]
pub struct Sprite {
    pub image: DynamicImage,
    pub sprite_sheet_coord: IPoint2,
    pub location: Point2
}

impl Sprite {
    pub fn new(sprite_sheet_coord: IPoint2, game_location: Point2, sprite_sheet: DynamicImage) -> Sprite {
        return Sprite {
            image: new_sprite_image(sprite_sheet_coord.clone(), &sprite_sheet),
            sprite_sheet_coord: sprite_sheet_coord.clone(),
            location: game_location
        }
    }

    pub fn draw_sprite(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        draw.texture(&wgpu::Texture::from_image(app, &self.image))
            .x_y(-WINDOW_RES/2.0 + ((self.location.x + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((self.location.y + 0.5) * SPRITE_RES * ZOOM) );
        draw.to_frame(app, frame).unwrap();
    }

    pub fn draw_sprites(sprites: Vec<Sprite>, app: &App, frame: &Frame) {
        let draw = app.draw();
        let texture = &wgpu::Texture::from_image(app, &sprites[0].image);
        for sprite in sprites {
            draw.texture(texture)
                .x_y(-WINDOW_RES/2.0 + ((sprite.location.x as f32 + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((sprite.location.y as f32 + 0.5) * SPRITE_RES * ZOOM) );
        }
        draw.to_frame(app, frame).unwrap();
    }
}

fn new_sprite_image(coord: IPoint2, sprite_sheet: &DynamicImage) -> DynamicImage {
    sprite_sheet.crop_imm(coord.x as u32 * SPRITE_RES as u32, coord.y as u32 * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
        .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
}
