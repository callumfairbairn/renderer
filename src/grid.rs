use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::prelude::*;
use nannou::image::{DynamicImage};

pub(crate) struct Grid(Vec<Vec<DynamicImage>>);

impl Grid {
    pub fn new(default_sprite: DynamicImage) -> Grid {
        let number_of_sprites = (WINDOW_RES / (SPRITE_RES * ZOOM)) as usize;
        Grid(vec![vec![default_sprite; number_of_sprites]; number_of_sprites])
    }

    pub fn draw(&self, app: &App, frame: &Frame) {
        let Grid(vec) = self;
        for (x, row) in vec.iter().enumerate() {
            for (y, sprite) in row.iter().enumerate() {
                let draw = Draw::new();
                draw.texture(&wgpu::Texture::from_image(app, &sprite))
                    .x_y(-WINDOW_RES/ZOOM + ((x as f32 * ZOOM + 1.0) * SPRITE_RES), WINDOW_RES/ZOOM - ((y as f32 * ZOOM + 1.0) * SPRITE_RES) );
                draw.to_frame(app, frame).unwrap();
            }
        }
    }
}