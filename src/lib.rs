//! [GDAL](http://gdal.org/) bindings for Rust.
//!
//! A high-level API to access the GDAL library, for vector and raster data.
//!
//! ## Use
//!
//! ```
//! use std::path::Path;
//! use gdal::vector::Dataset;
//!
//! let mut dataset = Dataset::open(Path::new("fixtures/roads.geojson")).unwrap();
//! let layer = dataset.layer(0).unwrap();
//! for feature in layer.features() {
//!     let highway_field = feature.field("highway").unwrap();
//!     let geometry = feature.geometry();
//!     println!("{} {}", highway_field.to_string().unwrap(), geometry.wkt().unwrap());
//! }
//! ```

#![crate_name="gdal"]
#![crate_type="lib"]

extern crate libc;
extern crate geo;
extern crate gdal_sys;
extern crate num_traits;
extern crate ndarray;
extern crate num;
extern crate byteorder;

#[macro_use]
extern crate error_chain;

pub use version::version_info;

mod utils;
mod gdal_major_object;
pub mod metadata;
pub mod version;
pub mod raster;
pub mod vector;
pub mod spatial_ref;
pub mod errors;
pub mod config;
