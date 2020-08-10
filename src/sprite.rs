use crate::constants::{SPRITE_RES, ZOOM, WINDOW_RES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point2, wgpu};
use nannou::{App, Frame};
use nannou::wgpu::Texture;
use std::collections::HashMap;
use crate::{TileInfo};

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy)]
pub struct IPoint2 {
    pub x: i32,
    pub y: i32
}

#[derive(Clone)]
pub struct Sprite {
    pub sprite_sheet_coord: IPoint2,
    pub location: Point2
}

impl Sprite {
    pub fn new(sprite_sheet_coord: IPoint2, game_location: Point2, tile_info: &mut TileInfo, app: &App) -> Sprite {
        add_texture_to_map_if_needed(sprite_sheet_coord, tile_info, app);
        return Sprite {
            sprite_sheet_coord: sprite_sheet_coord.clone(),
            location: game_location
        }
    }

    pub fn draw_sprite(&self, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>) {
        let draw = app.draw();
        draw.texture(get_texture(&self.sprite_sheet_coord, coord_texture_map))
            .x_y(-WINDOW_RES/2.0 + ((self.location.x + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((self.location.y + 0.5) * SPRITE_RES * ZOOM) );
        draw.to_frame(app, frame).unwrap();
    }

    pub fn draw_sprites(sprites: Vec<Sprite>, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>) {
        let draw = app.draw();
        for sprite in sprites {
            draw.texture(get_texture(&sprite.sprite_sheet_coord, coord_texture_map))
                .x_y(-WINDOW_RES/2.0 + ((sprite.location.x as f32 + 0.5 ) * SPRITE_RES * ZOOM), WINDOW_RES/2.0 - ((sprite.location.y as f32 + 0.5) * SPRITE_RES * ZOOM) );
        }
        draw.to_frame(app, frame).unwrap();
    }
}

fn new_texture_from_coord(coord: IPoint2, sprite_sheet: &DynamicImage, app: &App) -> Texture {
    wgpu::Texture::from_image(
        app,
        &sprite_sheet.crop_imm(coord.x as u32 * SPRITE_RES as u32, coord.y as u32 * SPRITE_RES as u32, SPRITE_RES as u32, SPRITE_RES as u32)
            .resize( (SPRITE_RES * ZOOM) as u32, (SPRITE_RES * ZOOM) as u32, FilterType::Nearest)
    )
}

fn get_texture<'a>(coord: &'a IPoint2, coord_texture_map: &'a HashMap<IPoint2, Texture>) -> &'a Texture {
    let texture = coord_texture_map.get(coord);
    return if texture.is_some() { texture.unwrap() } else { panic!("Texture not found in model.coord_texture_map") }
}

fn add_texture_to_map_if_needed(coord: IPoint2, tile_info: &mut TileInfo, app: &App) {
    let texture = tile_info.coord_texture_map.get(&coord);
    if texture.is_none() { tile_info.coord_texture_map.insert(coord, new_texture_from_coord(coord, &tile_info.sprite_sheet, app)); }
}
