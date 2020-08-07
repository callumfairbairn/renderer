use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM};
use nannou::prelude::*;
use nannou::image::{DynamicImage};
use std::ops::{Index, IndexMut};

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

    // pub fn update(&mut self, sprite: DynamicImage, location: Point2) {
    //     mem::replace(&mut self[location.x as usize][location.y as usize], &sprite)
    // }
}



impl Index<usize> for Grid {
    type Output = Vec<DynamicImage>;
    fn index(&self, index: usize) -> &Vec<DynamicImage> {
        let Grid(vec) = self;
        &vec[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Grid(vec) = self;
        &mut vec[index]
    }
}