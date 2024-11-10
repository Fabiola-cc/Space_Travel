use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::renderer::{Renderer, ShaderType};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, renderer: &Renderer) -> Color {
  match renderer.current_shader {
      ShaderType::RandomColor => gaseous_giant_shader(fragment, uniforms),
      ShaderType::BlackAndWhite => black_and_white(fragment, uniforms),
      ShaderType::Dalmata => dalmata_shader(fragment, uniforms),
      ShaderType::Cloud => cloud_shader(fragment, uniforms),
      ShaderType::Cellular => cellular_shader(fragment, uniforms),
      ShaderType::Lava => solar_shader(fragment, uniforms),
      ShaderType::BlueGreen => blue_green_shader(fragment, uniforms),
  }
}

fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para un planeta rocoso
  let base_color = Color::new(139, 69, 19);      // Marrón rojizo (típico de Marte)
  let highlight_color = Color::new(210, 180, 140); // Color claro para resaltar montañas y bordes
  let shadow_color = Color::new(50, 25, 0);       // Sombra para simular cráteres y profundidad

  // Coordenadas de posición del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let depth = fragment.depth;

  // Parámetros para el efecto de ruido
  let zoom = 150.0;                // Factor de zoom para el ruido, define el tamaño de los cráteres y montañas
  let bumpiness = 0.3;             // Ajuste de rugosidad para dar más textura a la superficie
  let t = uniforms.time as f32 * 0.005; // Variación temporal mínima para animar pequeñas partículas en suspensión

  // Primer nivel de ruido para simular detalles grandes de la superficie (montañas, valles)
  let terrain_noise = uniforms.noise.get_noise_3d(
      x * zoom, 
      y * zoom, 
      depth * zoom
  );

  // Segundo nivel de ruido, de menor escala, para añadir detalles más pequeños (rugosidad de la superficie)
  let fine_detail_noise = uniforms.noise.get_noise_3d(
      x * zoom * 5.0, 
      y * zoom * 5.0, 
      depth * zoom * 5.0
  );

  // Combinación de ruidos para obtener una superficie rugosa y con variaciones
  let combined_noise = (terrain_noise * 0.6 + fine_detail_noise * 0.4) * bumpiness;

  // Selección de color en función del valor de ruido para crear efecto de montañas y sombras
  let color = if combined_noise > 0.4 {
      highlight_color  // Áreas más elevadas y bordes de montañas
  } else if combined_noise > 0.2 {
      base_color       // Color base para la mayor parte de la superficie
  } else {
      shadow_color     // Sombra para simular cráteres y depresiones
  };

  // Ajustar la intensidad para efectos de luz y sombra
  color * fragment.intensity
}


fn gaseous_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la atmósfera de un gigante gaseoso (pueden ajustarse para simular diferentes planetas)
  let cloud_color = Color::new(232, 220, 77);  
  let band_color1 = Color::new(255, 255, 255); 
  let band_color2 = Color::new(255, 215, 0);   
  let shadow_color = Color::new(245, 212, 122); 

  // Coordenadas de posición del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let depth = fragment.depth;

  // Parámetros de desplazamiento y movimiento de las capas de nubes
  let t = uniforms.time as f32 * 0.02;           // Tiempo para animar el movimiento de nubes
  let zoom = 200.0;                              // Factor de zoom para el ruido
  let speed_factor = 0.1;                        // Controla la velocidad de desplazamiento de bandas

  // Primer nivel de ruido para la capa de nubes
  let noise1 = uniforms.noise.get_noise_3d(
      x * zoom + t * speed_factor, 
      y * zoom, 
      depth * zoom
  );

  // Segundo nivel de ruido para más detalle
  let noise2 = uniforms.noise.get_noise_3d(
      x * zoom * 0.5 + t * speed_factor * 0.5, 
      y * zoom * 0.5, 
      depth * zoom * 0.5
  );

  // Mezclar los dos niveles de ruido
  let combined_noise = (noise1 * 0.6 + noise2 * 0.4).abs();

  // Calcular colores basados en el valor del ruido combinado para crear bandas atmosféricas
  let color = if combined_noise > 0.6 {
      cloud_color
  } else if combined_noise > 0.3 {
      band_color1.lerp(&band_color2, combined_noise)
  } else {
      shadow_color
  };

  // Ajustar la intensidad para simular efectos de luz y sombra
  color * fragment.intensity
}


fn solar_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la textura de "estrella" o "sol"
  let core_color = Color::new(255, 69, 0); // Naranja rojizo intenso
  let outer_glow_color = Color::new(255, 179, 0); // Naranja rojizo intenso
  let dark_spot_color = Color::new(255, 200, 0);      // Amarillo brillante

  // Obtener la posición del fragmento
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  // Parámetros de pulsación
  let base_frequency = 0.3;
  let pulsate_amplitude = 0.4;
  let t = uniforms.time as f32 * 0.015;

  // Efecto de pulsación en el eje z
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Zoom para detalles de ruido
  let zoom = 800.0; // Factor de zoom para controlar el tamaño de las "llamas" y "manchas"
  let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom,
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1200.0) * zoom,
      (position.y + 1200.0) * zoom,
      (position.z + 1200.0 + pulsate) * zoom,
  );

  // Combinación de ruido para suavizar la transición y agregar complejidad
  let noise_value = (noise_value1 + noise_value2) * 0.5;

  // Definir colores basados en el valor de ruido y umbrales para simular actividad solar
  let color = if noise_value > 0.4 {
      core_color.lerp(&outer_glow_color, noise_value * 0.8)
  } else if noise_value > -0.2 {
      outer_glow_color.lerp(&dark_spot_color, noise_value * 0.5)
  } else {
      dark_spot_color
  };

  color * fragment.intensity
}


fn blue_green_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Define los colores base para azul y verde
  let blue_color = Color::new(0, 0, 255);    // Azul
  let green_color = Color::new(0, 255, 0);   // Verde

  // Ajusta estos valores para controlar el tamaño y la repetición de los patrones de color
  let zoom = 50.0;
  let ox = 100.0;
  let oy = 100.0;

  // Coordenadas de ruido
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.1;  // Suaviza la variación en el tiempo

  // Calcula un valor de ruido en 2D
  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Umbral para decidir el color, ajustado para obtener una distribución variada
  let color = if noise_value > 0.0 {
    blue_color
  } else {
    green_color
  };

  // Multiplica por intensidad para aplicar efectos de iluminación
  color * fragment.intensity
}

fn tri_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Define los colores para cada capa
  let blue_color = Color::new(0, 0, 255);      // Azul
  let green_color = Color::new(0, 255, 0);     // Verde
  let cyan_color = Color::new(0, 255, 255);    // Cian (como color intermedio)

  // Configura valores para el tamaño y la repetición del patrón
  let zoom = 50.0;
  let ox = 100.0;
  let oy = 100.0;

  // Coordenadas de ruido y variación en el tiempo
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.1; // Suaviza la variación temporal del ruido

  // Calcula el valor de ruido en 2D
  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define umbrales para decidir el color basado en el valor de ruido
  let color = if noise_value > 0.5 {
      blue_color
  } else if noise_value > -0.5 {
      cyan_color // Color intermedio
  } else {
      green_color
  };

  // Ajusta el color según la intensidad de iluminación del fragmento
  color * fragment.intensity
}


fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as u64;

  let mut rng = StdRng::seed_from_u64(seed);

  let r = rng.gen_range(0..=255);
  let g = rng.gen_range(0..=255);
  let b = rng.gen_range(0..=255);

  let random_color = Color::new(r, g, b);

  random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

  let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

  let random_number = rng.gen_range(0..=100);

  let black_or_white = if random_number < 50 {
    Color::new(0, 0, 0)
  } else {
    Color::new(255, 255, 255)
  };

  black_or_white * fragment.intensity
}

fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(255, 255, 255); // White
  let base_color = Color::new(0, 0, 0); // Black

  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define cloud threshold and colors
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds
  let sky_color = Color::new(30, 97, 145); // Sky blue

  // Determine if the pixel is part of a cloud or sky
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    sky_color
  };

  noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
  let ox = 50.0;    // Offset x in the noise map
  let oy = 50.0;    // Offset y in the noise map
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Use a cellular noise function to create the plant cell pattern
  let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Define different shades of green for the plant cells
  let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
  let cell_color_2 = Color::new(124, 252, 0);   // Light green
  let cell_color_3 = Color::new(34, 139, 34);   // Forest green
  let cell_color_4 = Color::new(173, 255, 47);  // Yellow green

  // Use the noise value to assign a different color to each cell
  let final_color = if cell_noise_value < 0.15 {
    cell_color_1
  } else if cell_noise_value < 0.7 {
    cell_color_2
  } else if cell_noise_value < 0.75 {
    cell_color_3
  } else {
    cell_color_4
  };

  // Adjust intensity to simulate lighting effects (optional)
  final_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Base colors for the lava effect
  let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
  let dark_color = Color::new(130, 20, 0);   // Darker red-orange

  // Get fragment position
  let position = Vec3::new(
    fragment.vertex_position.x,
    fragment.vertex_position.y,
    fragment.depth
  );

  // Base frequency and amplitude for the pulsating effect
  let base_frequency = 0.2;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.01;

  // Pulsate on the z-axis to change spot size
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Apply noise to coordinates with subtle pulsating on z-axis
  let zoom = 1000.0; // Constant zoom factor
  let noise_value1 = uniforms.noise.get_noise_3d(
    position.x * zoom,
    position.y * zoom,
    (position.z + pulsate) * zoom
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
    (position.x + 1000.0) * zoom,
    (position.y + 1000.0) * zoom,
    (position.z + 1000.0 + pulsate) * zoom
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions

  // Use lerp for color blending based on noise value
  let color = dark_color.lerp(&bright_color, noise_value);

  color * fragment.intensity
}

