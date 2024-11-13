use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "gpx")]
pub struct Gpx {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@creator")]
    creator: String,
    metadata: Option<Metadata>,
    pub trk: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    name: Option<String>,
    desc: Option<String>,
    time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub name: Option<String>,
    pub trkseg: Vec<TrackSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackSegment {
    pub trkpt: Vec<TrackPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackPoint {
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    pub ele: Option<f64>,
    time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmlrGpx {
    pub trk: Vec<SmlrTrack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmlrTrack {
    pub trkseg: Vec<SmlrTrackSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmlrTrackSegment {
    pub trkpt: Vec<SmlrTrackPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmlrTrackPoint {
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    pub ele: Option<f64>,
}
