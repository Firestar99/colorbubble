use colorbubble::main_loop::run;
use winit::event_loop::EventLoop;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let builder = winit::window::WindowBuilder::new();
    let window = builder.build(&event_loop)?;
    env_logger::init();
    pollster::block_on(run(event_loop, window))
}
