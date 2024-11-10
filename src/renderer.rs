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
}

impl Renderer {
    pub fn change_shader(&mut self, key: u8) {
        match key {
            1 => {
                self.current_shader = ShaderType::RandomColor;
                self.current_noise = NoiseUse::Cloud;
            }
            2 => {
                self.current_shader = ShaderType::BlackAndWhite;
                self.current_noise = NoiseUse::Cell;
            }
            3 => {
                self.current_shader = ShaderType::Dalmata;
                self.current_noise = NoiseUse::Lava;
            }
            4 => {
                self.current_shader = ShaderType::Cloud;
                self.current_noise = NoiseUse::Cloud;
            }
            5 => {
                self.current_shader = ShaderType::Cellular;
                self.current_noise = NoiseUse::Cell;
            }
            6 => {
                self.current_shader = ShaderType::Lava;
                self.current_noise = NoiseUse::Lava;
            }
            7 => {
                self.current_shader = ShaderType::BlueGreen;
                self.current_noise = NoiseUse::Ground;
            }
            _ => {} // Ignorar otras teclas
        }
    }
}
