#![cfg(target_arch = "wasm32")]

use crate::main_loop::run;
use wasm_bindgen::JsCast;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowBuilderExtWebSys;

// this wasm may be totally derp and not even compile, will fix later

#[wasm_bindgen(start)]
pub fn wasm_start() {
    wasm_run().unwrap()
}

pub fn wasm_run() -> anyhow::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let event_loop = EventLoop::new()?;
    let canvas = web_sys::window()?
        .document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let window = winit::window::WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)?;
    console_log::init()?;
    wasm_bindgen_futures::spawn_local(run(event_loop, window));
    Ok(())
}
