use std::io::{BufReader, Cursor};
use wgpu::util::DeviceExt;

use super::{model, textures, ecs};

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}

pub async fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue
) -> anyhow::Result<textures::texture::Texture> {
    let data = load_binary(file_name).await?;
    textures::texture::Texture::from_bytes(device, queue,&data, file_name)
}

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device
) -> anyhow::Result<model::Model> {
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, _obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text = load_string(&p).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        }
    ).await?;

    let meshes = models.into_iter().map(|m| {
        let vertices = (0..m.mesh.positions.len() / 3)
        .map(|i| {
            if m.mesh.normals.is_empty() {
                model::ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2]
                    ],
                    uv: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                    normal: [0.0, 0.0, 0.0]
                }
            } else {
                model::ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2]
                    ],
                    uv: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2]
                    ]
                }
            }
        }).collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", file_name)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", file_name)),
            contents: bytemuck::cast_slice(&m.mesh.indices),
            usage: wgpu::BufferUsages::INDEX
        });

        model::Mesh {
            _name: file_name.to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: m.mesh.indices.len() as u32
        }
    }).collect::<Vec<_>>();

    Ok(model::Model { meshes })
}

pub async fn load_material (
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    file_name: &str
) -> anyhow::Result<textures::texture::Material> {
    let diffuse = load_texture(file_name, device, queue).await?;
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse.view)
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse.sampler)
            }
        ]
    });
    Ok(textures::texture::Material { bind_group })
}

pub async fn load_scene(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
    cubemap_layout: &wgpu::BindGroupLayout
) -> anyhow::Result<ecs::ecs::World> {
    let json = load_string(file_name).await?;
    Ok(ecs::scene::parse_scene(&json, device, queue, texture_layout, cubemap_layout).await)
}

pub async fn load_cubemap_files(
    file_names: Vec<&str>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout
) -> anyhow::Result<textures::cubemap::CubemapComponent> {
    let mut binaries = Vec::new();
    for i in file_names.iter() {
        let binary_data =  load_binary(i).await?;
        binaries.push(binary_data);
    }
    let cubemap = textures::cubemap::Cubemap::from_bytes(binaries, device, queue)?;
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&cubemap.view)
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&cubemap.sampler)
            }
        ]
    });
    let vertices = textures::cubemap::create_cubemap_vertices(device);
    Ok(textures::cubemap::CubemapComponent { vertices, bind_group })
}
