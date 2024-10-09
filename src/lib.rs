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
