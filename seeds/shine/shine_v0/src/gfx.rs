
use wgpu::{BindGroup, Buffer, BufferAddress, Device, Queue, RenderPipeline, SurfaceConfiguration, TextureFormat};
use wgpu::util::DeviceExt;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    window::Window,
};

pub fn create_vertex_buffer(device: &Device, slice: &[u8]) -> Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: slice,
            usage: wgpu::BufferUsages::VERTEX,
        }
    )
}
pub fn create_index_buffer(device: &Device, slice: &[u8]) -> Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: slice,
            usage: wgpu::BufferUsages::INDEX,
        }
    )
}
pub fn create_dynamic_vertex_buffer(device: &Device, slice: &[u8]) -> Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: slice,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    )
}

// This function assumes the entire buffer is being overwritten, so data is not carried over if a resize happens.
pub fn easy_write_buffer(device: &Device, queue: &Queue, buffer: &mut Buffer, data: &[u8]) {
    let size = buffer.size() as usize;
    if data.len() >= size {
        let new_size = data.len().max(2 * size);
        let usage = buffer.usage();
        buffer.destroy();
        *buffer = device.create_buffer(&wgpu::BufferDescriptor{
            label: None,
            size: new_size as u64,
            usage,
            mapped_at_creation: false,
        });
    }
    queue.write_buffer(buffer, 0, data);
}

pub struct WindowStuff {
    pub window: Window,
    pub event_loop: EventLoop<()>,
}

pub struct GfxStuff<'window> {
    pub surface_config: SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'window>,
}


pub fn init_window(_width: u32, _height: u32) -> WindowStuff
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Not sure if this stuff works. Just copied it from online. Might help set window size to full page size.
    // {
    //     let canvas = window.canvas();
    //     let (width, height) = (canvas.client_width(), canvas.client_height());
    //     let factor = window.scale_factor();
    //     let logical = LogicalSize { width, height };
    //     let PhysicalSize { width, height } = logical.to_physical(factor);
    //     canvas.set_width(width as u32);
    //     canvas.set_height(height as u32);
    // }

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(_width, _height));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("shine")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    WindowStuff {
        window,
        event_loop
    }
}

pub async fn init_gfx<'window>(window: &'window Window) -> GfxStuff<'window>
{
    let size = window.inner_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        #[cfg(not(target_arch="wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch="wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });
    
    let surface = instance.create_surface(window).unwrap();

    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web, we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
                //wgpu::Limits::downlevel_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
            memory_hints: Default::default(),
        },
        None, // Trace path
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result in all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    GfxStuff {
        surface_config,
        queue,
        device,
        surface,
    }
}

pub fn run_events(event_loop: EventLoop<()>, window: &Window, window_events: &mut dyn FnMut(&WindowEvent, &winit::event_loop::EventLoopWindowTarget<()>)->()) 
{
    let _ = event_loop.run(
        |event, control_flow| {
            match event {
                Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                    window_events(event, control_flow);
                },
                // NewEvents(StartCause),
                // DeviceEvent { device_id: DeviceId, event: DeviceEvent, } => {},
                // UserEvent(T) => {},
                // Suspended => {},
                // Resumed => {},
                // AboutToWait => {},
                // LoopExiting => {},
                // MemoryWarning => {},
                _ => {}
            }
        }
    ).unwrap();
}


//====================================================================================================
// Simple text shader

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleTextVertex {
    pub base_position: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleTextInstance {
    //**TODO: Need a way to encode depth to implement layers. Could just add z coord.
    pub position: [f32; 2],
    pub scale: f32,
    //pub color: [f32; 3],
    pub color: u32,
    pub char_id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleTextVertUniforms {
    pub screen_to_clip: [f32; 12],  // 12 instead of 9, because columns each have an entry of padding.
}
impl SimpleTextVertUniforms {
    pub fn new() -> Self {
        Self {
            screen_to_clip: [0.0; 12],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleTextFragUniforms {
    pub pixel_size: [f32; 4],
}
impl SimpleTextFragUniforms {
    pub fn new() -> Self {
        Self {
            pixel_size: [0.0; 4],
        }
    }
}

pub struct SimpleTextPipeline {
    pub vert_uniform_buffer: Buffer,
    pub frag_uniform_buffer: Buffer,
    pub uniform_bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
}

pub fn make_simple_text_pipeline(device: &wgpu::Device, target_format: TextureFormat, vert_uniforms: SimpleTextVertUniforms, frag_uniforms: SimpleTextFragUniforms) -> SimpleTextPipeline
{
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader/simple_text.wgsl").into()),
    });

    let vert_uniform_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[vert_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );
    let frag_uniform_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[frag_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );

    let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
        label: Some("bind_group_layout"),
    });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: vert_uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: frag_uniform_buffer.as_entire_binding(),
            }
        ],
        label: Some("bind_group"),
    });
    
    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<SimpleTextVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ]
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<SimpleTextInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Unorm8x4, // format is abgr
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 4,
                            format: wgpu::VertexFormat::Uint32,
                        },
                    ]
                }
            ],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,  // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            unclipped_depth: false,  // Requires Features::DEPTH_CLIP_CONTROL
            conservative: false,  // Requires Features::CONSERVATIVE_RASTERIZATION
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });

    SimpleTextPipeline {
        vert_uniform_buffer,
        frag_uniform_buffer,
        uniform_bind_group,
        render_pipeline,
    }
}
