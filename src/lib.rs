#[derive(Debug)]
pub struct YIQ {
    pub y: f32,
    pub i: f32,
    pub q: f32,
}

impl YIQ {
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        let matrix: [[f32; 3]; 3] = [
            [0.29889531, 0.58662247, 0.11448223],
            [0.59597799, -0.27417160, -0.32180189],
            [0.21147019, -0.52261711, 0.31114694],
        ];

        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let y = matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b;
        let i = matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b;
        let q = matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b;

        Self { y, i, q }
    }
}
