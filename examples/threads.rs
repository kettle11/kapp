extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;
use std::sync::mpsc::channel;
use std::thread;

fn new_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> <Context as HasContext>::Shader {
    #[cfg(all(target_arch = "wasm32"))]
    let version = "#version 300 es";
    #[cfg(all(not(target_arch = "wasm32")))]
    let version = "#version 410";

    let source = &format!("{}\n{}", version, source);
    unsafe {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            println!("Type: {:?}", shader_type);
            println!("{}", source);
            panic!(gl.get_shader_info_log(shader));
        }

        shader
    }
}

fn new_shader_program(
    gl: &glow::Context,
    vertex_source: &str,
    fragment_source: &str,
) -> <Context as HasContext>::Program {
    unsafe {
        let vertex_shader = new_shader(gl, VERTEX_SHADER, vertex_source);
        let fragment_shader = new_shader(gl, FRAGMENT_SHADER, fragment_source);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);

        if !gl.get_program_link_status(shader_program) {
            panic!(gl.get_program_info_log(shader_program));
        }
        shader_program
    }
}

fn setup(gl: &glow::Context) {
    let vertex_source = r#"
    const vec2 verts[3] = vec2[3](
        vec2(0.0f, 1.0f),
        vec2(-1.0f, -1.0f),
        vec2(1.0f, -1.0f)
    );
    void main() {
        gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
    }
    "#;

    let fragment_source = r#"
    precision mediump float;
  
    out vec4 color;
    void main()
    {
        color = vec4(0.0, 0.0, 1.0, 1.0);
    }
  "#;

    let shader_program = new_shader_program(&gl, vertex_source, fragment_source);

    unsafe {
        // OpenGL requires a vertex array to be bound for rendering, even if the vertex array is unused.
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        gl.use_program(Some(shader_program));
    }
}

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let mut app_main = app.clone();
    let window = app
        .new_window()
        .dimensions(20, 20)
        .title("Hello")
        .build()
        .unwrap();
    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    gl_context.set_window(Some(&window)).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    setup(&gl);
    let (tx, rx) = channel();

    unsafe {
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        gl.draw_arrays(TRIANGLES, 0, 3);
    }
    gl_context.swap_buffers();

    thread::spawn(move || {
        gl_context.make_current();
        while let Ok(event) = rx.recv() {
            match event {
                Event::WindowResized { .. } => {
                    gl_context.update_target();
                    unsafe {
                        gl.clear_color(0.0, 0.0, 0.0, 1.0);
                        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                        gl.draw_arrays(TRIANGLES, 0, 3);
                    }
                    gl_context.swap_buffers(); // Swaps the currently bound window. Blocks if vSync is used
                                               //app.request_frame();, // This call updates the window backbuffer to match the new window size.
                }
                Event::Draw => {}
                _ => {}
            }
        }
    });

    app_main.event_loop().run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => tx.send(event).unwrap(),
    });
}
