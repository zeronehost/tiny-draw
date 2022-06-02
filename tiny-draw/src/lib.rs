mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  
}

mod core;
use crate::core::canvas::Canvas;
#[wasm_bindgen]
pub struct TinyDraw {
  canvas: core::canvas::Canvas
}

#[wasm_bindgen]
impl TinyDraw {
  #[wasm_bindgen(constructor)]
  pub fn new(id: &str, width: u32, height: u32) -> Result<TinyDraw, JsError> {
    let canvas = match Canvas::new(id, width, height) {
      Ok(x) => x,
      Err(_) => return Err(JsError::new("error"))
    };
    Ok(Self { canvas })
  }
}

// https://github.com/lgq627628/2020/blob/master/%E5%9B%BE%E5%BD%A2%E5%AD%A6/fabric/src
