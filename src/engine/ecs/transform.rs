use cgmath::{Rad, SquareMatrix};

pub struct Transform {
    pub translation: cgmath::Vector3<f32>,
    pub scale: f32,
    pub rotation: cgmath::Vector3<f32>
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translation: cgmath::vec3(0.0, 0.0, 0.0),
            scale: 1.0,
            rotation: cgmath::vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn mat4(&self) -> cgmath::Matrix4<f32> {
        //  I dont believe these matrix constructions are very expensive, so all this extra work
        //  shouldnt be too big a deal for now
        //  for more performance though I could probably derive the matrix construction
        //  and just calculate the final model matrix from scratch instead of multiplying
        //  all of these individual ones
        let translation = cgmath::Matrix4::from_translation(self.translation);

        let rotation_x = cgmath::Matrix4::from_angle_x(Rad(self.rotation.x));
        let rotation_y = cgmath::Matrix4::from_angle_y(Rad(self.rotation.x));
        let rotation_z = cgmath::Matrix4::from_angle_z(Rad(self.rotation.x));
        let rotation = rotation_x * rotation_y * rotation_z;

        let scale = cgmath::Matrix4::from_scale(self.scale);

        cgmath::Matrix4::identity() * translation * rotation * scale
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelPush {
    pub model: [[f32; 4]; 4]
}

impl ModelPush {
    pub fn new() -> Self {
        Self {
            model: cgmath::Matrix4::identity().into()
        }
    }
}
