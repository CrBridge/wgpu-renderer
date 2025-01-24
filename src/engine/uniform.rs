use super::camera::{
    Camera,
    Projection
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4],
    view_without_translation: [[f32; 4]; 4],
    projection: [[f32; 4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_projection: cgmath::Matrix4::identity().into(),
            view_without_translation: cgmath::Matrix4::identity().into(),
            projection: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera, projection: &Projection) {
        let projection_matrix = projection.calculate_matrix();
        let mut view_matrix = camera.calculate_matrix();
        self.view_projection = (projection_matrix * view_matrix).into();
        view_matrix.w = cgmath::vec4(0.0, 0.0, 0.0, 1.0);
        self.view_without_translation = view_matrix.into();
        self.projection = projection_matrix.into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    light_direction: [f32; 3],
    padding_1: u32,
    light_color: [f32; 3],
    padding_2: u32
}

impl LightUniform {
    pub fn new(
        direction: cgmath::Vector3<f32>, 
        color: cgmath::Vector3<f32>
    ) -> Self {
        Self {
            light_direction: direction.into(),
            padding_1: 0,
            light_color: color.into(),
            padding_2: 0
        }
    }
}
