use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn slow_func(){
    let mut result: f32 = 0.0;
    let upperbound: i32 = 10;
	for i in (0..upperbound.pow(7)).rev() {
        result = result + ((i as f32).tan() * (i as f32).atan());
    }
}