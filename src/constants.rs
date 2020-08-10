use crate::tile::IPoint2;

pub(crate) const TILE_RES: f32 = 32.0;
pub(crate) const ZOOM: f32 = 2.0; // This must be a power of 2
pub(crate) const WINDOW_RES: f32 = 768.0; // This must be a multiple of TILE_RES

pub const DEFAULT_BACKGROUND_COORD: IPoint2 = IPoint2{ x: 7, y: 15};