use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
extern crate photon_rs as photon;
use photon::{channels, monochrome};
use image;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, qrcode-wasm!");
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn get_qrcode(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    canvas2: HtmlCanvasElement,
    ctx2: CanvasRenderingContext2d,
) {
    let mut img: photon::PhotonImage = photon::open_image(canvas, ctx);
    //monochrome::grayscale(&mut img);
    let width: u32 = img.get_width();
    let height: u32 = img.get_height();
    let raw_pixels: Vec<u8> = img.get_raw_pixels();
    
    let img = match image::load_from_memory(&raw_pixels) {
        Ok(v) => v,
        Err(_e) => {
            log("[Error] No QR code detected in image");
            return;
        },
    };
    
    let img = img.into_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // if grids.len() != 1 {
    //     return format!("{}", "[Error] No QR code detected in image")
    // }
    // let mut img_new: photon::PhotonImage =
    //     photon::PhotonImage::new(raw_pixels, width, height);
    // channels::alter_channel(&mut img_new, 1, 50);
    // let raw_pixels2: Vec<u8> = img_new.get_raw_pixels();
    // photon::putImageData(canvas2, ctx2, img_new);
    let gray_raw_pixels = raw_pixels.len() / 4;
    let mut raw_grayscale_pixels = Vec::with_capacity(gray_raw_pixels as usize);
    let mut raw_black_pixels = Vec::with_capacity(gray_raw_pixels as usize);
    let mut raw_white_pixels = Vec::with_capacity(gray_raw_pixels as usize);
    for i in 0..raw_pixels.len() / 4 {
        let avg: u32 = (raw_pixels[i * 4] as u32 + raw_pixels[i * 4 + 1] as u32 + raw_pixels[i * 4 + 2] as u32) / 3;
        if avg < 200 {
            raw_black_pixels.push(0);
            raw_grayscale_pixels.push(0);
        }
        else{
            raw_white_pixels.push(255);
            raw_grayscale_pixels.push(255);
        }
    }
    // for y in 0..height {
    //     let mut total_rgb = 0;
    //     let mut count = 1;
    //     for x in 0..width * 4 {
    //         let mut pixel = raw_pixels[(x + y) as usize];
    //         //count of 4 is alpha which is ignored
    //         if count < 4 {
    //             total_rgb += pixel;
    //             count += 1;
    //         } else {
    //             count = 1;
    //             let average = total_rgb / 3;
    //             if average < 200 {
    //                 raw_grayscale_pixels.push(0);
    //                 raw_black_pixels.push(0);
    //             } else {
    //                 raw_grayscale_pixels.push(255);
    //                 raw_white_pixels.push(255);
    //             }
    //         }
    //     }
    // }
    let fn_mut = |x: usize, y: usize| -> u8 { raw_grayscale_pixels[x + y] };
    log(&(String::from("width: ") + width.to_string().as_ref()));
    log(&(String::from("height: ") + height.to_string().as_ref()));
    log(&(String::from("raw_grayscale_pixels.length: ") + raw_grayscale_pixels.len().to_string().as_ref()));
    log(&(String::from("raw_white_pixels.length: ") + raw_white_pixels.len().to_string().as_ref()));
    log(&(String::from("raw_black_pixels.length: ") + raw_black_pixels.len().to_string().as_ref()));
    log(&(String::from("raw_pixels.length: ") + raw_pixels.len().to_string().as_ref()));

    // let mut rqrr_img = rqrr::PreparedImage::prepare_from_greyscale(
    //     width as usize,
    //     height as usize,
    //     fn_mut,
    // );
    // let grids = rqrr_img.detect_grids();
    // assert_eq!(grids.len(), 1);
    // // Decode the grid
    // let result = grids[0].decode();
    // assert_eq!(result.is_ok(), true);
    // let mut result_return: String = String::new();
    // if let Ok((_, content)) = result {
    //     result_return = String::from(content);
    // }
    // log(&(String::from("result_return: ") + result_return.to_string().as_ref()));
    // //assert_eq!(meta.ecc_level, 0);
    // //assert_eq!(content, "https://github.com/WanzenBug/rqrr");
    // result_return
}

#[wasm_bindgen]
pub fn decode_qr(bytes: &[u8]) -> String {
    let img = match image::load_from_memory(&bytes) {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed when trying to load image"),
    };

    let img = img.to_luma8();

    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    if grids.len() != 1 {
        return format!("{}", "[Error] No QR code detected in image")
    }

    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed decoding the image"),
    };

    return format!("{}", content);
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
    photon::putImageData(canvas, ctx, new_image);
}

#[wasm_bindgen]
pub fn grayscale_pass(img: &mut photon::PhotonImage) {
    monochrome::grayscale(img);
}
