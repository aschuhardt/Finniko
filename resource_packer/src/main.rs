extern crate image;
extern crate serde;
extern crate serde_json;
extern crate texture_packer;
extern crate texture_coords;
extern crate walkdir;

use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::BufWriter;

use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::importer::ImageImporter;
use texture_packer::exporter::ImageExporter;
use texture_coords::CoordinateMap;

fn main() {
    let config = TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 1,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    println!("{:?}", env::args().nth(1).unwrap());

    for entry in walkdir::WalkDir::new(env::args().nth(1).unwrap()) {
        if let Ok(path) = entry {
            let path = path.path();
            if let Some(ext) = path.extension() {
                if ext == "png" {
                    println!("{:?}", path.clone());
                    let filename = String::from(path.file_stem().unwrap().to_str().unwrap());
                    let texture = ImageImporter::import_from_file(&path).unwrap();
                    packer.pack_own(filename, texture);
                }
            }
        }
    }

    let mut coords = CoordinateMap { map: HashMap::<String, [f64; 4]>::new() };

    for (name, frame) in packer.get_frames() {
        let frame_rect = frame.frame;
        let rect = [
            frame_rect.x as f64,
            frame_rect.y as f64,
            frame_rect.w as f64,
            frame_rect.h as f64,
        ];
        coords.map.insert(name.clone(), rect);
    }

    let file = File::create("textures_map.json").unwrap();
    let _ = serde_json::to_writer_pretty(BufWriter::new(file), &coords)
        .unwrap_or_else(|_| panic!("Unable to write texture coordinate file."));

    let exporter = ImageExporter::export(&packer).unwrap();
    let mut file = File::create("textures.png").unwrap();
    exporter.save(&mut file, image::PNG).unwrap();
}
