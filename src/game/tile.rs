use super::Drawable;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Wall(WallOrientation, WallType),
    Floor(FloorType),
}

#[derive(Clone)]
pub enum WallOrientation {
    /*0X0
      0X0
      0X0*/
    Vertical,

    /*000
      XXX
      000*/
    Horizontal,

    /*000
      0XX
      000*/
    EndLeftHorizontal,

    /*0X0
      0X0
      000*/
    EndLowerVertical,

    /*0X0
      0XX
      000*/
    CornerLeftLower,

    /*0X0
      XXX
      000*/
    TeeLower,

    /*0X0
      XX0
      000*/
    CornerRightLower,

    /*0X0
      XXX
      0X0*/
    Cross,

    /*000
      XX0
      000*/
    EndRightHorizontal,

    /*000
      0X0
      0X0*/
    EndUpperVertical,

    /*000
      0XX
      0X0*/
    CornerLeftUpper,

    /*000
      XXX
      0X0*/
    TeeUpper,

    /*000
      XX0
      0X0*/
    CornerRightUpper,

    /*0X0
      XX0
      0X0*/
    TeeRight,

    /*0X0
      0XX
      0X0*/
    TeeLeft,
}

#[derive(Clone)]
pub enum WallType {
    Basic,
}

#[derive(Clone)]
pub enum FloorType {
    Dirt,
    Stone,
    Grass,
    TileBlue,
    TileBeige,
    Concrete,
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
    /// Creates and returns a new instance of the Tile struct with its tile type
    /// set to `TileType::Empty`.
    pub fn new() -> Tile {
        Tile {
            tile_type: TileType::Floor(FloorType::Dirt),
            position: [0, 0],
        }
    }
}

impl Drawable for Tile {
    fn sprite_key(&self) -> String {
        let tt = &self.tile_type;
        match tt {
            &TileType::Empty => String::from("Void"),
            &TileType::Wall(_, _) => String::from(""),
            &TileType::Floor(ref style) => {
                match style {
                    &FloorType::Concrete => String::from("16 16 Light Stone"),
                    &FloorType::Dirt => String::from("16 16 Dark Sand"),
                    &FloorType::Grass => String::from("16 16 Light Grass"),
                    &FloorType::Stone => String::from("16 16 Stone Brick"),
                    &FloorType::TileBeige => String::from("biege brick floor"),
                    &FloorType::TileBlue => String::from("floor tile 2"),
                }
            }
        }
    }
}
