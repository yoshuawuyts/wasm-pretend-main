use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(not(debug_assertions))]
    console_error_panic_hook::set_once();

    println!("Hello, wasm-pretend-main!");
}
