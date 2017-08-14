use std::fs::File;
use std::path::PathBuf;
use std::io::{BufWriter, BufReader};
use opengl_graphics::{GlGraphics, Texture};
use graphics::types::Rectangle;
use graphics::image::Image;
use graphics::math::Matrix2d;
use graphics::DrawState;
use find_folder::Search;
use serde_json;
use image::{self, DynamicImage};
use texture_coords::CoordinateMap;
use texture::{Filter, TextureSettings};

const TEXTURE_DIR: &'static str = "assets/";
const TEXTURE_FILE: &'static str = "textures.png";
const TEXTURE_COORDS: &'static str = "textures_map.json";

/// A wrapper capable of mapping tile type enums into sub-images at
/// specific coordinates.  Also stores a copy of a base texture to use for
/// the sake of convenience, and provides methods for drawing onto the screen.
pub struct TextureMapper {
    texture: Texture,
    coords: CoordinateMap,
    draw_state: DrawState,
}

impl TextureMapper {
    /// Creates and returns a new instance of the TextureMapper struct.
    pub fn new() -> TextureMapper {
        TextureMapper {
            texture: TextureMapper::load_texture(),
            coords: TextureMapper::load_coords(),
            draw_state: DrawState::default(),
        }
    }

    /// Draws a texture designated by the provided `key` at the location
    /// and size specified by the provided `Rectangle`.
    pub fn draw_at<R: Into<Rectangle>>(
        &self,
        destination: R,
        key: &String,
        transform: Matrix2d,
        g: &mut GlGraphics,
    ) {
        if self.coords.map.contains_key(key) {
            Image::new()
                .src_rect(self.coords.map[key])
                .rect(destination)
                .draw(&self.texture, &self.draw_state, transform, g);
        } else {
            error!("No texture coordinate info found for key {:?}", key);
        }
    }

    fn load_texture() -> Texture {
        let assets = Search::ParentsThenKids(3, 3)
            .for_folder(TEXTURE_DIR)
            .unwrap();
        let path = assets.join(TEXTURE_FILE);

        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => panic!("Could not load texture: {:?}", e),
        };

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba(),
        };

        Texture::from_image(&img, &TextureSettings::new().filter(Filter::Nearest))
        // Texture::from_path(path).unwrap_or_else(|p| panic!("Unable to load texture at {:?}", p))
    }

    fn load_coords() -> CoordinateMap {
        let assets = Search::ParentsThenKids(3, 3)
            .for_folder(TEXTURE_DIR)
            .unwrap();
        let path = assets.join(TEXTURE_COORDS);
        info!(
            "Attempting to load texture coordinate map info from {:?}...",
            path
        );
        let file = match File::open(path.clone()) {
            Err(why) => {
                info!(
                    "Creating template texture coordinate map file at {:?}",
                    path
                );
                TextureMapper::save_template_coords(path);
                panic!("Unable to find texture coordinate file: {:?}", why)
            }
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| {
            panic!("Unable to parse texture coordinate info.")
        })
    }

    fn save_template_coords(path: PathBuf) {
        let file = File::create(path).expect("Unable to create template texture coordinates file.");
        let _ = serde_json::to_writer_pretty(
            BufWriter::new(file),
            &CoordinateMap {
                map: [(String::from("key_text"), [0.0, 0.0, 16.0, 16.0])]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ).unwrap_or_else(|_| {
            panic!("Unable to write template texture coordinate file.")
        });
    }
}
