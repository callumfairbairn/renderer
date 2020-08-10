use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM, DEFAULT_BACKGROUND_COORD};
use nannou::prelude::*;
use nannou::image::{DynamicImage};
use std::ops::{Index, IndexMut};
use crate::sprite::Sprite;

pub(crate) struct Grid(Vec<Vec<Sprite>>);

impl Grid {
    pub fn new(sprite_sheet: DynamicImage) -> Grid {
        let sprites_per_row = (WINDOW_RES / (SPRITE_RES * ZOOM)) as usize;
        let mut grid = Vec::new();
        for x in 0..sprites_per_row {
            let mut row = Vec::new();
            for y in 0..sprites_per_row {
                row.push(Sprite::new(DEFAULT_BACKGROUND_COORD, Point2::new(x as f32, y as f32), sprite_sheet.clone()))
            }
            grid.push(row);
        }
        Grid(grid)
    }

    pub fn draw_background(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        let texture = wgpu::Texture::from_image(app, &self[0][0].image);
        let Grid(vec) = self;
        for (x, row) in vec.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                draw.texture(&texture)
                    .x_y(-WINDOW_RES/2.0 + ((x as f32 + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((y as f32 + 0.5) * SPRITE_RES * ZOOM) );
            }
        }
        draw.to_frame(app, frame).unwrap();
    }
}



impl Index<usize> for Grid {
    type Output = Vec<Sprite>;
    fn index(&self, index: usize) -> &Vec<Sprite> {
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