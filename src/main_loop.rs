use std::time::Duration;

use crate::delta_time::DeltaTimer;
use crate::level::Level;
use crate::rendering::game_renderer::{GameRenderer, RenderConfig};
use anyhow::Context;
use glam::{vec2, Vec2};
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const TIMESTEP: Duration = Duration::from_millis(10);
const GRAVITY: Vec2 = vec2(0.0, -1.0);

pub struct Player {
    pub pos: Vec2,
    vel: Vec2,
}

impl Player {
    fn update(&mut self, level: &Level) {
        let new_pos = self.pos + (self.vel + GRAVITY);
        if level.is_hit(new_pos.as_uvec2()) {
            self.vel = Vec2::ZERO;
        } else {
            self.pos = new_pos;
        }
    }
}

pub async fn run(event_loop: EventLoop<()>, window: Window) -> anyhow::Result<()> {
    let levels = Level::load_file_tree()?;
    let current_level_idx = 0;
    let level = &levels[current_level_idx];

    let mut player = Player {
        pos: level.entry_point.as_vec2(),
        vel: vec2(0.0, -1.0),
    };

    dbg!(&level.entry_point);

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

    let renderer = GameRenderer::new(&RenderConfig {
        device: device.clone(),
        queue,
        swapchain_format: surface.get_capabilities(&adapter).formats[0],
    });

    let mut delta_timer = DeltaTimer::default();
    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut old_jump_pressed = false;
    let mut jump_pressed = false;
    let mut time_sum = Duration::ZERO;
    event_loop.run(|event, target| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyA) => {
                    left_pressed = event.state == ElementState::Pressed;
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    right_pressed = event.state == ElementState::Pressed;
                }
                PhysicalKey::Code(KeyCode::Space) => {
                    jump_pressed = event.state == ElementState::Pressed;
                }
                PhysicalKey::Code(KeyCode::Escape) => {
                    target.exit();
                }
                _ => (),
            },
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

                    if jump_pressed && !old_jump_pressed {
                        player.vel.y = 10.0;
                    } else {
                        player.vel.y *= 0.8;
                    }

                    player.update(level);
                    old_jump_pressed = jump_pressed;
                }

                // UPDATE CODE

                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                renderer.draw(&player, view);
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
