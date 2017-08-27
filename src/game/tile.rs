use super::Drawable;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall(WallOrientation, WallType),
    Floor(FloorType),
}

#[derive(Clone)]
pub enum WallOrientation {
    Face,
    Top,
}

#[derive(Clone)]
pub enum WallType {
    Brick,
    Stone,
    Adobe,
}

#[derive(Clone)]
pub enum FloorType {
    Dirt,
    Stone,
    Grass,
    Gravel,
    Wood,
    Overgrown,
    Water,
    Mud,
}

/// Represents a unit of space within the game's map.
#[derive(Clone)]
pub struct Tile {
    /// The type of tile that this instance represents.
    pub tile_type: TileType,

    /// The position of the tile in the 2D array that makes up the map.
    pub position: [i32; 2],
}

impl Tile {
    /// Creates and returns a new instance of the Tile struct
    pub fn new() -> Tile {
        Tile {
            tile_type: TileType::Floor(FloorType::Dirt),
            position: [0, 0],
        }
    }
}

impl Drawable for Tile {
    fn sprite_components(&self) -> (String, [f32; 4]) {
        let tt = &self.tile_type;
        match tt {
            &TileType::Empty => (String::from("void"), [0.0; 4]),
            &TileType::Wall(ref orientation, ref style) => {
                let key = match orientation {
                    &WallOrientation::Face => String::from("wall_face"),
                    &WallOrientation::Top => String::from("wall_top"),
                };
                let color = match style {
                    &WallType::Brick => [1.0, 1.0, 1.0, 1.0],
                    &WallType::Stone => [1.0, 1.0, 1.0, 1.0],
                    &WallType::Adobe => [1.0, 1.0, 1.0, 1.0],
                };
                (key, color)
            }
            &TileType::Floor(ref style) => {
                match style {
                    &FloorType::Dirt => (String::from("ground"), [0.525, 0.408, 0.29, 1.0]),
                    &FloorType::Stone => (String::from("brick"), [0.267, 0.427, 0.416, 1.0]),
                    &FloorType::Grass => (String::from("ground"), [0.224, 0.408, 0.247, 1.0]),
                    &FloorType::Gravel => (String::from("ground"), [0.176, 0.153, 0.137, 1.0]),
                    &FloorType::Wood => (String::from("wood"), [0.675, 0.49, 0.333, 1.0]),
                    &FloorType::Overgrown => (String::from("plant"), [0.078, 0.314, 0.165, 1.0]),
                    &FloorType::Water => (String::from("fluid"), [0.067, 0.224, 0.588, 1.0]),
                    &FloorType::Mud => (String::from("fluid"), [0.216, 0.149, 0.11, 1.0]),
                }
            }
        }
    }
}
