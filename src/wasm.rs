#![cfg(target_arch = "wasm32")]

use crate::main_loop::run;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowBuilderExtWebSys;

// this wasm may be totally derp and not even compile, will fix later

#[wasm_bindgen(start)]
pub fn wasm_start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let event_loop = EventLoop::new().unwrap();
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();
    console_log::init().unwrap();
    wasm_bindgen_futures::spawn_local(async { run(event_loop, window).await.unwrap() });
}
