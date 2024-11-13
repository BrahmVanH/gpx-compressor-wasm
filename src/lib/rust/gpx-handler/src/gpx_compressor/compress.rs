use std::io::{ self, Write };
use zstd::stream::write::Encoder;
use super::gpx::SmlrGpx;
use bincode;
use wasm_bindgen::prelude::*;

pub fn compress_gpx_to_memory(gpx: &SmlrGpx) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let binary_data = bincode::serialize(gpx)?;

    // let json_bytes = serde_json::to_vec(gpx)?;
    let mut encoder = Encoder::new(Vec::new(), 19)?;
    encoder.write_all(&binary_data)?;
    Ok(encoder.finish()?)
}

pub fn compress_gpx_to_file(
    gpx: &SmlrGpx,
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let json_bytes = serde_json::to_vec(gpx)?;
    let file = std::fs::File::create(output_path)?;

    let mut encoder = Encoder::new(file, 0)?;
    encoder.write_all(&json_bytes)?;
    encoder.finish()?;
    Ok(())
}

#[wasm_bindgen]
pub fn compress_gpx_to_buffer(
    gpx: &SmlrGpx
) -> Result<std::io::Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
    let json_bytes = serde_json::to_vec(gpx)?;
    let buffer = std::io::Cursor::new(Vec::new());

    let mut encoder = Encoder::new(buffer, 0)?;
    encoder.write_all(&json_bytes)?;
    Ok(std::io::Cursor::new(encoder.finish()?.into_inner()))
}
