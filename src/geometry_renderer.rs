use std::{mem, slice};
use std::borrow::Cow;
use glam::Vec2;
use wgpu::{Buffer, BufferDescriptor, BufferSlice, BufferUsages, Device, IndexFormat, PipelineLayout, RenderPass, RenderPipeline, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::util::Vertex;

pub struct GeometryRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,

    pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline
}

impl GeometryRenderer {
    pub fn new(device: &Device, surface_format: TextureFormat) -> Self {
        let vertices = [Vertex::new(Vec2::new(-0.5, 0.5)),
            Vertex::new(Vec2::new(0.5, 0.5)),
            Vertex::new(Vec2::new(-0.5, -0.5))];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: unsafe { slice::from_raw_parts(vertices.as_ptr() as *const _, vertices.len() * mem::size_of::<Vertex>())},
            usage: BufferUsages::VERTEX,
        });

        let indices: [u32; 3] = [0, 1, 2];

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: unsafe { slice::from_raw_parts(indices.as_ptr() as *const _, indices.len() * mem::size_of::<u32>())},
            usage: BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("triangle.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as _,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            pipeline_layout,
            pipeline
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw(0..3, 0..1);
    }
}