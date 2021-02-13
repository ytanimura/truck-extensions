use crate::*;
use glsl_to_spirv::ShaderType;

impl<'a> Rendered for SphericalPolygonInstance<'a> {
    derive_render_id!(entity);
    derive_vertex_buffer!(entity);
    derive_bind_group_layout!(entity);
    derive_bind_group!(entity);
    fn pipeline(
        &self,
        handler: &DeviceHandler,
        layout: &PipelineLayout,
        sample_count: u32,
    ) -> Arc<RenderPipeline> {
        let vertex_shapder = PolygonInstance::default_vertex_shader();
        let fragment_code = include_str!("shaders/polygon.frag").to_string() + self.shader;
        let fragment_binary = spirv_util::compile_glsl(&fragment_code, ShaderType::Fragment);
        let fragment_shader = wgpu::util::make_spirv(&fragment_binary);
        self.entity.pipeline_with_shader(
            vertex_shapder,
            fragment_shader,
            handler,
            layout,
            sample_count,
        )
    }
}

impl<'a> SphericalPolygonInstance<'a> {
    pub fn new(entity: PolygonInstance, shader: &'a str) -> Self { Self { entity, shader } }
}
