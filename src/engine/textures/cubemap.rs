use image::GenericImageView;
use anyhow::*;
use wgpu::util::DeviceExt;

pub struct Cubemap {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler
}

impl Cubemap {
    pub fn from_bytes(
        binaries: Vec<Vec<u8>>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let mut images = Vec::new();
        for i in binaries.iter() {
            let image = image::load_from_memory(i)?;
            images.push(image);
        }
        let images = images.iter().collect();
        Self::from_image(images, device, queue)
    }

    pub fn from_image(
        images: Vec<&image::DynamicImage>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let dimensions = images[0].dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 6,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Cubemap Texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );
        for (layer, image) in images.iter().enumerate() {
            let rgba = image.to_rgba8();
            queue.write_texture(wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0, y: 0, z: layer as u32
                },
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1)
            },
            wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            });
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Cubemap Texture View"),
            dimension: Some(wgpu::TextureViewDimension::Cube),
            array_layer_count: Some(6),
            ..Default::default()
        });

        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Ok(Self { texture, view, sampler })
    }
}

pub struct CubemapBinding {
    pub bind_group: wgpu::BindGroup
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubemapVertex {
    pub position: [f32; 3]
}

impl CubemapVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CubemapVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

pub fn create_cubemap_vertices(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: &[CubemapVertex] = &[
        CubemapVertex{ position:[-1.0,  1.0, -1.0] },
        CubemapVertex{ position:[-1.0, -1.0, -1.0] },
        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },
        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },
        CubemapVertex{ position:[ 1.0,  1.0, -1.0] },
        CubemapVertex{ position:[-1.0,  1.0, -1.0] },

        CubemapVertex{ position:[-1.0, -1.0,  1.0] },
        CubemapVertex{ position:[-1.0, -1.0, -1.0] },
        CubemapVertex{ position:[-1.0,  1.0, -1.0] },
        CubemapVertex{ position:[-1.0,  1.0, -1.0] },
        CubemapVertex{ position:[-1.0,  1.0,  1.0] },
        CubemapVertex{ position:[-1.0, -1.0,  1.0] },

        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },
        CubemapVertex{ position:[ 1.0, -1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0, -1.0] },
        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },

        CubemapVertex{ position:[-1.0, -1.0,  1.0] },
        CubemapVertex{ position:[-1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0, -1.0,  1.0] },
        CubemapVertex{ position:[-1.0, -1.0,  1.0] },

        CubemapVertex{ position:[-1.0,  1.0, -1.0] },
        CubemapVertex{ position:[ 1.0,  1.0, -1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[ 1.0,  1.0,  1.0] },
        CubemapVertex{ position:[-1.0,  1.0,  1.0] },
        CubemapVertex{ position:[-1.0,  1.0, -1.0] },

        CubemapVertex{ position:[-1.0, -1.0, -1.0] },
        CubemapVertex{ position:[-1.0, -1.0,  1.0] },
        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },
        CubemapVertex{ position:[ 1.0, -1.0, -1.0] },
        CubemapVertex{ position:[-1.0, -1.0,  1.0] },
        CubemapVertex{ position:[ 1.0, -1.0,  1.0] }
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("Cubemap Vertex Buffer")),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX
    })
}
