//----------------------------------------------
// pick which implementation is compiled
//----------------------------------------------
#[cfg(feature = "simd")]
pub mod simd;

pub mod scalar;            // always available

//----------------------------------------------
// re‑export a single public entry point
//----------------------------------------------
#[cfg(feature = "simd")]
pub use simd::composite_frames;

#[cfg(not(feature = "simd"))]
pub use scalar::composite_frames;
