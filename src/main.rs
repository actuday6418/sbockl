use glium::{uniform, glutin, implement_vertex, Surface};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = std::fs::read("triangle.vert").expect("Unable to read shader!");
    let vertex_shader_src = std::str::from_utf8(&vertex_shader_src).unwrap();

    let fragment_shader_src = std::fs::read("triangle.frag").expect("Unable to read shader!");
    let fragment_shader_src = std::str::from_utf8(&fragment_shader_src).unwrap();
    
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        
       let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(6_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let vertex1 = Vertex {
            position: [-1.0, -1.0],
        };
        let vertex2 = Vertex {
            position: [1.0, -1.0],
        };
        let vertex3 = Vertex {
            position: [-1.0, 1.0],
        };
        let vertex4 = Vertex {
            position: [1.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        let res = target.get_dimensions();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform!(resolution: [res.0 as f32, res.1 as f32]),
                &Default::default(),
            )
            .unwrap();
        println!("{:?}",target.get_dimensions());
        target.finish().unwrap();
    });
}
