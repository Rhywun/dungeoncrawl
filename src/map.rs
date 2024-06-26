use crate::prelude::*;

/// The number of tiles in the map.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// The kind of map tile.
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

/// The dungeon map.
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Is the [Point] inside the bounds of this [Map]?
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < SCREEN_WIDTH
            && point.y >= 0
            && point.y < SCREEN_HEIGHT
    }

    /// Can a player enter the tile at this [Point]?
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Return [Some] map index of this [Point] if it's in bounds, otherwise 
    /// [None].
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

/// Convert x/y coordinates into a single index.
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
