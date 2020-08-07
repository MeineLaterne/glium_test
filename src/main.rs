#![allow(dead_code)]

#[macro_use]
extern crate glium;

pub mod matrix;

use glium::{Surface, Vertex};
use glium::index::{self};
use glium::vertex::{self};
use matrix::Matrix4;

// Vertex Struct
#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 2],
}

implement_vertex!(Vert, position);

struct Shape<V> where V:Vertex {
    vertex_buffer: vertex::VertexBuffer<V>,
    index_buffer: index::IndexBuffer<u16>,
    primitive_type: index::PrimitiveType,
}

impl<V> Shape<V> where V:Vertex {
    fn new(ctx: &glium::Display, vertices: &[V], indices: &[u16], primitive_type: index::PrimitiveType) -> Shape<V> {
        Shape {
            vertex_buffer: vertex::VertexBuffer::new(ctx, &vertices).expect("failed to build VertexBuffer"),
            index_buffer: index::IndexBuffer::new(ctx, primitive_type, &indices).expect("failed to build IndexBuffer"),
            primitive_type: primitive_type,
        }
    }
}

struct GameObject {
    position: (f32, f32),
    size: (f32, f32),
    id: usize,
    transform: Matrix4<f32>,
    shape: Shape<Vert>,
}

impl GameObject {
    fn draw(&self, frame: &mut glium::Frame, program: &glium::Program, projection_matrix: &Matrix4<f32>) -> Result<(), glium::DrawError> {
        let uniforms = uniform! {
            transform: self.transform.to_array(),
            projection: projection_matrix.to_array()
        };
        
        frame.draw(
            &self.shape.vertex_buffer, 
            &self.shape.index_buffer,
            program, 
            &uniforms,
            &Default::default()
        )
    }

    fn update_transform(&mut self) {
        let translation_matrix = Matrix4::<f32>::translation(self.position.0, self.position.1, 0.0);
        self.transform *= translation_matrix;
    }
}

fn main() {
    // glium boilerplate:
    use glium::glutin;
    
    // 1. glutin event_loop erstellen
    let event_loop = glutin::event_loop::EventLoop::new();

    // 2. window builder
    let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(960.0, 540.0))
            .with_title("glium_test".to_string());

    // 3. context builder
    let cb = glutin::ContextBuilder::new();

    // 4. display ist unser fenster und opengl context
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    let top_right = Vert { position: [100.0, 100.0] };
    let bottom_right = Vert { position: [100.0, -100.0] };
    let bottom_left = Vert { position: [-100.0, -100.0] };
    let top_left = Vert { position: [-100.0, 100.0] };
    let shape = Shape::new(
        &display,
        &[bottom_left, top_left, top_right, bottom_right],
        &[1u16, 2, 0, 3],
        index::PrimitiveType::TriangleStrip);
    
    let mut game_object = GameObject {
        position: (540.0 - 50.0, 270.0 - 50.0),
        size: (1.0, 1.0),
        id: 1,
        shape: shape,
        transform: Matrix4::<f32>::new(),
    };

    game_object.update_transform();
    
    let projection_matrix = Matrix4::<f32>::orthographic(
        0.0, // left
        960.0, // right
        540.0, // bottom
        0.0, // top
        0.0, // near
        -1.0, // far
    );
    // shader quellcode
    let vertex_shader_src = include_str!("res/vert.glsl");
    let frag_shader_src = include_str!("res/frag.glsl");

    // shader program erstellen
    let program = glium::Program::from_source(&display, vertex_shader_src, frag_shader_src, None).unwrap();
    //let mat = matrix::rotation_2d(std::f64::consts::PI as f32);
    //println!("{:?}", mat);

    // zum schluss wird der event_loop gestartet
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
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

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.2, 0.2, 1.0);
        
        game_object.draw(&mut frame, &program, &projection_matrix).unwrap();
        frame.finish().unwrap();
    });
}
