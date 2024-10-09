use crate::canvas::Canvas;
use crate::framebuffer::{FrameBufferPoint, Framebuffer};
use crate::math::{triangle_edge, Vec2, VecOps};

pub fn rect(cnv: &mut Canvas, a: &Vec2, b: &Vec2, color: u32) {
    let s_a = FrameBufferPoint::from_ndc_vec(a, cnv.fb.width, cnv.fb.height);
    let s_b = FrameBufferPoint::from_ndc_vec(b, cnv.fb.width, cnv.fb.height);

    let min_x = s_a.x.min(s_b.x);
    let max_x = s_a.x.max(s_b.x);
    let min_y = s_a.y.min(s_b.y);
    let max_y = s_a.y.max(s_b.y);

    for p_x in min_x..max_x {
        for p_y in min_y..max_y {
            cnv.fb
                .put_pixel(&mut FrameBufferPoint::new(p_x, p_y), color);
        }
    }
}

pub fn sdf_circle(cnv: &mut Canvas, center: &Vec2, radius: i32, color: u32) {
    let s_center = FrameBufferPoint::from_ndc_vec(center, cnv.fb.width, cnv.fb.height);

    for y in s_center.x - radius..s_center.x + radius {
        for x in s_center.y - radius..s_center.y + radius {
            let p_vec = Vec2::new(x as f32, y as f32);
            let sdf = (p_vec.clone() - center).length() - radius as f32;
            if sdf < 0. {
                cnv.fb.put_pixel(&p_vec.to_point(), color);
            }
        }
    }
}

pub fn sdf_rect(cnv: &mut Canvas, a: &Vec2, b: &Vec2, radius: i32, color: u32) {
    let s_a = FrameBufferPoint::from_ndc_vec(a, cnv.fb.width, cnv.fb.height);
    let s_b = FrameBufferPoint::from_ndc_vec(b, cnv.fb.width, cnv.fb.height);
    let center = FrameBufferPoint::new((s_a.x + s_b.x) / 2, (s_a.y + s_b.y) / 2);

    let half_dimensions =
        FrameBufferPoint::new((s_a.x - s_b.x).abs() / 2, (s_a.y - s_b.y).abs() / 2);

    for p_x in center.x - half_dimensions.x..center.x + half_dimensions.x {
        for p_y in center.y - half_dimensions.y..center.y + half_dimensions.y {
            let s_point_vec = (Vec2::new(p_x as f32, p_y as f32)
                - &Vec2::new(center.x as f32, center.y as f32))
                .abs();
            let s_point = s_point_vec.to_point();
            let dist_x = s_point.x - half_dimensions.x + radius;
            let dist_y = s_point.y - half_dimensions.y + radius;
            let dist_c = (s_point_vec - &half_dimensions.to_vec2()
                + &Vec2::new(radius as f32, radius as f32))
                .length();
            let sdf_dist = if dist_x > 0 && dist_y > 0 {
                dist_c
            } else {
                dist_x.max(dist_y) as f32
            };
            if sdf_dist < radius as f32 {
                cnv.fb.put_pixel(&FrameBufferPoint::new(p_x, p_y), color);
            }
        }
    }
}

fn draw_triangle_to_framebuffer(fb: &mut Framebuffer, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
    let s_a = FrameBufferPoint::from_ndc_vec(a, fb.width, fb.height);
    let s_b = FrameBufferPoint::from_ndc_vec(b, fb.width, fb.height);
    let s_c = FrameBufferPoint::from_ndc_vec(c, fb.width, fb.height);

    let min_x = s_a.x.min(s_b.x).min(s_c.x);
    let max_x = s_a.x.max(s_b.x).max(s_c.x);
    let min_y = s_a.y.min(s_b.y).min(s_c.y);
    let max_y = s_a.y.max(s_b.y).max(s_c.y);

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let p = FrameBufferPoint::new(x, y);
            let abp = triangle_edge(&s_a, &s_b, &p);
            let cbp = triangle_edge(&s_b, &s_c, &p);
            let acp = triangle_edge(&s_c, &s_a, &p);

            if (abp >= 0 && cbp >= 0 && acp >= 0) || (abp <= 0 && cbp <= 0 && acp <= 0) {
                fb.put_pixel(&p, color);
            }
        }
    }
}

pub fn triangle(cnv: &mut Canvas, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
    draw_triangle_to_framebuffer(&mut cnv.fb, a, b, c, color);
}

pub fn triangle_ssaa(cnv: &mut Canvas, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
    draw_triangle_to_framebuffer(&mut cnv.ssaa_fb, a, b, c, color);
}

pub fn line(cnv: &mut Canvas, a: &Vec2, b: &Vec2, color: u32) {
    let s_a = FrameBufferPoint::from_ndc_vec(a, cnv.fb.width, cnv.fb.height);
    let s_b = FrameBufferPoint::from_ndc_vec(b, cnv.fb.width, cnv.fb.height);

    let x_diff = s_a.x - s_b.x;
    let y_diff = s_a.y - s_b.y;
    let slope = y_diff as f32 / x_diff as f32;
    let c = s_a.y - (slope * s_a.x as f32) as i32;

    if x_diff != 0 {
        let mut l_x = s_a.x;
        let mut r_x = s_b.x;

        if l_x > r_x {
            std::mem::swap(&mut l_x, &mut r_x);
        }

        for x in l_x..r_x {
            let y = (slope * x as f32) as i32 + c;
            cnv.fb.put_pixel(&FrameBufferPoint::new(x, y), color);
        }
    } else {
        let mut l_y = s_a.y;
        let mut r_y = s_b.y;

        if l_y > r_y {
            std::mem::swap(&mut l_y, &mut r_y);
        }

        for y in l_y..r_y {
            cnv.fb.put_pixel(&FrameBufferPoint::new(s_a.x, y), color);
        }
    }
}
