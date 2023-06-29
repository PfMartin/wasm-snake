use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
pub fn greet_admin() {
    alert("Hello Admin")
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
