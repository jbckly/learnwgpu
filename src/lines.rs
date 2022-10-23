use super::Vertex;
use wgpu::util::DeviceExt;
use crate::point::Pt;


trait AddLine {
    fn addl(&mut self, line: (Pt, Pt));
}

trait Ins<T> {
    fn ins(&mut self, el: T);
}

impl<T> Ins<T> for Vec<T> {
    fn ins(&mut self, el: T) {
        self.insert(self.len(), el);
    }
}

impl Vertex for Pt {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Pt>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

impl AddLine for Vec<Pt> {
    fn addl(&mut self, line: (Pt, Pt)) {
        self.insert(self.len(), line.0);
        self.insert(self.len(), line.1);
    }
}
pub struct LinePass {
    line_vertices: Vec<Pt>,
    line_vertex_buffer: wgpu::Buffer,
    line_render_pipeline: wgpu::RenderPipeline,
}

impl LinePass {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let mut line_vertices: Vec<Pt> = Vec::new();
        line_vertices.addl((Pt (0.0158984, -0.4734766, 0.0), Pt(0.0158984, -0.4734766, 0.0)));

        line_vertices.ins(Pt(0.48239258, 0.01796875 * -1.0, 0.0));
        line_vertices.ins(Pt(0.48435547, -0.09550781 * -1.0, 0.0));

        line_vertices.ins(Pt(0.48435547, -0.09550781 * -1.0, 0.0));
        line_vertices.ins(Pt(0.48627930, -0.15149740 * -1.0, 0.0));

        line_vertices.ins(Pt(0.48627930, -0.15149740 * -1.0, 0.0));
        line_vertices.ins(Pt(0.50114258, 0.12231771 * -1.0, 0.0));

        line_vertices.ins(Pt(0.50114258, 0.12231771 * -1.0, 0.0));
        line_vertices.ins(Pt(0.49539062, 0.29201823 * -1.0, 0.0));

        line_vertices.ins(Pt(0.49539062, 0.29201823 * -1.0, 0.0));
        line_vertices.ins(Pt(0.44889648, 0.39617188 * -1.0, 0.0));

        line_vertices.ins(Pt(0.44889648, 0.39617188 * -1.0, 0.0));
        line_vertices.ins(Pt(0.30957031, 0.42516927 * -1.0, 0.0));

        let line_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Pass VB"),
            contents: bytemuck::cast_slice(&line_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let line_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader for lines"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_line.wgsl").into()),
        });

        let line_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Line Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let line_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&line_render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: "vs_main",
                buffers: &[Pt::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &line_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        LinePass {
            line_vertices,
            line_vertex_buffer,
            line_render_pipeline,
        }
    }

    pub fn render(
        &self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        camera_bind_group: &wgpu::BindGroup,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Depth Visual Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.line_render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.line_vertex_buffer.slice(..));
        render_pass.draw(0..self.line_vertices.len() as u32, 0..1);
    }

    pub fn set_lines(&mut self, device: &wgpu::Device, pts: Vec<Pt>) {
        self.line_vertices = pts;
        self.line_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Pass VB"),
            contents: bytemuck::cast_slice(&self.line_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
    }
}
