#[derive(Clone, Copy)]
pub struct Material {
    pub color: image::Rgb<f32>,
    pub diffusion: f32,
    pub reflection: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Material {
            color: image::Rgb([r, g, b]),
            diffusion: 1.0,
            reflection: 3.0,
            shininess: 10.0,
        }
    }
}

