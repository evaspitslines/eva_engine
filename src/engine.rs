use crate::geometry_renderer::GeometryRenderer;
use futures_lite::future;


use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, Instance, InstanceDescriptor, PresentMode,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
    TextureViewDescriptor,
};
use winit::dpi::{PhysicalSize, Size};

use winit::event_loop::{EventLoop};
use winit::window::{Window, WindowBuilder};

pub const DEFAULT_WIDTH: u32 = 2560;
pub const DEFAULT_HEIGHT: u32 = 1440;

pub struct Engine {
    window: Window,
    instance: Instance,
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface_config: SurfaceConfiguration,

    geometry_renderer: GeometryRenderer,
}

impl Engine {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_inner_size(Size::Physical(PhysicalSize::new(
                DEFAULT_WIDTH,
                DEFAULT_HEIGHT,
            )))
            .with_title("Eva Engine")
            .build(event_loop)
            .unwrap();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = future::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let surface_format = surface.get_capabilities(&adapter).formats[0];

        let (device, queue) = future::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![surface_format],
        };

        surface.configure(&device, &surface_config);

        let geometry_renderer = GeometryRenderer::new(&device, surface_format);

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            geometry_renderer,
        }
    }

    #[inline]
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.geometry_renderer.render(&mut rpass);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    #[inline]
    pub fn device(&self) -> &Device {
        &self.device
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline]
    pub fn geometry_renderer(&self) -> &GeometryRenderer {
        &self.geometry_renderer
    }

    #[inline]
    pub fn geometry_renderer_mut(&mut self) -> &mut GeometryRenderer {
        &mut self.geometry_renderer
    }
}
