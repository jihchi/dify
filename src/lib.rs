pub mod cli;
pub mod diff;
mod yiq;

use crate::yiq::Yiq;
use image::{Pixel, RgbaImage};
use std::cmp;

fn get_diagonal_neighbours(x1: u32, y1: u32, width: u32, height: u32) -> ((u32, u32), (u32, u32)) {
    // (x0, y0)
    //          (x1, y1)
    //                   (x2, y2)
    let (x0, y0) = (x1.saturating_sub(1), y1.saturating_sub(1));
    let (x2, y2) = (
        cmp::min(x1.saturating_add(1), width.saturating_sub(1)),
        cmp::min(y1.saturating_add(1), height.saturating_sub(1)),
    );

    ((x0, y0), (x2, y2))
}

fn on_the_edge(top_left: (u32, u32), center: (u32, u32), bottom_right: (u32, u32)) -> bool {
    center.0 == top_left.0
        || center.0 == bottom_right.0
        || center.1 == top_left.1
        || center.1 == bottom_right.1
}

fn has_many_siblings(image: &RgbaImage, x1: u32, y1: u32, width: u32, height: u32) -> bool {
    let ((x0, y0), (x2, y2)) = get_diagonal_neighbours(x1, y1, width, height);

    let mut zeros: u32 = if on_the_edge((x0, y0), (x1, y1), (x2, y2)) {
        0
    } else {
        1
    };

    let center = &image.get_pixel(x1, y1);

    for x in x0..=x2 {
        for y in y0..=y2 {
            if x == x1 && y == y1 {
                continue;
            }

            if center == &image.get_pixel(x, y) {
                zeros += 1;
            }

            if zeros > 2 {
                return true;
            }
        }
    }

    false
}

pub fn antialiased(
    left: &RgbaImage,
    x1: u32,
    y1: u32,
    width: u32,
    height: u32,
    right: &RgbaImage,
) -> bool {
    let ((x0, y0), (x2, y2)) = get_diagonal_neighbours(x1, y1, width, height);

    let mut zeros: u32 = if on_the_edge((x0, y0), (x1, y1), (x2, y2)) {
        1
    } else {
        0
    };

    let mut min: f32 = 0.0;
    let mut max: f32 = 0.0;
    let mut min_x: u32 = 0;
    let mut min_y: u32 = 0;
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    let center = &left.get_pixel(x1, y1).to_rgb();

    for x in x0..=x2 {
        for y in y0..=y2 {
            if x == x1 && y == y1 {
                continue;
            }

            let neighbor = left.get_pixel(x, y).to_rgb();
            let delta = Yiq::delta_y(center, &neighbor);

            if delta == 0.0 {
                zeros += 1;
                if zeros > 2 {
                    return false;
                }
            } else if delta < min {
                min = delta;
                min_x = x;
                min_y = y;
            } else if delta > max {
                max = delta;
                max_x = x;
                max_y = y;
            }
        }
    }

    if min == 0.0 || max == 0.0 {
        return false;
    }

    (has_many_siblings(left, min_x, min_y, width, height)
        && has_many_siblings(right, min_x, min_y, width, height))
        || (has_many_siblings(left, max_x, max_y, width, height)
            && has_many_siblings(right, max_x, max_y, width, height))
}

pub fn blend_semi_transparent_white(color: f32, alpha: f32) -> f32 {
    255.0 + (color - 255.0) * alpha
}

#[cfg(test)]
mod tests {
    use super::blend_semi_transparent_white;

    #[test]
    fn test_blend_semi_transparent_white() {
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.0));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.1));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.3));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.5));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.7));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 0.9));
        assert_eq!(255.0, blend_semi_transparent_white(255.0, 1.0));

        assert_eq!(255.0, blend_semi_transparent_white(128.0, 0.0));
        assert_eq!(242.3, blend_semi_transparent_white(128.0, 0.1));
        assert_eq!(216.9, blend_semi_transparent_white(128.0, 0.3));
        assert_eq!(191.5, blend_semi_transparent_white(128.0, 0.5));
        assert_eq!(166.1, blend_semi_transparent_white(128.0, 0.7));
        assert_eq!(140.70001, blend_semi_transparent_white(128.0, 0.9));
        assert_eq!(128.0, blend_semi_transparent_white(128.0, 1.0));

        assert_eq!(255.0, blend_semi_transparent_white(0.0, 0.0));
        assert_eq!(229.5, blend_semi_transparent_white(0.0, 0.1));
        assert_eq!(178.5, blend_semi_transparent_white(0.0, 0.3));
        assert_eq!(127.5, blend_semi_transparent_white(0.0, 0.5));
        assert_eq!(76.5, blend_semi_transparent_white(0.0, 0.7));
        assert_eq!(25.5, blend_semi_transparent_white(0.0, 0.9));
        assert_eq!(0.0, blend_semi_transparent_white(0.0, 1.0));
    }
}
