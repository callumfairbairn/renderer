use crate::sprite::IPoint2;

pub(crate) const SPRITE_RES: f32 = 32.0;
pub(crate) const ZOOM: f32 = 2.0; // This must be a power of 2
pub(crate) const WINDOW_RES: f32 = 768.0; // This must be a multiple of SPRITE_RES

pub const DEFAULT_BACKGROUND_COORD: IPoint2 = IPoint2{ x: 8, y: 17};