use std::sync::Arc;
use truck_platform::*;
use wgpu::*;

pub struct SphericalBackground<'a> {
    shader: &'a str,
    id: RenderID,
}

mod background;
mod spirv_util;
