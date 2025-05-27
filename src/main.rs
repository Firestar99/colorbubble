use colorbubble::main_loop::run;
use winit::event_loop::EventLoop;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let builder = winit::window::WindowAttributes::default();
    #[expect(deprecated)]
    let window = event_loop.create_window(builder)?;
    pollster::block_on(run(event_loop, window))
}
