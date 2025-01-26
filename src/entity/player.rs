use crate::entity::bubble::Bubble;
use crate::entity::splash::Splash;
use crate::hsv2rgb::hsv2rgb;
use crate::level::Level;
use glam::{vec2, Vec2, Vec3, Vec4};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

const GRAVITY: Vec2 = vec2(0.0, -1.1);
const SPEED_X: f32 = 5.5;
const JUMP_Y: f32 = 18.0;
const DAMP_X: f32 = 0.8;
const DAMP_Y: f32 = 1.;
const BUBBLE_SPAWN_DISTANCE: Vec2 = vec2(10., 0.);
const HSV_HUE_SPEED: f32 = 0.01;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub hsv_hue: f32,
    pub hidden: bool,

    on_ground: bool,
    left_pressed: bool,
    right_pressed: bool,
    old_jump_pressed: bool,
    jump_pressed: bool,
    old_bubble_spawn_pressed: bool,
    bubble_spawn_pressed: bool,
    // false = pointed left
    pointed_right: bool,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(0.0, -1.0),
            hsv_hue: 0.,
            hidden: false,
            on_ground: false,
            left_pressed: false,
            right_pressed: false,
            old_jump_pressed: false,
            jump_pressed: false,
            old_bubble_spawn_pressed: false,
            bubble_spawn_pressed: false,
            pointed_right: false,
        }
    }

    pub fn color(&self) -> Vec4 {
        Vec4::from((hsv2rgb(Vec3::new(self.hsv_hue, 1., 1.)), 1.))
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        let pressed = event.state == ElementState::Pressed;
        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyA) => {
                self.left_pressed = pressed;
                if pressed {
                    self.pointed_right = false;
                }
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                self.right_pressed = pressed;
                if pressed {
                    self.pointed_right = true;
                }
            }
            PhysicalKey::Code(KeyCode::Space) => {
                self.jump_pressed = pressed;
            }
            PhysicalKey::Code(
                KeyCode::ShiftLeft | KeyCode::ShiftRight | KeyCode::KeyE | KeyCode::KeyQ,
            ) => {
                self.bubble_spawn_pressed = pressed;
            }
            _ => (),
        }
    }

    pub fn update(&mut self, level: &Level, particles: &mut Vec<Splash>) -> Option<Bubble> {
        dbg!(self.pos);
        self.hsv_hue = (self.hsv_hue + HSV_HUE_SPEED) % 1.;

        if self.left_pressed {
            self.vel.x = -SPEED_X;
        } else if self.right_pressed {
            self.vel.x = SPEED_X;
        } else {
            self.vel.x *= DAMP_X;
        }

        if self.jump_pressed && !self.old_jump_pressed && self.on_ground {
            self.vel.y = JUMP_Y;
        } else {
            self.vel.y *= DAMP_Y;
        }

        let bubble = if self.bubble_spawn_pressed && !self.old_bubble_spawn_pressed {
            Some(Bubble {
                pos: self.pos,
                vel: if self.pointed_right {
                    vec2(1.0, 0.0)
                } else {
                    vec2(-1.0, 0.0)
                } * BUBBLE_SPAWN_DISTANCE,
                color: self.color(),
                ..Default::default()
            })
        } else {
            None
        };

        self.vel += GRAVITY;
        let new_pos = self.pos + self.vel;
        if level.is_hit(new_pos.as_ivec2()) {
            self.vel = Vec2::ZERO;
            self.pos.x = new_pos.x; // horribly broken
            self.on_ground = true; // not necessarily true
        } else {
            self.pos = new_pos;
            self.on_ground = false;
        }

        if level.is_death(new_pos.as_ivec2()) {
            Splash::spawn_many(particles, self.pos, 2., self.color(), 25);
            self.pos = level.entry_point.as_vec2();
            self.vel = Vec2::ZERO;
        }

        self.old_jump_pressed = self.jump_pressed;
        self.old_bubble_spawn_pressed = self.bubble_spawn_pressed;
        bubble
    }
}
