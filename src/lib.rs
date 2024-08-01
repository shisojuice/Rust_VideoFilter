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

            (r,g,b)=closest_color(r,g,b);

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

fn closest_color(r: u32, g: u32, b: u32) -> (u32, u32, u32) {
    let colors = [
        (0, 0, 0),          // 黒
        (32, 32, 32),     // 濃いグレー
        (64, 64, 64),     // グレー
        (96, 96, 96),     // 薄いグレー
        (128, 128, 128), // もっと薄いグレー
        (160, 160, 160), // もっともっと薄いグレー
        (192, 192, 192), // もっともっともっと薄いグレー
        (225, 225, 225), // 白
        (128, 0, 0),      // 暗い赤
        (255, 0, 0),      // 赤
        (168, 42, 42),     // 茶色
        (255, 160, 122), // ピンク
        (0, 128, 0),      // 暗い緑
        (0, 255, 0),      // 緑
        (128, 128, 0),     // 黄緑
        (255, 255, 0),     // 黄
    ];
    *colors
        .iter()
        .min_by_key(|&&(cr, cg, cb)| color_distance(r, g, b, cr, cg, cb))
        .unwrap()
}

fn color_distance(r1: u32, g1: u32, b1: u32, r2: u32, g2: u32, b2: u32) -> u32 {
    let r_diff = r1 as i32 - r2 as i32;
    let g_diff = g1 as i32 - g2 as i32;
    let b_diff = b1 as i32 - b2 as i32;
    // 3次元空間上の距離を計算
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}
