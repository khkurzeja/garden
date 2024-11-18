#![allow(unused_unsafe)]

mod ga_traits;
mod geom;
mod gfx;
mod input;
mod pga2d;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use winit::{event::*, keyboard::KeyCode, keyboard::PhysicalKey};

use ga_traits::*;
use geom::{Frame2, Vec2, Vec3};
use pga2d::*;

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
    let mut verts = vec![
        gfx::BasicVertex {
            position: [0.0, 0.0],
            color: [0.0, 0.0, 0.0],
        }; 
        3000  // Allow up to 1000 triangles
    ];

    let basic_pipeline = gfx::make_basic_pipeline(&device, surface_config.format, gfx::BasicUniforms::new());

    // === Init wgpu stuff ===

    let vertex_buffer = gfx::create_dynamic_vertex_buffer(&device, bytemuck::cast_slice(verts.as_slice()));

    let start = Univec { x: 100.0, y: 100.0, w: 1.0 };
    let dx = 200.0;
    let dy = 100.0;
    let motor = Oddvec { x: -dy/2.0, y: dx/2.0, w: 0.0, xyw: 1.0 };

    struct State {
        input: input::Input,
        verts: Vec<gfx::BasicVertex>,
        start: Univec,
        motor: Oddvec,
    }
    let mut state = State {
        input: input::Input::new(),
        verts,
        start,
        motor,
    };

    let update = |state: &mut State, width: f64, height: f64|
    {
        let input = &state.input;

        let mouse_pos = Vec2::from_array(input.mouse());
        let mouse_prev = Vec2::from_array(input.mouse_at(1));

        // Controls
        if input.mouse_down(MouseButton::Left) {
            
        }
    };

    let render = |state: &mut State, width: f32, height: f32| -> Result<(), wgpu::SurfaceError> 
    {
        let start = state.start;
        let motor = state.motor;

        // Update triangles
        let verts = &mut state.verts;
        verts.clear();
        let r = 6.0;
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(start.x + r, start.y).to_arr_f32(), 
            color: [0.0, 0.0, 0.0] 
        });
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(start.x + r * 2.094_f64.cos(), start.y + r * 2.094_f64.sin()).to_arr_f32(), 
            color: [0.0, 0.0, 0.0] 
        });
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(start.x + r * 4.18879_f64.cos(), start.y + r * 4.18879_f64.sin()).to_arr_f32(), 
            color: [0.0, 0.0, 0.0] 
        });

        let antireverse = |a: Oddvec| -> Oddvec {
            right_complement(reverse(left_complement(a)))
        };
        //**TODO: I need antireverse.
        let end = antimul(antimul(motor, start), antireverse(motor));
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(end.x + r, end.y).to_arr_f32(), 
            color: [1.0, 0.0, 0.0] 
        });
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(end.x + r * 2.094_f64.cos(), end.y + r * 2.094_f64.sin()).to_arr_f32(), 
            color: [1.0, 0.0, 0.0] 
        });
        verts.push(gfx::BasicVertex { 
            position: Vec2::new(end.x + r * 4.18879_f64.cos(), end.y + r * 4.18879_f64.sin()).to_arr_f32(), 
            color: [1.0, 0.0, 0.0] 
        });

        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(verts.as_slice()));

        let mut basic_uniforms = gfx::BasicUniforms::new();
        {
            let sx = 2.0 / width;
            let sy = -2.0 / height;
            let tx = -1.0;
            let ty = 1.0;
            basic_uniforms.screen_to_clip = [
                sx,  0.0, 0.0, 420.0,
                0.0, sy,  0.0, 420.0,
                tx,  ty,  1.0, 420.0,
            ];
        }
        queue.write_buffer(&basic_pipeline.uniform_buffer, 0, bytemuck::cast_slice(&[basic_uniforms]));

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
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
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
            render_pass.draw(0..verts.len() as _, 0..1);
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
