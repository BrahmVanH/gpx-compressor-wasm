use quick_xml::name::LocalName;
use quick_xml::se;
use std::{ fs, path::Path };
use wasm_bindgen::{ JsValue, prelude::* };
use crate::WasmDescribe;
use super::gpx::{ Gpx, SmlrGpx, SmlrTrack, SmlrTrackSegment, SmlrTrackPoint };

// use crate::gpx::{ Gpx, SmlrGpx, SmlrTrack, SmlrTrackSegment, SmlrTrackPoint };

// impl Gpx {
//     #[wasm_bindgen(constructor)]
//     pub fn new(version: String, creator: String) -> Gpx {
//         Gpx {
//             version,
//             creator,
//             metadata: None,
//             trk: Vec::new(),
//         }
//     }

//     // Getters
//     #[wasm_bindgen(getter)]
//     pub fn version(&self) -> String {
//         self.version.clone()
//     }

//     #[wasm_bindgen(getter)]
//     pub fn creator(&self) -> String {
//         self.creator.clone()
//     }

//     // You might want to add getters for tracks as JsValue
//     pub fn get_tracks(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
//         serde_wasm_bindgen::to_value(&self.trk)
//     }
// }

// #[wasm_bindgen]
// impl SmlrGpx {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> SmlrGpx {
//         SmlrGpx {
//             trk: Vec::new(),
//         }
//     }

//     // Add a method to convert to JsValue for JavaScript
//     pub fn to_js(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
//         serde_wasm_bindgen::to_value(&self)
//     }
// }

pub fn reduce_gpx_size(gpx: &Gpx) -> SmlrGpx {
    let smlr_gpx: SmlrGpx;

    println!("size of gpx struct: {} bytes", std::mem::size_of::<Gpx>());
    println!("original size of gpx: {} bytes", bincode::serialize(&gpx).unwrap().len());

    let mut smlr_trk: Vec<SmlrTrack> = Vec::new();
    for (track_index, track) in gpx.trk.iter().enumerate() {
        println!("\nTrack {}", track_index + 1);
        if let Some(name) = &track.name {
            println!("Track name: {}", name);
        }

        // let total_points: usize = track.trkseg
        //     .iter()
        //     .map(|seg| seg.trkpt.len())
        //     .sum();

        let mut smlr_trk_segs: Vec<SmlrTrackSegment> = Vec::new();

        for seg in &track.trkseg {
            // println!("segment: {:?}", seg);
            let smlr_seg: SmlrTrackSegment;
            let mut smlr_trk_pts: Vec<SmlrTrackPoint> = Vec::new();
            for track_point in &seg.trkpt {
                let rounded_lat = (track_point.lat * 100.0).round() / 100.0;
                let rounded_lon = (track_point.lon * 100.0).round() / 100.0;
                let rounded_ele = match track_point.ele {
                    Some(ele) => (ele * 100.0).round() / 100.0,
                    None => 0.0,
                };

                let smlr_trk_pt = SmlrTrackPoint {
                    lat: rounded_lat,
                    lon: rounded_lon,
                    ele: Some(rounded_ele),
                };
                smlr_trk_pts.push(smlr_trk_pt);
            }

            smlr_seg = SmlrTrackSegment {
                trkpt: smlr_trk_pts,
            };

            smlr_trk_segs.push(smlr_seg);
        }

        let smlr_track = SmlrTrack {
            trkseg: smlr_trk_segs,
        };

        smlr_trk.push(smlr_track);
    }

    smlr_gpx = SmlrGpx {
        trk: smlr_trk,
    };

    println!("size of smlr_gpx struct: {} bytes", std::mem::size_of::<SmlrGpx>());

    // if let Some(first_segment) = track.trkseg.first() {
    //     if let Some(first_point) = first_segment.trkpt.first() {
    //         println!("First Point - Lat: {}, Lon: {}", first_point.lat, first_point.lon);
    //         if let Some(ele) = first_point.ele {
    //             println!("Elevation: {}m", ele);
    //         }
    //     }
    // }
    return smlr_gpx;
}
