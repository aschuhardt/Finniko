use std::fs::File;
use std::path::PathBuf;
use opengl_graphics::{GlGraphics, Texture};
use graphics::image::Image;
use graphics::math::Matrix2d;
use graphics::DrawState;
use image::{self, DynamicImage};
use find_folder::Search;
use texture::{Filter, TextureSettings};
use bmfont::{OrdinateOrientation, BMFont};

const FONTS_DIR: &'static str = "assets/fonts";
const FONT_32: &'static str = "32.fnt";
const FONT_24: &'static str = "24.fnt";
const FONT_18: &'static str = "18.fnt";

/// Represents available predefined font sizes
pub enum FontSize {
    Size32,
    Size24,
    Size18,
}

/// Used for drawing text to the screen.  Necessary because
/// piston's truetype text rendering is slow as hell.
pub struct TextRenderer {
    glyphs_32: Texture,
    font_32: BMFont,
    glyphs_24: Texture,
    font_24: BMFont,
    glyphs_18: Texture,
    font_18: BMFont,
    draw_state: DrawState,
}

impl TextRenderer {
    /// Creates and returns a new instance of the TextRenderer struct.
    pub fn new() -> TextRenderer {
        let assets = Search::ParentsThenKids(3, 3).for_folder(FONTS_DIR).unwrap();
        let font_32 = BMFont::new(
            File::open(assets.join(FONT_32)).expect(
                format!(
                    "Unable to open {:?}",
                    FONT_32
                ).as_str(),
            ),
            OrdinateOrientation::TopToBottom,
        ).expect("Unable to load size-32 font");
        let font_24 = BMFont::new(
            File::open(assets.join(FONT_24)).expect(
                format!(
                    "Unable to open {:?}",
                    FONT_24
                ).as_str(),
            ),
            OrdinateOrientation::TopToBottom,
        ).expect("Unable to load size-24 font");
        let font_18 = BMFont::new(
            File::open(assets.join(FONT_18)).expect(
                format!(
                    "Unable to open {:?}",
                    FONT_18
                ).as_str(),
            ),
            OrdinateOrientation::TopToBottom,
        ).expect("Unable to load size-18 font");
        TextRenderer {
            glyphs_32: TextRenderer::load_texture(assets.join(&font_32.pages()[0])),
            font_32: font_32,
            glyphs_24: TextRenderer::load_texture(assets.join(&font_24.pages()[0])),
            font_24: font_24,
            glyphs_18: TextRenderer::load_texture(assets.join(&font_18.pages()[0])),
            font_18: font_18,
            draw_state: DrawState::default(),
        }
    }

    /// Draws text at the specified position on the screen.
    pub fn draw_at(
        &self,
        position: [f64; 2],
        text: String,
        transform: Matrix2d,
        size: FontSize,
        color: [f32; 4],
        g: &mut GlGraphics,
    ) {
        let (font, glyphs) = match size {
            FontSize::Size32 => (&self.font_32, &self.glyphs_32),
            FontSize::Size24 => (&self.font_24, &self.glyphs_24),
            FontSize::Size18 => (&self.font_18, &self.glyphs_18),
        };

        if let Ok(map) = font.parse(text.as_str()) {
            for char_info in &map {
                Image::new_color(color)
                    .src_rect(
                        [
                            char_info.page_rect.x as f64,
                            char_info.page_rect.y as f64,
                            char_info.page_rect.width as f64,
                            char_info.page_rect.height as f64,
                        ],
                    )
                    .rect(
                        [
                            position[0] + char_info.screen_rect.x as f64,
                            position[1] + char_info.screen_rect.y as f64,
                            char_info.screen_rect.width as f64,
                            char_info.screen_rect.height as f64,
                        ],
                    )
                    .draw(glyphs, &self.draw_state, transform, g);
            }
        }
    }

    /// Returns the line-height corresponding to a given font size
    pub fn line_height(&self, size: FontSize) -> u32 {
        match size {
            FontSize::Size32 => self.font_32.line_height(),
            FontSize::Size24 => self.font_24.line_height(),
            FontSize::Size18 => self.font_18.line_height(),
        }
    }

    fn load_texture(path: PathBuf) -> Texture {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => panic!("Could not load texture: {:?}", e),
        };

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba(),
        };

        Texture::from_image(&img, &TextureSettings::new().filter(Filter::Nearest))
    }
}
