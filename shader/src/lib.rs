//! A small SPIR-V interpreter used by the software rasterizer.

mod error;

pub mod glsl;
pub mod il;
pub mod interpreter;
pub mod spirv;

pub use error::Error;
