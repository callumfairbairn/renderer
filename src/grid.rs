use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::prelude::*;
use nannou::image::{DynamicImage};

pub(crate) struct Grid(Vec<Vec<DynamicImage>>);

impl Grid {
    pub fn new(default_sprite: DynamicImage) -> Grid {
        let sprites_per_row = (WINDOW_RES / (SPRITE_RES * ZOOM)) as usize;
        Grid(vec![vec![default_sprite; sprites_per_row]; sprites_per_row])
    }

    pub fn draw(&self, app: &App, frame: &Frame) {
        let Grid(vec) = self;
        for (x, row) in vec.iter().enumerate() {
            for (y, sprite) in row.iter().enumerate() {
                let draw = Draw::new();
                draw.texture(&wgpu::Texture::from_image(app, &sprite))
                    .x_y(-WINDOW_RES/2.0 + ((x as f32 + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((y as f32 + 0.5) * SPRITE_RES * ZOOM) );
                draw.to_frame(app, frame).unwrap();
            }
        }
    }
}