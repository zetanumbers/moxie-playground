use moxie_dom::prelude::*;
use wasm_bindgen::prelude::*;

mod app;
mod arrows;
mod checkbox;
mod points;
mod utils;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn boot(root: sys::Node) {
    console_error_panic_hook::set_once();
    moxie_dom::boot(root, app::main);
}
