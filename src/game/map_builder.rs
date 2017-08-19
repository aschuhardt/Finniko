use rand;
use std::time::SystemTime;
use noise::{Seedable, Fbm, NoiseModule};
use super::map::Map;
use super::tile::{TileType, FloorType};

const NOISE_SCALE: f32 = 0.02;

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

        let stone_noise = Fbm::<f32>::new().set_seed(self.seed + 1);
        map.mut_parallel(move |t| {
            let (x, y) = (t.position[0] as f32, t.position[1] as f32);
            if stone_noise.get(
                [
                    (((width - 1.0) * offset_x) + x) * NOISE_SCALE,
                    (((height - 1.0) * offset_y) + y) * NOISE_SCALE,
                    1.0,
                ],
            ) > 0.1
            {
                t.tile_type = TileType::Floor(FloorType::Concrete);
            }
        });

        let grass_noise = Fbm::<f32>::new().set_seed(self.seed);
        map.mut_parallel(move |t| {
            let (x, y) = (t.position[0] as f32, t.position[1] as f32);
            if grass_noise.get(
                [
                    (((width - 1.0) * offset_x) + x) * NOISE_SCALE,
                    (((height - 1.0) * offset_y) + y) * NOISE_SCALE,
                    1.0,
                ],
            ) > 0.1
            {
                t.tile_type = TileType::Floor(FloorType::Grass);
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
