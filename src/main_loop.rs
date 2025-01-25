use std::time::Duration;

use crate::delta_time::DeltaTimer;
use crate::level::Level;
use crate::rendering::game_renderer::{GameRenderer, RenderConfig};
use crate::rendering::quad_texture::QuadTexture;
use anyhow::Context;
use glam::{vec2, Vec2, Vec3};
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const TIMESTEP: Duration = Duration::from_millis(10);
const GRAVITY: Vec2 = vec2(0.0, -5.0);

pub struct Player {
    pub pos: Vec2,
    vel: Vec2,
    on_ground: bool,
}

impl Player {
    fn update(&mut self, level: &Level) {
        let new_pos = self.pos + (self.vel + GRAVITY);
        if level.is_hit(new_pos.as_uvec2()) {
            self.vel = Vec2::ZERO;
            self.pos.x = new_pos.x; // horribly broken
            self.on_ground = true; // not necessarily true
        } else {
            self.pos = new_pos;
            self.on_ground = false;
        }
    }
}

pub struct Bubble {
    pub pos: Vec2,
    vel: Vec2,
    dead: bool,
}

impl Bubble {
    fn update(&mut self, level: &Level, particles: &mut Vec<Particle>) -> bool {
        if self.dead {
            return self.dead;
        }

        let new_pos = self.pos + self.vel;
        if level.is_hit(new_pos.as_uvec2()) {
            self.pop(particles);
        } else {
            self.pos = new_pos;
        }

        self.dead
    }

    fn pop(&mut self, particles: &mut Vec<Particle>) {
        self.dead = true;
        for i in 0..10 {
            particles.push(Particle {
                age: 0,
                pos: self.pos,
                vel: Vec2::from_angle(i as f32) * 5.0,
            });
        }
    }
}

pub struct Particle {
    pub pos: Vec2,
    vel: Vec2,
    age: usize,
}

impl Particle {
    // returns true if collided
    fn update(&mut self, level: &Level) -> bool {
        self.age += 1;
        self.pos += self.vel;
        level.is_hit(self.pos.as_uvec2()) || self.age > 500
    }
}

pub struct ParticleRenderData<'a> {
    pub pos: Vec2,
    pub img: &'a QuadTexture,
}

pub struct DespawnedParticle {
    pub pos: Vec2,
    pub color: Vec3, //??
}

pub async fn run(event_loop: EventLoop<()>, window: Window) -> anyhow::Result<()> {
    let levels = Level::load_file_tree()?;
    let current_level_idx = 1;
    let level = &levels[current_level_idx];

    let mut player = Player {
        pos: level.entry_point.as_vec2(),
        vel: vec2(0.0, -1.0),
        on_ground: false,
    };

    dbg!(&level.entry_point);

    let mut bubble: Option<Bubble> = None;
    let mut particles: Vec<Particle> = vec![];

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());

    let surface = instance.create_surface(&window)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .context("Failed to find an appropriate adapter")?;

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .context("Failed to create device")?;

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let mut renderer = GameRenderer::new(&RenderConfig {
        device: device.clone(),
        queue,
        swapchain_format: surface.get_capabilities(&adapter).formats[0],
    });
    renderer.level.load_level(level.clone());
    let bubble_texture = renderer.particle.load_texture("assets/bubble.png");
    let particle_texture = renderer.particle.load_texture("assets/Splash.png");

    let mut delta_timer = DeltaTimer::default();
    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut old_jump_pressed = false;
    let mut jump_pressed = false;
    let mut old_bubble_spawn_pressed = false;
    let mut bubble_spawn_pressed = false;
    let mut time_sum = Duration::ZERO;
    let mut pointed_right = true; // false = pointed left
    event_loop.run(|event, target| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        left_pressed = pressed;
                        if pressed {
                            pointed_right = false;
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        right_pressed = pressed;
                        if pressed {
                            pointed_right = true;
                        }
                    }
                    PhysicalKey::Code(KeyCode::Space) => {
                        jump_pressed = pressed;
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft | KeyCode::ShiftRight) => {
                        bubble_spawn_pressed = pressed;
                    }
                    PhysicalKey::Code(KeyCode::Escape) => {
                        target.exit();
                    }
                    _ => (),
                }
            }
            WindowEvent::Resized(new_size) => {
                // Reconfigure the surface with the new size
                config.width = new_size.width.max(1);
                config.height = new_size.height.max(1);
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let dt = delta_timer.next();
                time_sum += Duration::from_secs_f32(dt.delta_time);

                let mut despawned_particles = Vec::new();

                // UPDATE CODE
                while let Some(new) = time_sum.checked_sub(TIMESTEP) {
                    time_sum = new;

                    if left_pressed {
                        player.vel.x = -5.5;
                    } else if right_pressed {
                        player.vel.x = 5.5;
                    } else {
                        player.vel.x *= 0.8;
                    }

                    if jump_pressed && !old_jump_pressed && player.on_ground {
                        player.vel.y = 40.0;
                    } else {
                        player.vel.y *= 0.8;
                    }

                    if bubble_spawn_pressed && !old_bubble_spawn_pressed {
                        if let Some(bubble) = &mut bubble {
                            bubble.pop(&mut particles);
                        }

                        bubble = Some(Bubble {
                            dead: false,
                            pos: player.pos,
                            vel: if pointed_right {
                                vec2(10.0, 0.0)
                            } else {
                                vec2(-10.0, 0.0)
                            },
                        });
                    }

                    player.update(level);
                    if let Some(bubble_val) = &mut bubble {
                        if bubble_val.update(level, &mut particles) {
                            bubble = None;
                        }
                    }

                    let mut remove = Vec::new();
                    for (i, particle) in particles.iter_mut().enumerate() {
                        if particle.update(level) {
                            remove.push(i);
                        }
                    }

                    for i in remove.into_iter().rev() {
                        let particle = particles.remove(i);
                        despawned_particles.push(DespawnedParticle {
                            pos: particle.pos,
                            color: Vec3::ONE, //todo??
                        });
                    }

                    old_jump_pressed = jump_pressed;
                    old_bubble_spawn_pressed = bubble_spawn_pressed;
                }

                // UPDATE CODE

                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                renderer.draw(
                    &player,
                    bubble
                        .iter()
                        .map(|bubble| ParticleRenderData {
                            pos: bubble.pos,
                            img: &bubble_texture,
                        })
                        .chain(particles.iter().map(|particle| ParticleRenderData {
                            pos: particle.pos,
                            img: &particle_texture,
                        })),
                    view,
                    despawned_particles,
                );
                frame.present();
                window.request_redraw();
            }
            WindowEvent::CloseRequested => target.exit(),
            _ => {}
        },
        _ => {}
    })?;
    Ok(())
}
