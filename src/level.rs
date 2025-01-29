use bevy_math::{Rect, URect};
use glam::{uvec2, IVec2, UVec2};
use image::{GrayImage, ImageFormat, ImageReader, Luma, Rgba, RgbaImage};
use std::io::Cursor;
use std::sync::Arc;

const ENTRY_POINT: Rgba<u8> = Rgba([0, 99, 0, 255]);
const PORTAL: Rgba<u8> = Rgba([0, 98, 0, 255]);
const DEATH: Rgba<u8> = Rgba([0, 0, 100, 255]);
const COLLISION: Rgba<u8> = Rgba([255, 255, 255, 255]);
// const COLLISION: Rgba<u8> = Rgba([0, 0, 255, 255]);

const COLLISION_LUMA: Luma<u8> = Luma([255]);
const DEATH_LUMA: Luma<u8> = Luma([1]);

#[derive(Debug, Clone, Default)]
pub struct Level {
    pub size: UVec2,
    pub image: RgbaImage,
    pub collision_map: GrayImage,
    pub entry_point: UVec2,
    pub portal: UVec2,
}

impl Level {
    pub fn load_from_binary(bytes: &[u8]) -> anyhow::Result<Arc<Level>> {
        let image = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Png)
            .decode()?
            .flipv()
            .into_rgba8();

        let mut collision_map = GrayImage::new(image.width(), image.height());
        let mut entry_point = UVec2::ZERO;
        let mut portal = UVec2::ZERO;

        for y in 0..image.height() {
            for x in 0..image.width() {
                let pos = UVec2::new(x, y);
                let mut pixel = *image.get_pixel(x, y);
                pixel.0[3] = 255;
                match pixel {
                    ENTRY_POINT => entry_point = pos,
                    PORTAL => portal = pos,
                    COLLISION => collision_map.put_pixel(pos.x, pos.y, COLLISION_LUMA),
                    DEATH => collision_map.put_pixel(pos.x, pos.y, DEATH_LUMA),
                    _ => {}
                }
            }
        }

        Ok(Arc::new(Self {
            size: uvec2(image.width(), image.height()),
            image,
            collision_map,
            entry_point,
            portal,
        }))
    }

    pub fn load_file_tree() -> anyhow::Result<Vec<Arc<Level>>> {
        [
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl01.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl02.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl03.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl04.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl05.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl06.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl07.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl08.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl09.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl10.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl11.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl12.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl13.png")).as_slice(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/levels/Lvl14.png")).as_slice(),
        ]
        .iter()
        .map(|a| Self::load_from_binary(a))
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

    pub fn is_hit(&self, pos: IVec2) -> bool {
        if pos.x < 0 && pos.y < 0 {
            false
        } else {
            self.collision_map
                .get_pixel_checked(pos.x as u32, pos.y as u32)
                == Some(&COLLISION_LUMA)
        }
    }

    #[inline(never)]
    pub fn is_death(&self, pos: IVec2) -> bool {
        if pos.x < 0 && pos.y < 0 {
            true
        } else {
            self.collision_map
                .get_pixel_checked(pos.x as u32, pos.y as u32)
                .map_or(true, |e| *e == DEATH_LUMA)
        }
    }

    pub fn extent(&self) -> UVec2 {
        UVec2::new(self.collision_map.width(), self.collision_map.height())
    }
}
