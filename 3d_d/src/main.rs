mod fragment;
mod framebuffer;
mod line;
mod matrix;
mod obj;
mod shaders;
mod triangle;
mod vertex; // solo una vez

use crate::matrix::new_matrix4;
use framebuffer::Framebuffer;
use obj::Obj;
use rand::Rng;
use raylib::prelude::*;
use shaders::vertex_shader;
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = new_matrix4(
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = new_matrix4(
        cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let scale_matrix = new_matrix4(
        scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let translation_matrix = new_matrix4(
        1.0,
        0.0,
        0.0,
        translation.x,
        0.0,
        1.0,
        0.0,
        translation.y,
        0.0,
        0.0,
        1.0,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    scale_matrix * rotation_matrix * translation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();

    let mut rng = rand::thread_rng();

    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 >= transformed_vertices.len() {
            break;
        }

        let v0 = &transformed_vertices[i];
        let v1 = &transformed_vertices[i + 1];
        let v2 = &transformed_vertices[i + 2];

        let color = Vector3::new(0.96, 0.8, 0.27);
        let fragments = triangle(v0, v1, v2)
            .into_iter()
            .map(|mut frag| {
                frag.color = color;
                frag
            })
            .collect::<Vec<_>>();

        for frag in fragments {
            framebuffer.point(frag.position.x as i32, frag.position.y as i32, frag.color);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Rust Renderer - Triángulos con color sólido")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.1, 0.1, 0.15));
    framebuffer.init_texture(&mut window, &thread);

    let mut translation = Vector3::new(400.0, 300.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 120.0;

    let obj = Obj::load("assets/models/Fusee.obj").expect("❌ No se pudo cargar Fusee.obj");
    let vertex_array = obj.get_vertex_array();

    while !window.window_should_close() {
        handle_input(&mut window, &mut translation, &mut rotation, &mut scale);
        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { model_matrix };

        render(&mut framebuffer, &uniforms, &vertex_array);
        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));
    }
}

fn handle_input(
    window: &mut RaylibHandle,
    translation: &mut Vector3,
    rotation: &mut Vector3,
    scale: &mut f32,
) {
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        translation.x += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        translation.x -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        translation.y -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        translation.y += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        *scale += 2.0;
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        *scale -= 2.0;
    }
    if window.is_key_down(KeyboardKey::KEY_Q) {
        rotation.x -= PI / 32.0;
    }
    if window.is_key_down(KeyboardKey::KEY_W) {
        rotation.x += PI / 32.0;
    }
    if window.is_key_down(KeyboardKey::KEY_E) {
        rotation.y -= PI / 32.0;
    }
    if window.is_key_down(KeyboardKey::KEY_R) {
        rotation.y += PI / 32.0;
    }
    if window.is_key_down(KeyboardKey::KEY_T) {
        rotation.z -= PI / 32.0;
    }
    if window.is_key_down(KeyboardKey::KEY_Y) {
        rotation.z += PI / 32.0;
    }
}
