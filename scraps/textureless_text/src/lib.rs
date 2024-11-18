#![allow(unused_unsafe)]

mod geom;
mod gfx;
mod input;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use winit::{event::*, keyboard::KeyCode, keyboard::PhysicalKey};

use geom::{Frame2, Vec2, Vec3};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log { 
    ($($t:tt)*) => (unsafe{log(&format_args!($($t)*).to_string())}) 
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() 
{
    let gfx::WindowStuff {window, event_loop} = gfx::init_window(800, 600);
    let gfx::GfxStuff {mut surface_config, queue, device, surface} = gfx::init_gfx(&window).await;

    // === Data stuff ===

    type Vertex = gfx::BasicVertex;
    let mut verts = vec![];
    let mut indices: Vec<u16> = vec![];

    verts.push(Vertex { base_position: [0.0, 0.0] });
    verts.push(Vertex { base_position: [8.0, 0.0] });
    verts.push(Vertex { base_position: [8.0, 16.0] });
    verts.push(Vertex { base_position: [0.0, 16.0] });
    indices.push(0);
    indices.push(1);
    indices.push(2);
    indices.push(0);
    indices.push(2);
    indices.push(3);
    
    let mut instances = vec![
        gfx::BasicInstance {
            position: [0.0, 0.0],
            scale: 1.0,
            color: [0.0, 0.0, 0.0],
            char_id: 0,
        }; 26 * 6
    ];

    let basic_pipeline = gfx::make_basic_pipeline(&device, surface_config.format, gfx::BasicVertUniforms::new(), gfx::BasicFragUniforms::new());

    // === Init wgpu stuff ===

    let vertex_buffer = gfx::create_vertex_buffer(&device, bytemuck::cast_slice(verts.as_slice()));
    let index_buffer = gfx::create_index_buffer(&device, bytemuck::cast_slice(indices.as_slice()));
    let instance_buffer = gfx::create_dynamic_vertex_buffer(&device, bytemuck::cast_slice(instances.as_slice()));

    struct State {
        input: input::Input,
        instances: Vec<gfx::BasicInstance>,
        frame_count: u64,
    }
    let mut state = State {
        input: input::Input::new(),
        instances,
        frame_count: 0,
    };

    let update = |state: &mut State, width: f64, height: f64|
    {
        //let input = &state.input;
        //let frame_count = state.frame_count;
        let instances = &mut state.instances;
        
        //let scale = (frame_count as f32 * 0.001).sin() + 2.0;
        let scale = (state.input.mouse()[0] / width * 5.0) as f32;

        instances.clear();

        // Scaled by 1
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0, 100.0],
                scale: 1.0,
                color: [0.0, 0.0, 0.0],
                char_id: 'a' as u32 + i as u32,
            });
        }
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0, 116.0],
                scale: 1.0,
                color: [0.0, 0.0, 0.0],
                char_id: 'A' as u32 + i as u32,
            });
        }

        // Scaled by 2
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0 * 2.0, 100.0 + 16.0 * 2.0],
                scale: 2.0,
                color: [0.0, 0.0, 0.0],
                char_id: 'a' as u32 + i as u32,
            });
        }
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0 * 2.0, 100.0 + 16.0 * 2.0 + 32.0],
                scale: 2.0,
                color: [0.0, 0.0, 0.0],
                char_id: 'A' as u32 + i as u32,
            });
        }

        // Scaled by 1.5
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0 * scale, 100.0 + 16.0 * 2.0 + 32.0 * 2.0],
                scale,
                color: [0.0, 0.0, 0.0],
                char_id: 'a' as u32 + i as u32,
            });
        }
        for i in 0..26 {
            instances.push(gfx::BasicInstance {
                position: [10.0 + i as f32 * 8.0 * scale, 100.0 + 16.0 * 2.0 + 32.0 * 2.0 + 16.0 * scale],
                scale,
                color: [0.0, 0.0, 0.0],
                char_id: 'A' as u32 + i as u32,
            });
        }
    };

    let render = |state: &mut State, width: f32, height: f32| -> Result<(), wgpu::SurfaceError> 
    {
        let instances = &state.instances;
        queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(instances.as_slice()));

        let mut basic_vert_uniforms = gfx::BasicVertUniforms::new();
        {
            let sx = 2.0 / width;
            let sy = -2.0 / height;
            let tx = -1.0;
            let ty = 1.0;
            basic_vert_uniforms.screen_to_clip = [
                sx,  0.0, 0.0, 420.0,
                0.0, sy,  0.0, 420.0,
                tx,  ty,  1.0, 420.0,
            ];
        }
        queue.write_buffer(&basic_pipeline.vert_uniform_buffer, 0, bytemuck::cast_slice(&[basic_vert_uniforms]));

        let mut basic_frag_uniforms = gfx::BasicFragUniforms::new();
        {
            basic_frag_uniforms.pixel_size = [1.0 / width as f32,  1.0 / height as f32, 420.0, 420.0,];
        }
        queue.write_buffer(&basic_pipeline.frag_uniform_buffer, 0, bytemuck::cast_slice(&[basic_frag_uniforms]));

        let output = surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color{r: 1.0, g: 1.0, b: 1.0, a: 1.0}),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&basic_pipeline.render_pipeline);
            render_pass.set_bind_group(0, &basic_pipeline.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..instances.len() as _);
        }
    
        // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    };

    let mut surface_configured = false;

    let mut window_events = |event: &WindowEvent, control_flow: &winit::event_loop::EventLoopWindowTarget<()>| -> () {
        let width = surface_config.width;
        let height = surface_config.height;
        match event {
            WindowEvent::RedrawRequested => {
                window.request_redraw();
                if !surface_configured { return; }

                update(&mut state, width as f64, height as f64);
                let render_result = render(&mut state, width as f32, height as f32);
                match render_result {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => { surface.configure(&device, &surface_config); },  // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::OutOfMemory) => { control_flow.exit(); },
                    Err(wgpu::SurfaceError::Timeout) => {},  // This happens when the a frame takes too long to present
                }
                state.input.end_frame();
                state.frame_count += 1;
            },
            WindowEvent::Resized(physical_size) => {
                if physical_size.width > 0 && physical_size.height > 0 {
                    surface_config.width = physical_size.width;
                    surface_config.height = physical_size.height;
                    surface.configure(&device, &surface_config);
                    surface_configured = true;
                }
            },
            // device_id identifies which mouse/keyboard/screen was used
            WindowEvent::CursorMoved { device_id:_, position } => { state.input.mouse_position[0] = [position.x, position.y]; },
            WindowEvent::MouseInput { device_id:_, state:button_state, button } => { 
                state.input.mouse_down[0][input::button_id(*button)] = match button_state { winit::event::ElementState::Pressed => { true }, _ => { false } }; 
            },
            WindowEvent::MouseWheel { device_id:_, delta, phase:_ } => { 
                match delta {
                    winit::event::MouseScrollDelta::PixelDelta(d) => { state.input.mouse_wheel_delta = [d.x, d.y]; },
                    _ => {}
                }
            },
            WindowEvent::KeyboardInput { device_id:_, event, is_synthetic } => {
                if !is_synthetic {
                    match event.physical_key {
                        PhysicalKey::Code(code) => {
                            state.input.key_down[0][code as usize] = match event.state { winit::event::ElementState::Pressed => { true }, _ => { false } };
                        },
                        _ => {}
                    }
                }
            },
            WindowEvent::Touch(_touch) => {},
            WindowEvent::CloseRequested => { control_flow.exit() },
            _ => {},
        }
    };

    gfx::run_events(event_loop, &window, &mut window_events);
}
