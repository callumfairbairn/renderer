mod update;
mod event;
mod tile;
mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES_X, WINDOW_RES_Y};
use nannou::image::{DynamicImage, open};
use nannou::wgpu::Texture;
use crate::grid::Grid;
use crate::tile::{Tile, IPoint2};
use crate::event::{event, KeyDownStatus};
use crate::update::update;
use std::collections::HashMap;

pub struct TileInfo {
    coord_texture_map: HashMap<IPoint2, Texture>,
    tile_sheet: DynamicImage
}

pub struct Model {
    player: Tile,
    grid: Grid,
    key_down_status: KeyDownStatus,
    tile_info: TileInfo
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES_X as u32, WINDOW_RES_Y as u32).event(event).view(view).build().unwrap();

    let tile_sheet = open(app.assets_path().unwrap().join("tilesheet.png")).unwrap();
    let coord_texture_map = HashMap::new();
    let mut tile_info = TileInfo{ tile_sheet, coord_texture_map };
    let mut grid = Grid::new(IPoint2{ x: 8, y: 16 }, &mut tile_info, app);

    let mut additional_background_tiles = vec![];
    additional_background_tiles.push(Tile::new(IPoint2{x: 9, y: 16}, Point2::new(0.0, 0.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 9, y: 16}, Point2::new(2.0, 1.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 9, y: 16}, Point2::new(7.0, 4.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 10, y: 16}, Point2::new(6.0, 6.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 10, y: 16}, Point2::new(12.0, 9.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 10, y: 16}, Point2::new(0.0, 4.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 11, y: 16}, Point2::new(3.0, 9.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 11, y: 16}, Point2::new(10.0, 5.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 11, y: 16}, Point2::new(9.0, 7.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 11, y: 16}, Point2::new(2.0, 2.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 12, y: 16}, Point2::new(1.0, 4.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 12, y: 16}, Point2::new(3.0, 5.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 12, y: 16}, Point2::new(8.0, 8.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 12, y: 16}, Point2::new(11.0, 2.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 13, y: 16}, Point2::new(12.0, 4.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 13, y: 16}, Point2::new(6.0, 0.0), &mut tile_info, app));
    additional_background_tiles.push(Tile::new(IPoint2{x: 14, y: 16}, Point2::new(7.0, 2.0), &mut tile_info, app));
    grid.add_tiles(additional_background_tiles);

    Model {
        player: Tile::new(IPoint2{x: 4, y: 4}, Point2::new(4.0, 4.0), &mut tile_info, app),
        grid,
        key_down_status: KeyDownStatus::new(),
        tile_info
    }
}



fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    model.grid.draw_background(app, &frame, &model.tile_info.coord_texture_map);
    model.player.draw_tile(app, &frame, &model.tile_info.coord_texture_map)
}