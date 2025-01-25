use std::fs;

use glam::UVec2;
use image::{ImageReader, Rgb, RgbImage};

const ENTRY_POINT: Rgb<u8> = Rgb([0, 99, 0]);
const COLLISION: Rgb<u8> = Rgb([255, 255, 255]);
// const COLLISION: Rgb<u8> = Rgb([0, 0, 255]);

#[derive(Default)]
pub struct Level {
    pub entry_point: UVec2,
    pub collision_map: RgbImage,
}

impl Level {
    pub fn load_from_disk() -> Vec<Level> {
        let mut a = Vec::new();
        let mut levl_prths: Vec<_> = fs::read_dir("levels")
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        levl_prths.sort();

        for level_file in levl_prths {
            let mut level = Level::default();
            let img = ImageReader::open(&level_file)
                .unwrap()
                .decode()
                .unwrap()
                .into_rgb8();
            let mut collision_map = img.clone();
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let pixel = img.get_pixel(x, y);

                    let flipped_pos = UVec2::new(x, img.height() - 1 - y);
                    let mut collision = false;
                    if pixel == &ENTRY_POINT {
                        level.entry_point = flipped_pos;
                    } else if pixel == &COLLISION {
                        collision = true;
                    }

                    collision_map.put_pixel(
                        flipped_pos.x,
                        flipped_pos.y,
                        if collision {
                            COLLISION
                        } else {
                            Rgb(Default::default())
                        },
                    );
                }
            }

            level.collision_map = collision_map;
            a.push(level);
            break;
        }

        a
    }

    pub fn is_hit(&self, pos: UVec2) -> bool {
        self.collision_map.get_pixel_checked(pos.x, pos.y) == Some(&COLLISION)
    }

    pub fn extent(&self) -> UVec2 {
        UVec2::new(self.collision_map.width(), self.collision_map.height())
    }
}
