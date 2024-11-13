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

fn map(q: Vec2) -> Vec3 {
    Vec3::new(q.x, q.y, 1.0 - q.sq_length())
}

fn unmap(q: Vec3) -> Vec2 {
    if q.x == 0.0 && q.y == 0.0 { return Vec2::zero(); }
    //**TODO: If p.z == 0

    // Derivation of below
    // d = x / (1 - x^2)  // 1d map
    // d * (1 - x^2) = x
    // d - d*x^2 = x
    // -d*x^2 - x + d = 0  // Quadratic eq

    let d = (q.xy() / q.z).length();
    let s = if q.z < 0.0 { -1.0 } else { 1.0 };
    let r = (-1.0 + s * (1.0 + 4.0*d*d).sqrt()) / (2.0*d);
    q.xy().normalized() * r * s
}

fn full_map(q: Vec2, screen_to_clip: Frame2, map_camera: Frame2, camera: Frame2) -> Vec3 {
    let m = (map_camera * screen_to_clip * q.as_pt()).xy();
    camera * map(m)
}

fn full_unmap(q: Vec3, screen_to_clip: Frame2, map_camera: Frame2, camera: Frame2) -> Vec2 {
    (screen_to_clip.inverse() * map_camera.inverse() * unmap(camera.inverse() * q).as_pt()).xy()
}

fn screen_to_clip_frame(width: f64, height: f64) -> Frame2 {
    let s = 2.0 / height;
    let tx = -width / height;
    let ty = -1.0;
    Frame2 {
        origin: Vec2::new(tx, ty),
        u: Vec2::new(s, 0.0),
        v: Vec2::new(0.0, s),
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() 
{
    let gfx::WindowStuff {window, event_loop} = gfx::init_window(800, 600);
    let gfx::GfxStuff {mut surface_config, queue, device, surface} = gfx::init_gfx(&window).await;

    // === Data stuff ===

    type Vertex = gfx::BasicVertex;
    let mut verts = vec![];
    let mut indices = vec![];
    let num_sides = 20;
    verts.push(Vertex { position: [0.0, 0.0] });
    for i in 0..num_sides {
        let t = i as f64 / (num_sides - 1) as f64 * std::f64::consts::TAU;
        verts.push(Vertex { position: Vec2::new(t.cos(), t.sin()).to_arr_f32() });
        indices.push(0);
        indices.push(i as u16);
        indices.push((i + 1) % (num_sides + 1) as u16);
    }
    
    let mut instances = vec![];
    for j in 0..10 {
        for i in 0..10 {
            instances.push(gfx::BasicInstance {
                center: [i as f32 / 9.0 * 2.0 - 1.0, j as f32 / 9.0 * 2.0 - 1.0], 
                radius: 5.0 as f32,
                color: [i as f32 / 9.0, j as f32 / 9.0, 0.0],
            });
        }
    }

    let basic_pipeline = gfx::make_basic_pipeline(&device, surface_config.format, gfx::BasicUniforms::new());

    type PosVert = gfx::PositionVertex;
    let grid_verts: &[PosVert] = &[
        PosVert { position: [-1.0, -1.0] },
        PosVert { position: [1.0, -1.0] },
        PosVert { position: [1.0, 1.0] },
        PosVert { position: [-1.0, 1.0] },
    ];
    let grid_indices: &[u16] = &[
        0, 1, 2,
        0, 2, 3,
    ];

    let grid_pipeline = gfx::make_panoptigon_grid_pipeline(&device, surface_config.format, gfx::PanoptigonGridUniforms::new());

    // === Init wgpu stuff ===

    let vertex_buffer = gfx::create_vertex_buffer(&device, bytemuck::cast_slice(verts.as_slice()));
    let index_buffer = gfx::create_index_buffer(&device, bytemuck::cast_slice(indices.as_slice()));
    let instance_buffer = gfx::create_dynamic_vertex_buffer(&device, bytemuck::cast_slice(instances.as_slice()));

    let grid_vertex_buffer = gfx::create_vertex_buffer(&device, bytemuck::cast_slice(grid_verts));
    let grid_index_buffer = gfx::create_index_buffer(&device, bytemuck::cast_slice(grid_indices));

    struct State {
        frame_count: usize,
        map_cam: Frame2,
        cam: Frame2,
        input: input::Input,
        instances: Vec<gfx::BasicInstance>,
    }
    let mut state = State {
        frame_count: 0,
        map_cam: Frame2::identity(),
        cam: Frame2::identity().local_dilated(2.0),
        input: input::Input::new(),
        instances,
    };

    let update = |state: &mut State, width: f64, height: f64|
    {
        let input = &state.input;
        let instances = &mut state.instances;
        let map_cam = &mut state.map_cam;
        let cam = &mut state.cam;
        let screen_to_clip = screen_to_clip_frame(width, height);

        let mouse_pos = Vec2::from_array(input.mouse());
        let mouse_prev = Vec2::from_array(input.mouse_at(1));

        // Unfortunately, I think scroll delta might vary a lot between different machines.
        // Hopefully this choice generally works well.
        let scroll = input.mouse_wheel_delta[1] * 0.01;

        // Controls
        if input.key_down(KeyCode::ShiftLeft) {
            if input.mouse_down(MouseButton::Left) {
                let mapped_pos = (*map_cam * screen_to_clip * mouse_pos.as_pt()).xy();
                let mapped_prev = (*map_cam * screen_to_clip * mouse_prev.as_pt()).xy();
                map_cam.origin = map_cam.origin - (mapped_pos - mapped_prev);
            }
            if scroll != 0.0 {
                let mapped_pos = (*map_cam * screen_to_clip * mouse_pos.as_pt()).xy();
                let factor = (1.025f64).powf(-scroll);
                map_cam.origin = mapped_pos - factor * (mapped_pos - map_cam.origin);
                map_cam.u = factor * map_cam.u;
                map_cam.v = factor * map_cam.v;
            }
        }
        else {
            if input.mouse_down(MouseButton::Left) {
                let mapped_pos = full_map(mouse_pos, screen_to_clip, *map_cam, *cam);
                let mapped_prev = full_map(mouse_prev, screen_to_clip, *map_cam, *cam);

                if mapped_pos.z != 0.0 && mapped_prev.z != 0.0 {
                    let mapped_pos = mapped_pos.xy() / mapped_pos.z;
                    let mapped_prev = mapped_prev.xy() / mapped_prev.z;
                    cam.origin = cam.origin - (mapped_pos - mapped_prev);
                }
            }
            if scroll != 0.0 {
                let mapped_pos = full_map(mouse_pos, screen_to_clip, *map_cam, *cam);
                if mapped_pos.z != 0.0 {
                    let mapped_pos = mapped_pos.xy() / mapped_pos.z;
                    let factor = (1.025f64).powf(-scroll);
                    cam.origin = mapped_pos - factor * (mapped_pos - cam.origin);
                    cam.u = factor * cam.u;
                    cam.v = factor * cam.v;
                }
            }
        }

        // Move points
        let frame_count = state.frame_count;
        for i in 0..instances.len() {
            let dx = ((frame_count + i*i) as f32 * 0.01).sin() / 400.0;
            let dy = ((frame_count + i*i) as f32 * 0.01).cos() / 400.0;
            instances[i].center[0] += dx;
            instances[i].center[1] += dy;
        }
    };

    let render = |state: &mut State, width: f32, height: f32| -> Result<(), wgpu::SurfaceError> 
    {
        // Update uniforms
        let map_cam = &mut state.map_cam;
        let cam = &mut state.cam;        
        let mut grid_uniforms = gfx::PanoptigonGridUniforms::new();
        {
            grid_uniforms.camera = [
                cam.u.x as f32, cam.u.y as f32, 0.0, 420.0,
                cam.v.x as f32, cam.v.y as f32, 0.0, 420.0,
                cam.origin.x as f32, cam.origin.y as f32, 1.0, 420.0,
            ];
        }
        {
            grid_uniforms.map_camera = [
                map_cam.u.x as f32, map_cam.u.y as f32, 0.0, 420.0,
                map_cam.v.x as f32, map_cam.v.y as f32, 0.0, 420.0,
                map_cam.origin.x as f32, map_cam.origin.y as f32, 1.0, 420.0,
            ];
        }
        {
            let s = 2.0 / height;
            let tx = -width / height;
            let ty = -1.0;
            grid_uniforms.screen_to_clip = [
                s,   0.0, 0.0, 420.0,
                0.0, s,   0.0, 420.0,
                tx,  ty,  1.0, 420.0,
            ];
        }
        queue.write_buffer(&grid_pipeline.uniform_buffer, 0, bytemuck::cast_slice(&[grid_uniforms]));

        // Make copy of instances where centers are mapped to screen space. 
        // Would be nice to avoid the copy, but meh.
        let mut instances = state.instances.clone();
        for instance in &mut instances {
            let p = full_unmap(Vec3::new(instance.center[0] as f64, instance.center[1] as f64, 1.0), screen_to_clip_frame(width as f64, height as f64), *map_cam, *cam);
            instance.center = [p.x as f32, p.y as f32];
        }
        queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(instances.as_slice()));

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
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,  g: 0.2,  b: 0.3,  a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&grid_pipeline.render_pipeline);
            render_pass.set_bind_group(0, &grid_pipeline.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, grid_vertex_buffer.slice(..));
            render_pass.set_index_buffer(grid_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..grid_indices.len() as u32, 0, 0..1);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
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
