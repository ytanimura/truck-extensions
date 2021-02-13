use crate::*;
use glsl_to_spirv::ShaderType;

impl<'a> Rendered for SphericalBackground<'a> {
    impl_render_id!(id);
    fn vertex_buffer(
        &self,
        handler: &DeviceHandler,
    ) -> (Arc<BufferHandler>, Option<Arc<BufferHandler>>) {
        let device = handler.device();
        let vb = BufferHandler::from_slice(&[0_u32, 1, 2, 3], device, BufferUsage::VERTEX);
        let ib = BufferHandler::from_slice(&[0_u32, 1, 2, 2, 1, 3], device, BufferUsage::INDEX);
        (Arc::new(vb), Some(Arc::new(ib)))
    }
    fn bind_group_layout(&self, handler: &DeviceHandler) -> Arc<BindGroupLayout> {
        let device = handler.device();
        Arc::new(device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        }))
    }
    fn bind_group(&self, handler: &DeviceHandler, layout: &BindGroupLayout) -> Arc<BindGroup> {
        Arc::new(handler.device().create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[],
        }))
    }
    fn pipeline(
        &self,
        handler: &DeviceHandler,
        layout: &PipelineLayout,
        sample_count: u32,
    ) -> Arc<RenderPipeline> {
        let (device, sc_desc) = (handler.device(), handler.sc_desc());
        let vertex_binary =
            spirv_util::compile_glsl(include_str!("shaders/background.vert"), ShaderType::Vertex);
        let vertex_spirv = wgpu::util::make_spirv(&vertex_binary);
        let vertex_module = device.create_shader_module(vertex_spirv);
        let fragment_code = include_str!("shaders/background.frag").to_string() + self.shader;
        let fragment_binary = spirv_util::compile_glsl(&fragment_code, ShaderType::Fragment);
        let fragment_spirv = wgpu::util::make_spirv(&fragment_binary);
        let fragment_module = device.create_shader_module(fragment_spirv);
        Arc::new(
            handler
                .device()
                .create_render_pipeline(&RenderPipelineDescriptor {
                    layout: Some(layout),
                    vertex_stage: ProgrammableStageDescriptor {
                        module: &vertex_module,
                        entry_point: "main",
                    },
                    fragment_stage: Some(ProgrammableStageDescriptor {
                        module: &fragment_module,
                        entry_point: "main",
                    }),
                    rasterization_state: Some(RasterizationStateDescriptor {
                        front_face: FrontFace::Ccw,
                        cull_mode: CullMode::None,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                        clamp_depth: false,
                    }),
                    primitive_topology: PrimitiveTopology::TriangleList,
                    color_states: &[ColorStateDescriptor {
                        format: sc_desc.format,
                        color_blend: BlendDescriptor::REPLACE,
                        alpha_blend: BlendDescriptor::REPLACE,
                        write_mask: ColorWrite::ALL,
                    }],
                    depth_stencil_state: Some(DepthStencilStateDescriptor {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::LessEqual,
                        stencil: Default::default(),
                    }),
                    vertex_state: VertexStateDescriptor {
                        index_format: IndexFormat::Uint32,
                        vertex_buffers: &[VertexBufferDescriptor {
                            stride: std::mem::size_of::<u32>() as BufferAddress,
                            step_mode: InputStepMode::Vertex,
                            attributes: &[VertexAttributeDescriptor {
                                format: VertexFormat::Uint,
                                offset: 0,
                                shader_location: 0,
                            }],
                        }],
                    },
                    sample_count,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                    label: None,
                }),
        )
    }
}

impl<'a> SphericalBackground<'a> {
    pub fn new(shader: &'a str) -> Self {
        Self {
            shader,
            id: RenderID::default(),
        }
    }
}
