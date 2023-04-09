mod complex;
mod mandelbrot;
use crate::mandelbrot::Mandelbrot;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
pub fn generate_mandelbrot_set(
    iters: usize,
    size: usize,
    x_min: f64,
    x_max: f64,                 
    y_min: f64,
    y_max: f64,
    ctx: CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    let mut mandelbrot = Mandelbrot::new(iters, x_min, x_max, y_min, y_max, size);
    mandelbrot.calculate_mandelbort();
    let mandelbrot_color = mandelbrot.return_mandelbrot_color_values();
    let image = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mandelbrot_color),
        size as u32,
        size as u32,
    )?;
    
    ctx.put_image_data(&image, 0.0, 0.0)
}
