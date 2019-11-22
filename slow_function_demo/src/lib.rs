use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn slow_func() -> f32{
    let mut result: f32 = 0.0;
    let upperbound: i32 = 10;
	for i in (0..upperbound.pow(7)).rev() {
        result = result + ((i as f32).tan() * (i as f32).atan());
    }
    result
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 { x + y }