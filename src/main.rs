use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod renderer;
mod texture;
mod normal_map;
mod skybox;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use crate::fragment::Fragment;
use crate::color::Color;
use shaders::{vertex_shader, moon_shader, ring_shader, gaseous_giant_shader, black_and_white,
    dalmata_shader, cloud_shader, cellular_shader, solar_shader, blue_green_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};
use crate::renderer::{Renderer, NoiseUse, ShaderType, Object, Transform};
use texture::init_texture;
use normal_map::init_normal_map;
use skybox::Skybox;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite
}

struct Scene {
    objects: Vec<Object>, // Lista de objetos en la escena
}

fn create_noise(renderer: &Renderer) -> FastNoiseLite {
    match renderer.current_noise {
        NoiseUse::Cloud => create_cloud_noise(),
        NoiseUse::Cell => create_cell_noise(),
        NoiseUse::Ground => create_ground_noise(),
        NoiseUse::Lava => create_lava_noise(),
    }
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_cell_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_frequency(Some(0.1));
    noise
}

fn create_ground_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    
    // Use FBm fractal type to layer multiple octaves of noise
    noise.set_noise_type(Some(NoiseType::Cellular)); // Cellular noise for cracks
    noise.set_fractal_type(Some(FractalType::FBm));  // Fractal Brownian Motion
    noise.set_fractal_octaves(Some(5));              // More octaves = more detail
    noise.set_fractal_lacunarity(Some(2.0));         // Lacunarity controls frequency scaling
    noise.set_fractal_gain(Some(0.5));               // Gain controls amplitude scaling
    noise.set_frequency(Some(0.05));                 // Lower frequency for larger features

    noise
}

fn create_lava_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    
    // Use FBm for multi-layered noise, giving a "turbulent" feel
    noise.set_noise_type(Some(NoiseType::Perlin));  // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::FBm)); // FBm for layered detail
    noise.set_fractal_octaves(Some(6));             // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.0));        // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.5));              // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.002));                // Low frequency = large features
    
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

// Función para dibujar una órbita
fn draw_orbit(framebuffer: &mut Framebuffer, radius: f32) {
    let num_points = 100; // Puntos para aproximar el círculo
    let aspect_ratio = framebuffer.width as f32 / framebuffer.height as f32;

    for i in 0..num_points {
        let angle1 = 2.0 * std::f32::consts::PI * (i as f32) / (num_points as f32);
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) as f32) / (num_points as f32);

        // Ajustar por la relación de aspecto
        let x1 = radius * angle1.cos() * aspect_ratio;
        let z1 = radius * angle1.sin();
        let x2 = radius * angle2.cos() * aspect_ratio;
        let z2 = radius * angle2.sin();

        framebuffer.draw_line(
            (x1 + framebuffer.width as f32 / 2.0) as usize,
            (z1 + framebuffer.height as f32 / 2.0) as usize,
            (x2 + framebuffer.width as f32 / 2.0) as usize,
            (z2 + framebuffer.height as f32 / 2.0) as usize,
            0x888888, // Color de la órbita
        );
    }
}


fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], 
    fragment_shader: fn(&Fragment, &Uniforms) -> Color) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_scene(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    uniforms: &mut Uniforms,
) {
    for object in &scene.objects {
        // Actualizar la matriz modelo para el objeto actual
        let model_matrix = create_model_matrix(
            object.transform.position,
            object.transform.scale,
            object.transform.rotation,
        );

        let shader_function = match object.shader {
            ShaderType::MoonShader => moon_shader, // Usar el nuevo shader rocoso
            ShaderType::RingShader => ring_shader,
            ShaderType::RandomColor => gaseous_giant_shader,
            ShaderType::BlackAndWhite => black_and_white,
            ShaderType::Dalmata => dalmata_shader,
            ShaderType::Cloud => cloud_shader,
            ShaderType::Cellular => cellular_shader,
            ShaderType::Lava => solar_shader,
            ShaderType::BlueGreen => fragment_shader,
        };

        // actualizar la matriz modelo
        uniforms.model_matrix = model_matrix;

        // Obtener los vértices del objeto
        let vertex_array = object.model.get_vertex_array();

        // Renderizar el objeto
        render(framebuffer, &uniforms, &vertex_array, shader_function);
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Renderer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
        .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    // Variables para la luna
    let orbit_radius = 0.75; // Distancia de la luna al planeta
    let moon_speed = 0.05;      // Velocidad de órbita
    let mut time = 0;         // Tiempo actual (simulado para cálculo dinámico)

    // model position
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;

    // camera parameters ABCD
    let mut camera = Camera::new(
        Vec3::new(-2.0, 10.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    // Posiciones de los planetas en el eje X
    let positions = [
        -4.0, -2.0, 0.0, 2.0, 4.0, 6.0,
    ];

    // Escalas de los planetas
    let scales = [
        0.3, 0.9, 0.6, 1.0, 0.4, 0.5
    ];

    // Shaders para cada planeta
    let shaders = [
        ShaderType::BlackAndWhite,
        ShaderType::Dalmata,
        ShaderType::Cloud,
        ShaderType::Cellular,        
        ShaderType::RandomColor,
        ShaderType::BlueGreen,
    ];


    // Velocidades de órbita (en radianes por unidad de tiempo)
    let orbit_speeds = [0.01, 0.015, 0.04, 0.025, 0.003, 0.0035, 0.02, 0.06];

    // Distancias al planeta central
    let orbit_radii = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0];
    let orbit_draw = [65.0, 100.0, 130.0, 175.0, 220.0, 250.0];

    // Planeta central
    let mut objects: Vec<Object> = vec![Object {
        model: Obj::load("assets/models/sphere.obj").expect("Failed to load obj"),
        transform: Transform {
            position: Vec3::new(-6.0, 0.0, 0.0), // Centro de la órbita
            scale: 2.0,                  // Tamaño del planeta central
            rotation: Vec3::new(0.0, 0.0, 0.0),
        },
        shader: ShaderType::Lava,
    }];

    // Añadir los planetas orbitantes
    positions
        .iter()
        .zip(shaders.iter())
        .zip(scales.iter())
        .zip(orbit_speeds.iter())
        .zip(orbit_radii.iter())
        .enumerate()
        .for_each(|(index, ((((&x, &shader), &scale), &speed), &radius))| {
            objects.push(Object {
                model: Obj::load("assets/models/sphere.obj").expect("Failed to load obj"),
                transform: Transform {
                    position: Vec3::new(x, 0.0, 0.0), // Se ajustará dinámicamente
                    scale,
                    rotation: Vec3::new(0.0, 0.0, 0.0),
                },
                shader,
            });

            // Añadir luna al cuarto planeta
            if index == 3 {
                objects.push(Object {
                    model: Obj::load("assets/models/sphere.obj").expect("Failed to load moon obj"),
                    transform: Transform {
                        position: Vec3::new(x + 1.5 * scale, 0.5, 0.0), // Posición relativa al planeta
                        scale: 0.3 * scale,                            // Escala proporcional
                        rotation: Vec3::new(0.0, 0.0, 0.0),
                    },
                    shader: ShaderType::MoonShader, // Shader para la luna
                });
            }

            // Añadir anillos al último planeta
            if index == positions.len() - 1 {
                objects.push(Object {
                    model: Obj::load("assets/models/rings.obj").expect("Failed to load rings obj"),
                    transform: Transform {
                        position: Vec3::new(x, 0.0, 0.0), // Centrado en el planeta
                        scale: 0.3 * scale,               // Escala proporcional
                        rotation: Vec3::new(0.0, 0.0, 0.0),
                    },
                    shader: ShaderType::RingShader, // Shader para los anillos
                });
            }
        });


    let skybox = Skybox::new(5000);

    init_texture("assets/texture/planet.jpg").expect("Failed To load texture");
    init_normal_map("assets/texture/planet_normal.jpg").expect("Failed To load normal map");

    let noise = create_cloud_noise();
    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let mut uniforms = Uniforms { 
        model_matrix: Mat4::identity(), 
        view_matrix: Mat4::identity(), 
        projection_matrix, 
        viewport_matrix, 
        time: 0, 
        noise
    };

    let mut scene = Scene {
        objects,
    };    

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        let mut planet_ax = 0.0;
        let mut planet_az = 0.0;
        let mut planet_bx = 0.0;
        let mut planet_bz = 0.0;
        let aspect_ratio = framebuffer.width as f32 / framebuffer.height as f32;

        // Actualizar posiciones orbitales de los planetas
        scene.objects.iter_mut().enumerate().for_each(|(index, obj)| {
            // Ignorar la luna o los anillos que no deben orbitar
            if matches!(obj.shader, ShaderType::MoonShader | ShaderType::RingShader | ShaderType::Lava) {
                return;
            }

            // Calcular la posición orbital solo si no es el planeta central
            if index != 0 {
                let radius = orbit_radii[index - 1];
                let speed = orbit_speeds[index - 1];
                let angle = time as f32 * speed;

                // Actualizar la posición orbital
                obj.transform.position.x = -6.0 + radius * angle.cos() * aspect_ratio;
                obj.transform.position.z = radius * angle.sin();

                if index == 3 {
                    planet_ax = obj.transform.position.x;
                    planet_az = obj.transform.position.z;
                }
                if index == 6 {
                    planet_bx = obj.transform.position.x;
                    planet_bz = obj.transform.position.z;
                }
            }
        });

        // Actualizar la posición dinámica de la luna
        let moon_angle = time as f32 * moon_speed; // Ángulo basado en el tiempo
        let moon_x = orbit_radius * moon_angle.cos();
        let moon_z = orbit_radius * moon_angle.sin();

        // Actualizar la posición de la luna en la escena
        if let Some(moon) = scene.objects.iter_mut().find(|obj| matches!(obj.shader, ShaderType::MoonShader)) {
            moon.transform.position = Vec3::new(planet_ax + moon_x, 0.5, planet_az + moon_z); // Relativa al planeta central
        }
        if let Some(rings) = scene.objects.iter_mut().find(|obj| matches!(obj.shader, ShaderType::RingShader)) {
            rings.transform.position = Vec3::new(planet_bx, 0.0, planet_bz); // Relativa al planeta central
        }

        handle_input(&window, &mut camera);

        framebuffer.clear();

        skybox.render(&mut framebuffer, &uniforms, camera.eye);

        uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        uniforms.time = time;
        framebuffer.set_current_color(0xFFDDDD);

        if camera.eye == Vec3::new(-4.5, 15.00, 0.00){
            // Dibujar órbitas como líneas
            orbit_draw.iter().for_each(|&radius| {
                let scaled_radius = radius;
                draw_orbit(&mut framebuffer, scaled_radius);
            });
        }

        // Renderizar la escena completa
        render_scene(&mut framebuffer, &scene, &mut uniforms);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
        
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
        
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
        print!("{}", camera.eye);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
        
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
        
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
        
    }
    if window.is_key_down(Key::B) {
        camera.eye = Vec3::new(-4.5, 15.00, 0.00);
        camera.center = Vec3::new(-6.0, 0.0, 0.0);
        camera.up = Vec3::new(0.0, 1.0, 0.0);
    }

}
