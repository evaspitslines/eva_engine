use glam::Vec3;

#[repr(C)]
pub struct Vertex {
    position: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3) -> Vertex {
        Self { position }
    }
}
