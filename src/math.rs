use crate::framebuffer::FrameBufferPoint;

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn to_point(&self) -> FrameBufferPoint {
        FrameBufferPoint {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

pub trait VecOps {
    fn abs(&self) -> Self;
    fn length(&self) -> f32;
}

impl VecOps for Vec2 {
    fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
    ((a + (b - a)) as f32 * t) as u32
}

pub fn triangle_edge(a: &FrameBufferPoint, b: &FrameBufferPoint, p: &FrameBufferPoint) -> i32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}
