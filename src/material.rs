#[derive(Clone, Copy, Debug)]
pub struct Color([f32; 3]);

impl Color {
    pub fn black() -> Self {
        Color([0., 0., 0.])
    }
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        let rgb = self.0;
        let rgb_bytes = [
            (rgb[0] * 256.).max(0.).min(255.) as u8,
            (rgb[1] * 256.).max(0.).min(255.) as u8,
            (rgb[2] * 256.).max(0.).min(255.) as u8,
        ];
        rgb_bytes.into()
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, s: f32) -> Color {
        let [r, g, b] = self.0;
        Color([r * s, g * s, b * s])
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let [r, g, b] = self.0;
        let [or, og, ob] = other.0;
        Color([r + or, g + og, b + ob])
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        let [or, og, ob] = other.0;
        self.0[0] += or;
        self.0[1] += og;
        self.0[2] += ob;
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub diffusion: f32,
    pub reflection: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Material {
            color: Color([r, g, b]),
            diffusion: 1.0,
            reflection: 3.0,
            shininess: 10.0,
        }
    }
}
