use cgmath::SquareMatrix;

pub struct Transform {
    pub translation: cgmath::Vector3<f32>,
    pub scale: f32,
    //pub rotation: cgmath::Vector3<f32>
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translation: cgmath::vec3(0.0, 0.0, 0.0),
            scale: 1.0,
            //rotation: cgmath::vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn mat4(&self) -> cgmath::Matrix4<f32> {
        let translation = cgmath::Matrix4::from_translation(self.translation);
        //let rotation = cgmath::Matrix4::angle...
        let scale = cgmath::Matrix4::from_scale(self.scale);
        // translation * rotation * scale, when I add rotation support
        cgmath::Matrix4::identity() * translation * scale
    }
}