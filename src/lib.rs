use std::collections::HashMap;
use std::io::{Cursor};
use image::{DynamicImage, ImageBuffer, Rgba, ImageFormat, GenericImage};
use image::imageops::FilterType;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::{Blob,BlobPropertyBag, Url};
use web_sys::js_sys::{Array, Uint8Array};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn dot_art_generate(input_size: String,image_data: &[u8]) {
    let mut img = resize_image(image::load_from_memory(image_data).unwrap());

    // セルのサイズ
    let dot_size:u32 = input_size.parse().unwrap_or(8);
    let cell_size:u32 = 512/dot_size;
    let mut output_img = ImageBuffer::new(img.width(), img.height());
    // セルごとに処理
    for y in 0..img.height() / cell_size {
        for x in 0..img.width() / cell_size {
            // セルを切り出す
            let cell = img.sub_image(
                x * cell_size,
                y * cell_size,
                cell_size,
                cell_size,
            );

            // セルを解析
            let recognized_rgba = analyze_cell(cell.to_image());
            for b in 0..cell_size  {
                for a in 0..cell_size {
                    output_img.put_pixel(a+(x*cell_size) , b+(y*cell_size),recognized_rgba);
                }
            }
        }
    }
    img = DynamicImage::ImageRgba8(output_img);

    //DL
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let img_data = buffer.into_inner();
    let window = web_sys::window().unwrap();
    let uint8_array = Uint8Array::from(img_data.as_slice());
    let parts = Array::new();
    parts.push(&uint8_array);
    // Blobを作成
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts,BlobPropertyBag::new().type_("image/png")).unwrap();
    // BlobのURLを取得
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    // a要素を作成
    let link = window.document().unwrap().create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
    link.set_href(&url);
    link.set_download("dot_art.png");
    link.click();
    // URLを解放
    Url::revoke_object_url(&url).unwrap();
}
fn analyze_cell(cell: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Rgba<u8> {
    let mut counts = HashMap::new();
    // ピクセルごとにRGBAをキーとして出現回数をカウント
    for pixel in cell.pixels() {
        *counts.entry(pixel).or_insert(0) += 1;
    }
    let result_rgba =  counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(rgba, _)| rgba).unwrap();

    *result_rgba
}
// 画像のアスペクト比・操作しやすいサイズに設定・調整
fn resize_image(img: DynamicImage) -> DynamicImage {
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // 横長の画像の場合
        (512, (512.0 / aspect_ratio) as u32)
    } else {
        // 縦長の画像の場合
        ((512.0 * aspect_ratio) as u32, 512)
    };
    // リサイズ
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    // 512x512のキャンバスを作成
    let mut canvas = DynamicImage::new_rgba8(512, 512);
    // 白を背景色とする
    let bkg_image= ImageBuffer::from_pixel(512, 512, Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut canvas, &bkg_image, 0, 0);
    // リサイズ画像をキャンバスの中央に貼付
    let x = (512 - resized_img.width()) / 2;
    let y = (512 - resized_img.height()) / 2;
    image::imageops::overlay(&mut canvas, &resized_img, x.into(), y.into());
    canvas
}