use super::math::Vec2;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn get_pixel(&self, p: &FrameBufferPoint) -> Option<u32> {
        if p.x >= 0 && p.y >= 0 {
            let idx = p.y as usize * self.width + p.x as usize;
            if idx <= self.width * self.height {
                return Some(self.buffer[idx]);
            }
        }
        None
    }

    pub fn put_pixel(&mut self, p: &FrameBufferPoint, color: u32) {
        if p.x >= 0 && p.y >= 0 {
            let idx = p.y as usize * self.width + p.x as usize;
            if idx <= self.width * self.height {
                self.buffer[idx] = color;
            }
        }
    }

    pub fn put_ndc_vec(&mut self, p: &Vec2, color: u32) {
        self.put_pixel(
            &FrameBufferPoint::from_ndc_vec(p, self.width, self.height),
            color,
        );
    }

    pub fn raw_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn fill(&mut self, color: u32) {
        for i in self.buffer.iter_mut() {
            *i = color;
        }
    }
}

#[derive(Clone, Debug)]
pub struct FrameBufferPoint {
    pub x: i32,
    pub y: i32,
}

impl FrameBufferPoint {
    pub fn new(x: i32, y: i32) -> FrameBufferPoint {
        FrameBufferPoint { x, y }
    }

    pub fn from_ndc_vec(v: &Vec2, width: usize, height: usize) -> FrameBufferPoint {
        FrameBufferPoint::new(
            ((v.x as f32 + 1.) * width as f32 / 2.) as i32,
            ((-v.y as f32 + 1.) * height as f32 / 2.) as i32,
        )
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}
