#[macro_use]
extern crate glium;

fn create_uniform((w, h): (u32, u32), x: f32, y: f32, zoom: f32) -> [[f32;3]; 3] {
    
    let ratio = w as f32 / h as f32;    

    [
        [2.0 * ratio / zoom, 0.0, 0.0],
        [0.0, 2.0 / zoom, 0.0],
        [x, 0.0, 1.0]
    ]
}

// [1, 0, 0]
// [0, 1, 0]
// [0, 0, 1]

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-1.0, -1.0] };
    let vertex2 = Vertex { position: [ 1.0,  -1.0] };
    let vertex3 = Vertex { position: [ 1.0, 1.0] };
    let vertex4 = Vertex { position: [ -1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    

    let vertex_shader_src = include_str!("./shader/shader.vs");
    let fragment_shader_src = include_str!("./shader/shader.fs");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut zoomer = 9.0f32;
    
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        //zoomer *= 1.115;

        let uniforms = uniform! {
            transform: create_uniform(display.gl_window().window().inner_size().into(), -1.5, 0.0, zoomer),
            zoom: zoomer,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);        
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}