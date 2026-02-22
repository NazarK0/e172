#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectUniform {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}