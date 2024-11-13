use wasm_bindgen::{ JsValue, JsError, prelude::* };

mod gpx_compressor;

use gpx_compressor::gpx::Gpx;

use serde_xml_rs::de;

pub trait WasmDescribe {
    fn describe();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let str = format!("Hello, {}!", name);
    unsafe {
        alert(&str);
    }
}

#[wasm_bindgen]
pub fn reduce_and_compress_gpx(gpx_str: &str) {
    let gpx: Gpx = match
        de::from_str(gpx_str).map_err(|e| JsError::new(&format!("Failed to parse GPX: {}", e)))
    {
        Ok(gpx) => gpx,
        Err(e) => {
            return;
        }
    };

    let reduced_gpx = gpx_compressor::reduce_gpx_size(&gpx);
    let compressed_gpx = match gpx_compressor::compress_gpx_to_buffer(&reduced_gpx) {
        Ok(buff) => buff,
        Err(e) => {
            return;
        }
    };
}
