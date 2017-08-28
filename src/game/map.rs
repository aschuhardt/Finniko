use std::marker::{Sync, Send};
use ndarray::{Axis, Array2};
use ndarray_parallel::prelude::*;
use super::tile::{Tile, TileType};
use super::{MAP_WIDTH, MAP_HEIGHT};

/// Generates and stores information about the game's current
/// arrangement of tiles and entities.  This ultimately makes
/// up the virtual space that the elements of the game interact
/// with, and within which they can be said to exist.
#[derive(Clone)]
pub struct Map {
    tiles: Array2<Tile>,
}

impl Map {
    /// Creates and returns a new instance of the Map struct.
    pub fn new() -> Map {
        Map { tiles: Map::create_tiles() }
    }

    /// Returns a reference to the element at the specified offset
    pub fn get_at(&self, position: [i32; 2]) -> Option<&Tile> {
        let (x, y) = (position[0], position[1]);
        if x >= 0 && x < MAP_WIDTH && y >= 0 && y < MAP_HEIGHT {
            Some(&self.tiles[[x as usize, y as usize]])
        } else {
            None
        }
    }

    // /// Sets the type of a tile at the specified position.
    // pub fn set_at(&mut self, position: [i32; 2], tile_type: TileType) {
    //     let (x, y) = (position[0], position[1]);
    //     if x >= 0 && x < MAP_WIDTH && y >= 0 && y < MAP_HEIGHT {
    //         self.tiles[[x as usize, y as usize]].tile_type = tile_type;
    //     } else {
    //         warn!(
    //             "Tried to set a tile's type at a position not within the map: {:?}",
    //             position
    //         );
    //     }
    // }

    /// Mutates the map's tiles in parallel using the provided closure operation.
    pub fn mut_parallel<F>(&mut self, op: F)
    where
        F: Fn(&Map, &mut Tile) + Sync + Send,
    {
        let m = self.clone();
        self.tiles.par_iter_mut().for_each(|t| op(&m, t));
    }

    /// Returns the width, in number of tiles, of the map.
    pub fn width(&self) -> usize {
        self.tiles.len_of(Axis(0))
    }

    /// Returns the height, in number of tiles, of the map.
    pub fn height(&self) -> usize {
        self.tiles.len_of(Axis(1))
    }

    fn create_tiles() -> Array2<Tile> {
        let mut tiles =
            Array2::<Tile>::from_elem((MAP_WIDTH as usize, MAP_HEIGHT as usize), Tile::new());
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                tiles[[x as usize, y as usize]].position = [x, y];
            }
        }
        tiles
    }
}
