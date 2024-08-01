use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pixel_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32,dot_size :u32) -> Vec<u8> {
    let width = canvas_width as usize;
    let height = canvas_height as usize;
    let dot_size = dot_size as usize;

    for y in (0..height).step_by(dot_size) {
        for x in (0..width).step_by(dot_size) {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for dy in 0..dot_size {
                for dx in 0..dot_size {
                    let i = ((y + dy) * width + (x + dx)) * 4; // RGBAなので4倍
                    if i + 3 < buffer.len() {
                        r += buffer[i] as u32;
                        g += buffer[i + 1] as u32;
                        b += buffer[i + 2] as u32;
                    }
                }
            }

            // ドット内のすべてのピクセルに平均色を設定
            r /= (dot_size * dot_size) as u32;
            g /= (dot_size * dot_size) as u32;
            b /= (dot_size * dot_size) as u32;

            for dy in 0..dot_size {
                for dx in 0..dot_size {
                    let i = ((y + dy) * width + (x + dx)) * 4;
                    if i + 3 < buffer.len() {
                        buffer[i] = r as u8;
                        buffer[i + 1] = g as u8;
                        buffer[i + 2] = b as u8;
                        buffer[i + 3] = 255; // アルファ値
                    }
                }
            }
        }
    }

    buffer
}
