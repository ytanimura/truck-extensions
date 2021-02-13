use std::sync::Arc;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::*;

pub struct SphericalBackground<'a> {
    shader: &'a str,
    id: RenderID,
}

pub struct SphericalPolygonInstance<'a> {
    entity: PolygonInstance,
    shader: &'a str,
}

mod background;
mod spirv_util;
mod polygon;
