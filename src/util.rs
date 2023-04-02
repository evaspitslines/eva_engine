use glam::Vec2;

#[repr(C)]
pub struct Vertex {
    position: Vec2
}

impl Vertex {
    pub fn new(position: Vec2) -> Vertex {
        Self {
            position
        }
    }
}