use super::framebuffer::{FrameBufferPoint, Framebuffer};

pub struct Canvas {
    pub fb: Framebuffer,
    pub ssaa_fb: Framebuffer,
    pub background: u32,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            fb: Framebuffer::new(width, height),
            ssaa_fb: Framebuffer::new(width * 2, height * 2),
            background: 0xFFFFFFFF,
        }
    }

    pub fn downsample_fb(&mut self) {
        for y in (1..self.ssaa_fb.height as i32).step_by(2) {
            for x in (1..self.ssaa_fb.width as i32).step_by(2) {
                let blended_color = super::color::blend_colors(vec![
                    self.ssaa_fb
                        .get_pixel(&FrameBufferPoint::new(x, y))
                        .unwrap(),
                    self.ssaa_fb
                        .get_pixel(&FrameBufferPoint::new(x - 1, y))
                        .unwrap(),
                    self.ssaa_fb
                        .get_pixel(&FrameBufferPoint::new(x, y - 1))
                        .unwrap(),
                    self.ssaa_fb
                        .get_pixel(&FrameBufferPoint::new(x - 1, y - 1))
                        .unwrap(),
                ]);
                self.fb.put_pixel(
                    &FrameBufferPoint::new((x / 2) as i32, (y / 2) as i32),
                    blended_color,
                );
            }
        }
    }
}
