#[derive(Debug, PartialEq)]
pub struct YIQ {
    y: f32, // luminance, in range [0, 1]
    i: f32, // hue of color, in range ~ [-0.5, 0.5]
    q: f32, // saturation of color, in range ~ [-0.5, 0.5]
}

impl YIQ {
    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn rgb2y(rgb: &image::Rgb<u8>) -> f32 {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        0.298_895_31 * r + 0.586_622_47 * g + 0.114_482_23 * b
    }

    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn rgb2i(rgb: &image::Rgb<u8>) -> f32 {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        0.595_977_99 * r - 0.274_171_6 * g - 0.321_801_89 * b
    }

    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn rgb2q(rgb: &image::Rgb<u8>) -> f32 {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        0.211_470_19 * r - 0.522_617_11 * g + 0.311_146_94 * b
    }

    pub fn from_rgb(rgb: &image::Rgb<u8>) -> Self {
        let y = Self::rgb2y(rgb);
        let i = Self::rgb2i(rgb);
        let q = Self::rgb2q(rgb);

        Self { y, i, q }
    }

    pub fn delta_y(left: &image::Rgb<u8>, right: &image::Rgb<u8>) -> f32 {
        Self::rgb2y(left) - Self::rgb2y(right)
    }

    // in the performance critical applications, square root can be omiitted
    pub fn squared_distance(&self, other: &Self) -> f32 {
        let delta_y = other.y - self.y;
        let delta_i = other.i - self.i;
        let delta_q = other.q - self.q;

        // introduce coefficients to compensate for irregularities
        0.5053 * delta_y.powi(2) + 0.299 * delta_i.powi(2) + 0.195_7 * delta_q.powi(2)
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
        assert_eq!(a.square_root_distance(&b), 0.140_669_82);
    }
}
