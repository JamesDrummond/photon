mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use image;
extern crate photon_rs as photon;
use photon::{channels,monochrome};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, qrcode-wasm!");
}

#[wasm_bindgen]
#[no_mangle]
pub fn open_image_pass(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
) -> photon::PhotonImage {
    photon::open_image(canvas, ctx)
}

#[wasm_bindgen]
pub fn alter_channel_pass(img: &mut photon::PhotonImage, channel: usize, amt: i16) {
    channels::alter_channel(img, channel, amt);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn putImageData_pass(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    new_image: photon::PhotonImage,
) {
    photon::putImageData(canvas,ctx,new_image);
}


#[wasm_bindgen]
pub fn grayscale_pass(img: &mut photon::PhotonImage) {
    monochrome::grayscale(img);
}
