use image::Pixel;

#[derive(Debug, PartialEq)]
pub struct Yiq {
    y: f32, // luminance, in range [0, 1]
    i: f32, // hue of color, in range ~ [-0.5, 0.5]
    q: f32, // saturation of color, in range ~ [-0.5, 0.5]
}

impl Yiq {
    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    pub fn rgb2y(rgb: &image::Rgb<u8>) -> f32 {
        let rgb = rgb.channels();
        let r = f32::from(rgb[0]);
        let g = f32::from(rgb[1]);
        let b = f32::from(rgb[2]);

        0.298_895_31 * r + 0.586_622_47 * g + 0.114_482_23 * b
    }

    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn rgb2i(rgb: &image::Rgb<u8>) -> f32 {
        let rgb = rgb.channels();
        let r = f32::from(rgb[0]);
        let g = f32::from(rgb[1]);
        let b = f32::from(rgb[2]);

        0.595_977_99 * r - 0.274_171_6 * g - 0.321_801_89 * b
    }

    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn rgb2q(rgb: &image::Rgb<u8>) -> f32 {
        let rgb = rgb.channels();
        let r = f32::from(rgb[0]);
        let g = f32::from(rgb[1]);
        let b = f32::from(rgb[2]);

        0.211_470_19 * r - 0.522_617_11 * g + 0.311_146_94 * b
    }

    pub fn from_rgba(rgba: &image::Rgba<u8>) -> Self {
        let rgb = rgba.to_rgb();
        let y = Self::rgb2y(&rgb);
        let i = Self::rgb2i(&rgb);
        let q = Self::rgb2q(&rgb);

        Self { y, i, q }
    }

    pub fn delta_y(left: &image::Rgb<u8>, right: &image::Rgb<u8>) -> f32 {
        Self::rgb2y(left) - Self::rgb2y(right)
    }

    // in the performance critical applications, square root can be omiitted
    pub fn squared_distance(&self, other: &Self) -> f32 {
        let delta_y = self.y - other.y;
        let delta_i = self.i - other.i;
        let delta_q = self.q - other.q;
        let delta = 0.5053 * delta_y.powi(2) + 0.299 * delta_i.powi(2) + 0.195_7 * delta_q.powi(2);

        if self.y > other.y {
            -delta
        } else {
            delta
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Yiq;

    #[test]
    fn test_from_rgb() {
        let expected = Yiq {
            y: 0.0,
            i: 0.0,
            q: 0.0,
        };
        let actual = Yiq::from_rgba(&image::Rgba([0, 0, 0, 0]));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_squared_distance_same() {
        let a = Yiq {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        let b = Yiq {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.squared_distance(&b), 0.0);
    }
}
