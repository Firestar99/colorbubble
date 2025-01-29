#![cfg(target_arch = "wasm32")]

use crate::main_loop::run;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowBuilderExtWebSys;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_append(true)
        .build(&event_loop)
        .unwrap();
    let _ = window.request_inner_size(PhysicalSize::new(800, 600));
    console_log::init().unwrap();
    wasm_bindgen_futures::spawn_local(async { run(event_loop, window).await.unwrap() });
}
