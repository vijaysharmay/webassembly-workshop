use std::time::{Instant};
use wasm_bindgen::prelude::*;
mod sqlite;

#[wasm_bindgen]
pub fn run_slow_func() {
    let start = Instant::now();
    slow_func();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[wasm_bindgen]
pub fn slow_func(){
    let mut result: f32 = 0.0;
    let upperbound: i32 = 10;
	for i in (0..upperbound.pow(7)).rev() {
        result = result + ((i as f32).tan() * (i as f32).atan());
    }
}

// #[wasm_bindgen]
// pub fn run_sqlite(){
//     sqlite::create();
//     sqlite::insert();
// }