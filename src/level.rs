use bevy_math::{Rect, URect};
use glam::{uvec2, UVec2};
use image::{GrayImage, ImageReader, Luma, Rgba, RgbaImage};
use std::fs;
use std::path::Path;
use std::sync::Arc;

const ENTRY_POINT: Rgba<u8> = Rgba([0, 99, 0, 255]);
const COLLISION: Rgba<u8> = Rgba([255, 255, 255, 255]);
// const COLLISION: Rgba<u8> = Rgba([0, 0, 255, 255]);

const COLLISION_LUMA: Luma<u8> = Luma([255]);

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub size: UVec2,
    pub image: RgbaImage,
    pub collision_map: GrayImage,
    pub entry_point: UVec2,
}

impl Level {
    pub fn load_from_file(path: &Path) -> anyhow::Result<Arc<Level>> {
        let image = ImageReader::open(path)?.decode()?.flipv().into_rgba8();

        let mut collision_map = GrayImage::new(image.width(), image.height());
        let mut entry_point = UVec2::ZERO;

        for y in 0..image.height() {
            for x in 0..image.width() {
                let pos = UVec2::new(x, y);
                let pixel = image.get_pixel(x, y);
                if pixel == &ENTRY_POINT {
                    entry_point = pos;
                } else if pixel == &COLLISION {
                    collision_map.put_pixel(pos.x, pos.y, COLLISION_LUMA);
                }
            }
        }

        Ok(Arc::new(Self {
            size: uvec2(image.width(), image.height()),
            image,
            entry_point,
            collision_map,
        }))
    }

    pub fn load_file_tree() -> anyhow::Result<Vec<Arc<Level>>> {
        let mut levl_prths: Vec<_> = fs::read_dir("levels")?.map(|e| e.unwrap().path()).collect();
        levl_prths.sort();
        levl_prths
            .iter()
            .map(|a| Self::load_from_file(a))
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn collision_rect(&self, rect: Rect) -> bool {
        self.collision_rectu(URect::from_corners(
            rect.min.as_uvec2(),
            rect.max.as_uvec2() + UVec2::new(1, 1),
        ))
    }

    pub fn collision_rectu(&self, rect: URect) -> bool {
        for y in 0..rect.height() {
            for x in 0..rect.width() {
                if *self.collision_map.get_pixel(x, y) == COLLISION_LUMA {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_hit(&self, pos: UVec2) -> bool {
        self.collision_map.get_pixel_checked(pos.x, pos.y) == Some(&COLLISION_LUMA)
    }

    pub fn extent(&self) -> UVec2 {
        UVec2::new(self.collision_map.width(), self.collision_map.height())
    }
}
