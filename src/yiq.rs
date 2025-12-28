use image::Pixel;

#[derive(Debug, PartialEq)]
pub struct Yiq {
    y: f32, // luminance
    i: f32, // hue of color
    q: f32, // saturation of color
}

/// Calculate background color components for blending transparent pixels.
/// Uses position-dependent colors (like pixelmatch) to ensure transparent
/// and opaque versions of the same color compare as different.
///
/// Design considerations from https://github.com/mapbox/pixelmatch/pull/142:
/// - Non-uniform color (no solid background)
/// - No large areas of uniform color
/// - High perceptual variability
/// - Deterministic computation
/// - Function of pixel index only
/// - Avoids common colors (especially white and black)
/// - Contains no lines or patterns expected in test images
#[allow(clippy::excessive_precision)]
fn background_color(k: usize) -> (f32, f32, f32) {
    let r = 48.0 + 159.0 * ((k % 2) as f32);
    let g = 48.0 + 159.0 * ((k as f32 / 1.618033988749895).floor() as u32 % 2) as f32;
    let b = 48.0 + 159.0 * ((k as f32 / 2.618033988749895).floor() as u32 % 2) as f32;
    (r, g, b)
}

impl Yiq {
    #[allow(clippy::excessive_precision)]
    pub fn rgb2y(r: f32, g: f32, b: f32) -> f32 {
        0.298_895_31 * r + 0.586_622_47 * g + 0.114_482_23 * b
    }

    #[allow(clippy::excessive_precision)]
    pub fn rgb2i(r: f32, g: f32, b: f32) -> f32 {
        0.595_977_99 * r - 0.274_171_6 * g - 0.321_801_89 * b
    }

    #[allow(clippy::excessive_precision)]
    pub fn rgb2q(r: f32, g: f32, b: f32) -> f32 {
        0.211_470_19 * r - 0.522_617_11 * g + 0.311_146_94 * b
    }

    /// Convert RGBA to YIQ with position-dependent background blending for
    /// transparent pixels. This ensures transparent and opaque versions of the
    /// same color compare as different.
    pub fn from_rgba_with_pos(rgba: &image::Rgba<u8>, pos: usize) -> Self {
        let rgba_channels = rgba.channels();
        let r = f32::from(rgba_channels[0]);
        let g = f32::from(rgba_channels[1]);
        let b = f32::from(rgba_channels[2]);
        let a = f32::from(rgba_channels[3]);

        let (r, g, b) = if a < 255.0 {
            // Blend with position-dependent background for transparent/semi-transparent pixels
            let alpha = a / 255.0;
            let (bg_r, bg_g, bg_b) = background_color(pos);
            // Alpha blending: result = background + (foreground - background) * alpha
            // When alpha=0: pure background; when alpha=1: pure foreground
            (
                bg_r + (r - bg_r) * alpha,
                bg_g + (g - bg_g) * alpha,
                bg_b + (b - bg_b) * alpha,
            )
        } else {
            // Fully opaque - use RGB values as-is
            (r, g, b)
        };

        let y = Self::rgb2y(r, g, b);
        let i = Self::rgb2i(r, g, b);
        let q = Self::rgb2q(r, g, b);

        Self { y, i, q }
    }

    /// Convert RGBA to YIQ without transparency handling.
    ///
    /// # Deprecated
    ///
    /// This method does not handle transparency correctly. Use [`from_rgba_with_pos`]
    /// instead, which properly handles transparent and semi-transparent pixels
    /// by blending with a position-dependent background.
    #[deprecated(
        since = "0.8.0",
        note = "Use from_rgba_with_pos instead for correct transparency handling"
    )]
    #[allow(dead_code)]
    pub fn from_rgba(rgba: &image::Rgba<u8>) -> Self {
        let rgb = rgba.to_rgb();
        let rgb_channels = rgb.channels();
        let r = f32::from(rgb_channels[0]);
        let g = f32::from(rgb_channels[1]);
        let b = f32::from(rgb_channels[2]);
        let y = Self::rgb2y(r, g, b);
        let i = Self::rgb2i(r, g, b);
        let q = Self::rgb2q(r, g, b);

        Self { y, i, q }
    }

    pub fn delta_y(left: &image::Rgb<u8>, right: &image::Rgb<u8>) -> f32 {
        let left_channels = left.channels();
        let (left_r, left_g, left_b) = (
            f32::from(left_channels[0]),
            f32::from(left_channels[1]),
            f32::from(left_channels[2]),
        );

        let right_channels = right.channels();
        let (right_r, right_g, right_b) = (
            f32::from(right_channels[0]),
            f32::from(right_channels[1]),
            f32::from(right_channels[2]),
        );

        Self::rgb2y(left_r, left_g, left_b) - Self::rgb2y(right_r, right_g, right_b)
    }

    // in the performance critical applications, square root can be omiitted
    pub fn squared_distance(&self, other: &Self) -> f32 {
        let delta_y = self.y - other.y;
        let delta_i = self.i - other.i;
        let delta_q = self.q - other.q;
        let delta = 0.5053 * delta_y.powi(2) + 0.299 * delta_i.powi(2) + 0.195_7 * delta_q.powi(2);

        if self.y > other.y { -delta } else { delta }
    }

    pub fn blend_with_white(&self, alpha: f32) -> f32 {
        255.0 + (self.y - 255.0) * alpha
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
        let actual = Yiq::from_rgba_with_pos(&image::Rgba([0, 0, 0, 255]), 0);
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

    #[test]
    fn test_issue_32_transparent_vs_opaque_black() {
        // https://github.com/jihchi/dify/issues/32
        // Transparent black (#00000000) and opaque black (#000000FF)
        // should NOT compare as equal since they appear different visually.
        let opaque_black = Yiq::from_rgba_with_pos(&image::Rgba([0, 0, 0, 255]), 0);
        let transparent_black = Yiq::from_rgba_with_pos(&image::Rgba([0, 0, 0, 0]), 0);

        assert_ne!(
            opaque_black.squared_distance(&transparent_black),
            0.0,
            "Transparent black and opaque black should have different YIQ values"
        );
    }

    #[test]
    fn test_semi_transparent_pixels() {
        // Semi-transparent pixels (alpha between 0 and 255) should be handled
        // by blending with the background color
        let opaque = Yiq::from_rgba_with_pos(&image::Rgba([100, 50, 25, 255]), 0);
        let semi_transparent = Yiq::from_rgba_with_pos(&image::Rgba([100, 50, 25, 128]), 0);
        let transparent = Yiq::from_rgba_with_pos(&image::Rgba([100, 50, 25, 0]), 0);

        // All three should have different YIQ values due to different blending
        assert_ne!(opaque.y, semi_transparent.y);
        assert_ne!(opaque.y, transparent.y);
        assert_ne!(semi_transparent.y, transparent.y);
    }

    #[test]
    fn test_position_dependent_background() {
        // Same transparent color at different positions should have different
        // YIQ values due to position-dependent background blending
        let transparent_red_pos0 = Yiq::from_rgba_with_pos(&image::Rgba([255, 0, 0, 0]), 0);
        let transparent_red_pos1 = Yiq::from_rgba_with_pos(&image::Rgba([255, 0, 0, 0]), 1);

        assert_ne!(
            transparent_red_pos0, transparent_red_pos1,
            "Same transparent color at different positions should differ"
        );
    }

    #[test]
    fn test_fully_transparent_pixels_with_different_rgb_compare_equal() {
        // Fully transparent pixels (alpha=0) should compare equal regardless of RGB values
        // because they are visually identical (completely invisible)
        let pos = 42;
        let transparent_black = Yiq::from_rgba_with_pos(&image::Rgba([0, 0, 0, 0]), pos);
        let transparent_red = Yiq::from_rgba_with_pos(&image::Rgba([255, 0, 0, 0]), pos);
        let transparent_white = Yiq::from_rgba_with_pos(&image::Rgba([255, 255, 255, 0]), pos);

        assert_eq!(transparent_black, transparent_red);
        assert_eq!(transparent_black, transparent_white);
        assert_eq!(
            transparent_black.squared_distance(&transparent_red),
            0.0,
            "Fully transparent pixels should have zero distance regardless of RGB"
        );
    }

    #[test]
    fn test_opaque_pixels_position_independent() {
        // Opaque pixels should NOT be affected by position
        let opaque_red_pos0 = Yiq::from_rgba_with_pos(&image::Rgba([255, 0, 0, 255]), 0);
        let opaque_red_pos1 = Yiq::from_rgba_with_pos(&image::Rgba([255, 0, 0, 255]), 100);

        assert_eq!(
            opaque_red_pos0, opaque_red_pos1,
            "Opaque pixels should be position-independent"
        );
    }

    #[test]
    fn test_various_colors_with_transparency() {
        // Test that transparency handling works for different colors
        let color_rgb = [
            [255, 0, 0],     // red
            [0, 255, 0],     // green
            [0, 0, 255],     // blue
            [255, 255, 255], // white
        ];

        // All transparent colors should differ from their opaque equivalents
        for rgb in color_rgb {
            let transparent = Yiq::from_rgba_with_pos(&image::Rgba([rgb[0], rgb[1], rgb[2], 0]), 0);
            let opaque = Yiq::from_rgba_with_pos(&image::Rgba([rgb[0], rgb[1], rgb[2], 255]), 0);

            assert_ne!(
                transparent, opaque,
                "Transparent {:?} should differ from opaque",
                rgb
            );
        }
    }
}
