use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();

  web_sys::console::log_1(&format!("{}", "Yo!").into());
}
