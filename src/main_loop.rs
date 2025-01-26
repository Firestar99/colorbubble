use crate::delta_time::DeltaTimer;
use crate::entity::game::Game;
use crate::level::Level;
use crate::rendering::game_renderer::{GameRenderer, RenderConfig};
use anyhow::Context;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub async fn run(event_loop: EventLoop<()>, window: Window) -> anyhow::Result<()> {
    let levels = Level::load_file_tree()?;
    let current_level_idx = 1;
    let level = &levels[current_level_idx];

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
    })?;
    renderer.level.load_level(level.clone());

    let mut game = Game::new(level.clone());

    let mut delta_timer = DeltaTimer::default();
    event_loop.run(|event, target| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::KeyboardInput { event, .. } => {
                game.player.handle_key_event(event);
                match event.physical_key {
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
                // UPDATE
                let dt = delta_timer.next();
                let despawned_particles = game.update(dt);

                // BUBBLE DRAW
                renderer.level.draw_color_splashes(&despawned_particles);

                // MAIN DRAW
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                renderer.draw(
                    &game.player,
                    game.player_bubble.as_slice(),
                    &game.particles,
                    view,
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
