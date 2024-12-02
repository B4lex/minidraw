use crate::canvas::Canvas;
use crate::framebuffer::FrameBufferPoint;
use crate::math::Vec2;
use crate::primitives::{sdf_circle, sdf_rect};
use js_sys::Uint32Array;
use wasm_bindgen::prelude::wasm_bindgen;

mod animation;
pub mod canvas;
pub mod color;
pub mod framebuffer;
pub mod math;
pub mod primitives;

pub fn background(cnv: &mut canvas::Canvas, color: u32) {
    cnv.background = color;
    cnv.fb.fill(color);
    cnv.ssaa_fb.fill(color);
}

#[wasm_bindgen]
pub fn fill_slider_example(switched_on: bool) -> Uint32Array {
    let mut cnv = Canvas::new(1280, 720);

    let a = Vec2::new(-0.4, 0.3);
    let b = Vec2::new(0.4, -0.3);

    if switched_on {
        sdf_rect(
            &mut cnv,
            &a,
            &b,
            &FrameBufferPoint::new(108, 108),
            0xFF00FF11,
        );
        sdf_circle(&mut cnv, &Vec2::new(0.23, 0.0), 96, 0xFFCCCCCC);
    } else {
        sdf_rect(
            &mut cnv,
            &a,
            &b,
            &FrameBufferPoint::new(108, 108),
            0xFF888888,
        );
        sdf_circle(&mut cnv, &Vec2::new(-0.23, 0.0), 96, 0xFFCCCCCC);
    }
    Uint32Array::from(&cnv.fb.raw_buffer()[..])
}
