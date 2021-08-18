#[macro_use] extern crate gfx;

use old_school_gfx_glutin_ext::*;

use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::WindowBuilder,
    event_loop::{ControlFlow, EventLoop}
};
use gfx::{traits::FactoryExt,Device};

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

const SCREEN: [Vertex; 4] = [
    Vertex { pos: [1.0, 1.0] },   // Top right
    Vertex { pos: [-1.0, 1.0] },  // Top left
    Vertex { pos: [-1.0, -1.0] }, // Bottom left
    Vertex { pos: [1.0, -1.0] },  // Bottom right
];

const SCREEN_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    pipeline pipe {
        // Vertex buffer
        vbuf: gfx::VertexBuffer<Vertex> = (),

        // Uniforms
        i_resolution: gfx::Global<[f32; 3]> = "iResolution",

        // Output color
        frag_color: gfx::RenderTarget<ColorFormat> = "fragColor",
    }
}

fn main() {

  let (mut height, mut width) = (1080f32, 1920f32);

  let vert_src_buf = std::fs::read("shaders/default.vert").unwrap();
  let frag_src_buf = std::fs::read("shaders/default.frag").unwrap();

  let event_loop = EventLoop::new();
  let window_config = WindowBuilder::new()
      .with_title("shadertoy-rs")
      .with_inner_size(glutin::dpi::PhysicalSize::new(width, height));
  
  let (window, mut device, mut factory, main_color, mut main_depth) =
      glutin::ContextBuilder::new()
          .with_gfx_color_depth::<ColorFormat, DepthFormat>()
          .build_windowed(window_config, &event_loop).unwrap()
          .init_gfx::<ColorFormat, DepthFormat>();

  let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

  let pso = factory
      .create_pipeline_simple(&vert_src_buf, &frag_src_buf, pipe::new())
      .unwrap();

  let (vertex_buffer, slice) =
      factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

  let mut data = pipe::Data {
      vbuf: vertex_buffer,

      i_resolution: [width, height, width / height],

      frag_color: main_color,
  };


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
          match event {
            Event::MainEventsCleared => window.window().request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(size) => {
                    window.update_gfx(&mut data.frag_color, &mut main_depth);
                    window.resize(size);

                    width = size.width as f32;
                    height = size.height as f32;
                }
                _ => {}
            }

                Event::RedrawRequested(_) => {
                    data.i_resolution = [width, height, width / height];
                    // draw a frame
                    encoder.clear(&data.frag_color, CLEAR_COLOR);
                    encoder.draw(&slice, &pso, &data);
                    encoder.flush(&mut device);
                    window.swap_buffers().unwrap();
                    device.cleanup();
                }

                _ => {},
            }
        }
    );
    
}
