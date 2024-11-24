use crate::obj::Obj;
use nalgebra_glm::Vec3;

#[derive(Clone, Copy)]
pub enum ShaderType {
    RandomColor,
    BlackAndWhite,
    Dalmata,
    Cloud,
    Cellular,
    Lava,
    BlueGreen,
    MoonShader,
    RingShader,
}

pub struct Object {
    pub model: Obj,         // La geometría del objeto
    pub transform: Transform, // Transformaciones (posición, rotación, escala)
    pub shader: ShaderType
}

pub struct Transform {
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3
}