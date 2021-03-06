extern crate gl;
extern crate glutin;
extern crate dust;

use dust::opengl::program::Program;
use dust::linalg::{Matrix4, Vector3};
use dust::linalg;
use dust::opengl;
use std::boxed::Box;
use glutin::GlContext;

mod util;
const VERTEX_SHADER_SRC : &'static [u8] = b"
#version 100

uniform mat4 projection;
uniform mat4 modelView;

attribute vec3 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = projection * modelView * vec4(position, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SRC : &'static [u8] = b"
#version 100

varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";


fn main() {
    println!("started!");
    let (mut event_loop, gl_window) = util::init("Colored Cubes");

    let program = Program::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

    let mesh2  = opengl::primitives::cube();

    let buff2 = mesh2.to_array_buffer_vbo();

    buff2.bind();

    let position = mesh2.attribs().by_name("position").unwrap();
    let color    = mesh2.attribs().by_name("color").unwrap();

    position.attrib_array_pointer( program.attrib_location("position").unwrap() ) ;
    color.attrib_array_pointer( program.attrib_location("color").unwrap() ) ;

    let (width, height) = gl_window.get_inner_size_pixels().unwrap();

    println!("size: {} x {}", &width, &height);

    let mut projection = Box::new(Matrix4::new());
    projection.projection(45.0, width as f32, height as f32, 0.1, 100.0);

    &program.uniform_matrix4fv_by_name("projection", &projection, false);

    let mut view = Matrix4::new();
    view.look_at(&Vector3::new(2.0, 2.0, 3.0), linalg::ZERO, linalg::Z_UP);

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);

        gl::Enable(gl::DEPTH_TEST);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut model     = Matrix4::new();
    let mut modelview = Box::new(Matrix4::new());

    let mut alpha = 0.0;
    let mut running = true;

    while running {

        event_loop.poll_events(|event| {
            running = util::continue_running(event);
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model.rotation(alpha, linalg::Z_UP);

            modelview.copy(&view);
            modelview.mult(&model);

            &program.uniform_matrix4fv_by_name("modelView", &modelview, false);

            &buff2.draw();
            //gl::DrawArrays(gl::TRIANGLES, 0, buff2.vertex_count() as i32);
        }

        alpha += 0.01;

        let _ = gl_window.swap_buffers().unwrap();
    }

}
