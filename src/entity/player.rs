use crate::entity::bubble::Bubble;
use crate::level::Level;
use glam::{vec2, Vec2};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub const GRAVITY: Vec2 = vec2(0.0, -5.0);

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,

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
            PhysicalKey::Code(KeyCode::ShiftLeft | KeyCode::ShiftRight) => {
                self.bubble_spawn_pressed = pressed;
            }
            _ => (),
        }
    }

    pub fn update(&mut self, level: &Level) -> Option<Bubble> {
        if self.left_pressed {
            self.vel.x = -5.5;
        } else if self.right_pressed {
            self.vel.x = 5.5;
        } else {
            self.vel.x *= 0.8;
        }

        if self.jump_pressed && !self.old_jump_pressed && self.on_ground {
            self.vel.y = 40.0;
        } else {
            self.vel.y *= 0.8;
        }

        let bubble = if self.bubble_spawn_pressed && !self.old_bubble_spawn_pressed {
            Some(Bubble {
                dead: false,
                pos: self.pos,
                vel: if self.pointed_right {
                    vec2(10.0, 0.0)
                } else {
                    vec2(-10.0, 0.0)
                },
            })
        } else {
            None
        };

        let new_pos = self.pos + (self.vel + GRAVITY);
        if level.is_hit(new_pos.as_uvec2()) {
            self.vel = Vec2::ZERO;
            self.pos.x = new_pos.x; // horribly broken
            self.on_ground = true; // not necessarily true
        } else {
            self.pos = new_pos;
            self.on_ground = false;
        }

        self.old_jump_pressed = self.jump_pressed;
        self.old_bubble_spawn_pressed = self.bubble_spawn_pressed;
        bubble
    }
}
