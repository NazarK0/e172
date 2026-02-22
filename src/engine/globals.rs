#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub projection: [[f32; 4]; 4],
}

impl Globals {
    pub fn new(width: f32, height: f32) -> Self {
        // Матриця, що перетворює [0..W, 0..H] -> [-1..1, 1..-1]
        let sx = 2.0 / width;
        let sy = -2.0 / height;
        Self {
            projection: [
                [sx,  0.0, 0.0, 0.0],
                [0.0, sy,  0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-1.0, 1.0, 0.0, 1.0],
            ],
        }
    }
}
