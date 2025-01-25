use std::fs;

use glam::UVec2;
use image::{ImageReader, Rgb, RgbImage};

const ENTRY_POINT: Rgb<u8> = Rgb([0, 99, 0]);
const COLLISION: Rgb<u8> = Rgb([0, 0, 255]);

#[derive(Default)]
struct Level {
    entry_point: UVec2,
    collision_map: RgbImage,
}

impl Level {
    fn load_from_disk() -> Vec<Level> {
        let mut a = Vec::new();
        for level_file in fs::read_dir("levels").unwrap() {
            let mut level = Level::default();
            let level_file = level_file.unwrap();
            let mut img = ImageReader::open(level_file.path())
                .unwrap()
                .decode()
                .unwrap()
                .into_rgb8();
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let pos = UVec2::new(x, y);
                    let pixel = img.get_pixel(x, y);
                    let mut collision = false;
                    if pixel == &ENTRY_POINT {
                        level.entry_point = pos;
                    } else if pixel == &COLLISION {
                        collision = true;
                    }

                    img.put_pixel(
                        x,
                        y,
                        if collision {
                            COLLISION
                        } else {
                            Rgb(Default::default())
                        },
                    );
                }
            }

            level.collision_map = img;
            a.push(level);
        }

        a
    }
}

pub fn doit() {
    println!("we're doing it ");
    let _lvl = Level::load_from_disk();
}
