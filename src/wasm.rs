#![cfg(target_arch = "wasm32")]

use crate::main_loop::run;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowExtWebSys;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("wasm-example")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
    console_log::init().unwrap();
    wasm_bindgen_futures::spawn_local(async { run(event_loop, window).await.unwrap() });
}
