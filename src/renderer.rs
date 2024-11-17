use crate::obj::Obj;
use nalgebra_glm::Vec3;

pub enum ShaderType {
    RandomColor,
    BlackAndWhite,
    Dalmata,
    Cloud,
    Cellular,
    Lava,
    BlueGreen,
}

pub enum NoiseUse {
    Cloud,
    Cell,
    Ground,
    Lava,
}

pub struct Renderer {
    pub current_shader: ShaderType,
    pub current_noise: NoiseUse,
    pub include_moon: bool,
    pub include_rings: bool,
}

pub struct Object {
    pub model: Obj,         // La geometría del objeto
    pub transform: Transform, // Transformaciones (posición, rotación, escala)
}

pub struct Transform {
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3
}

impl Renderer {
    pub fn change_shader(&mut self, key: u8) {
        match key {
            1 => {
                self.current_shader = ShaderType::RandomColor;
                self.current_noise = NoiseUse::Cloud;
                self.include_rings = true; // Ocultar anillos
                self.include_moon = false;  // Ocultar luna
            }
            2 => {
                self.current_shader = ShaderType::BlackAndWhite;
                self.current_noise = NoiseUse::Cell;
                self.include_rings = false; // Ocultar anillos
                self.include_moon = false;  // Ocultar luna
            }
            3 => {
                self.current_shader = ShaderType::Dalmata;
                self.current_noise = NoiseUse::Lava;
                self.include_rings = false; // Mostrar anillos
                self.include_moon = false; // Ocultar luna
            }
            4 => {
                self.current_shader = ShaderType::Cloud;
                self.current_noise = NoiseUse::Cloud;
                self.include_moon =true;  // Mostrar luna
                self.include_rings = false; // Ocultar anillos
            }
            5 => {
                self.current_shader = ShaderType::Cellular;
                self.current_noise = NoiseUse::Cell;
                self.include_rings = false; // Ocultar anillos
                self.include_moon = false;  // Ocultar luna
            }
            6 => {
                self.current_shader = ShaderType::Lava;
                self.current_noise = NoiseUse::Lava;
                self.include_rings = false; // Ocultar anillos
                self.include_moon = false;  // Ocultar luna
            }
            7 => {
                self.current_shader = ShaderType::BlueGreen;
                self.current_noise = NoiseUse::Ground;
                self.include_rings = false; // Ocultar anillos
                self.include_moon = false;  // Ocultar luna
            }
            _ => {} // Ignorar otras teclas
        }
    }
}
