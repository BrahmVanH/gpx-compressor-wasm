pub mod gpx;
pub mod reduce;
pub mod compress;

pub use gpx::{ Gpx, SmlrGpx, SmlrTrack, SmlrTrackSegment, SmlrTrackPoint };
pub use reduce::reduce_gpx_size;
pub use compress::{ compress_gpx_to_memory, compress_gpx_to_file, compress_gpx_to_buffer };
