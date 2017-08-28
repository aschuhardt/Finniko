use rand;
use std::time::SystemTime;
use noise::{Seedable, Fbm, RangeFunction, Worley, NoiseModule};
use super::map::Map;
use super::tile::{TileType, FloorType, WallOrientation, WallType, DEFAULT_WALL_TYPE};

const NOISE_SCALE: f32 = 0.2;

/// Builds maps.
pub struct MapBuilder {
    seed: usize,
    current_offset: [i32; 2],
}

impl MapBuilder {
    /// Creates and returns a new instance of the MapBuilder struct.
    pub fn new() -> MapBuilder {
        MapBuilder {
            seed: rand::random::<usize>(),
            current_offset: [0; 2],
        }
    }

    /// Generates a new map.
    pub fn create(&mut self) -> Map {
        self.create_offset([0, 0])
    }

    /// Creates a new map at the specified world offset.
    pub fn create_offset(&mut self, offset: [i32; 2]) -> Map {
        // increment the current offset by the values in the one provided
        self.current_offset = [
            self.current_offset[0] + offset[0],
            self.current_offset[1] + offset[1],
        ];

        let (offset_x, offset_y) = (self.current_offset[0] as f32, self.current_offset[1] as f32);

        let timer = SystemTime::now();
        let mut map = Map::new();

        let (width, height) = (map.width() as f32, map.height() as f32);

        // generate dungeon layout
        let dungeon_noise = Worley::<f32>::new().set_seed(self.seed);
        dungeon_noise.set_range_function(RangeFunction::Chebyshev);
        map.mut_parallel(move |_, t| {
            let (x, y) = (t.position[0] as f32, t.position[1] as f32);
            let noise_value = dungeon_noise.get(
                [
                    (((width - 1.0) * offset_x) + x) * NOISE_SCALE,
                    (((height - 1.0) * offset_y) + y) * NOISE_SCALE,
                    1.0,
                ],
            );
            if noise_value < 0.001 {
                t.tile_type = TileType::Wall(WallOrientation::Top, WallType::Stone);
            } else {
                t.tile_type = TileType::Floor(FloorType::Dirt);
            }
        });

        // fix wall orientations
        map.mut_parallel(move |m, t| {
            let mut should_convert_face = false;
            let mut converted_wall_type = DEFAULT_WALL_TYPE;
            if let TileType::Wall(ref orientation, ref wall_type) = t.tile_type {
                if let WallOrientation::Top = *orientation {
                    let (x, y) = (t.position[0], t.position[1]);
                    if let Some(tile_below) = m.get_at([x, y + 1]) {
                        if let TileType::Floor(_) = tile_below.tile_type {
                            should_convert_face = true;
                            converted_wall_type = wall_type.clone();
                        }
                    }
                }
            }
            if should_convert_face {
                t.tile_type = TileType::Wall(WallOrientation::Face, converted_wall_type);
            }
        });

        if let Ok(elapsed) = timer.elapsed() {
            info!(
                "Map was generated in {:?} ms",
                elapsed.subsec_nanos() / 1_000_000
            );
        }
        map
    }
}
