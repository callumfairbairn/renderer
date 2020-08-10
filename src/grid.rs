use crate::constants::{WINDOW_RES, SPRITE_RES, ZOOM, DEFAULT_BACKGROUND_COORD};
use nannou::prelude::*;
use nannou::image::{DynamicImage};
use std::ops::{Index, IndexMut};
use crate::sprite::{Sprite, IPoint2};

pub(crate) struct Grid(Vec<Vec<Sprite>>);

impl Grid {
    pub fn new(sprite_sheet: DynamicImage, app: &App) -> Grid {
        let sprites_per_row = (WINDOW_RES / (SPRITE_RES * ZOOM)) as usize;
        let mut grid = Vec::new();
        for x in 0..sprites_per_row {
            let mut row = Vec::new();
            for y in 0..sprites_per_row {
                row.push(Sprite::new(DEFAULT_BACKGROUND_COORD, Point2::new(x as f32, y as f32), sprite_sheet.clone(), app))
            }
            grid.push(row);
        }
        Grid(grid)
    }

    pub fn draw_background(&self, app: &App, frame: &Frame) {
        let sprite_sheet_coords = self.unique_sprite_sheet_coords_in_grid();
        let Grid(vec) = self;

        for sprite_sheet_coord in sprite_sheet_coords {
            let mut sprites_with_coord = vec![];
            for row in vec {
                for sprite in row {
                    if sprite_sheet_coord == sprite.sprite_sheet_coord {
                        sprites_with_coord.push(sprite.clone());
                    }
                }
            }
            Sprite::draw_sprites(sprites_with_coord, app, frame);
        }
    }

    fn unique_sprite_sheet_coords_in_grid(&self) -> Vec<IPoint2> {
        //ssc = sprite sheet coord
        let mut sscs = vec![];
        let Grid(vec) = self;
        for row in vec {
            for sprite in row {
                let ssc = &sprite.sprite_sheet_coord;
                if !sscs.contains(ssc) {
                    sscs.push(ssc.clone());
                }
            }
        }
        sscs
    }

    // Replaces sprite in grid that has the same location as the one provided
    pub fn _add_sprite(&mut self, sprite: Sprite) {
        self[sprite.location.x as usize][sprite.location.y as usize] = sprite.clone();
    }

    pub fn add_sprites(&mut self, sprites: Vec<Sprite>) {
        for sprite in sprites {
            self[sprite.location.x as usize][sprite.location.y as usize] = sprite.clone();
        }
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