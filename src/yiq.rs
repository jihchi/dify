#[derive(Debug, PartialEq)]
pub struct YIQ {
    y: f32, // luminance, in range [0, 1]
    i: f32, // hue of color, in range ~ [-0.5, 0.5]
    q: f32, // saturation of color, in range ~ [-0.5, 0.5]
}

impl YIQ {
    #[allow(clippy::many_single_char_names)]
    pub fn from_rgb(rgb: &image::Rgb<u8>) -> Self {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let y = 0.29_889_531 * r + 0.58_662_247 * g + 0.11_448_223 * b;
        let i = 0.59_597_799 * r + -0.27_417_160 * g + -0.32_180_189 * b;
        let q = 0.21_147_019 * r + -0.52_261_711 * g + 0.31_114_694 * b;

        Self { y, i, q }
    }

    // in the performance critical applications, square root can be omiitted
    pub fn squared_distance(&self, other: &Self) -> f32 {
        let delta_y = other.y - self.y;
        let delta_i = other.i - self.i;
        let delta_q = other.q - self.q;

        // introduce coefficients to compensate for irregularities
        0.5053 * delta_y.powi(2) + 0.299 * delta_i.powi(2) + 0.1957 * delta_q.powi(2)
    }

    pub fn square_root_distance(&self, other: &Self) -> f32 {
        self.squared_distance(other).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::YIQ;

    #[test]
    fn test_from_rgb() {
        let expected = YIQ {
            y: 0.0,
            i: 0.0,
            q: 0.0,
        };
        let actual = YIQ::from_rgb(&image::Rgb([0, 0, 0]));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_squared_distance_same() {
        let a = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.squared_distance(&b), 0.0);
    }

    #[test]
    fn test_squared_distance_not_same() {
        let a = YIQ {
            y: 0.5,
            i: 0.1,
            q: -0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.squared_distance(&b), 0.019_788);
    }

    #[test]
    fn test_square_root_distance_same() {
        let a = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.square_root_distance(&b), 0.0);
    }

    #[test]
    fn test_square_root_distance_not_same() {
        let a = YIQ {
            y: 0.5,
            i: 0.1,
            q: -0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.square_root_distance(&b), 0.14_066_982);
    }
}
