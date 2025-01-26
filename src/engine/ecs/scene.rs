use super::{
    ecs,
    transform,
    super::resources
};
use serde_json::Value;

pub async fn parse_scene(
    file: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
    cubemap_layout: &wgpu::BindGroupLayout
) -> ecs::World {
    let mut world = ecs::World::new();

    let scene: Value = serde_json::from_str(file).unwrap();
    let entities = scene["entities"].as_array().unwrap();

    for entity in entities {
        let world_entity = world.new_entity();

        if let Some(model_path) = entity["model_path"].as_str() {
            let entity_model = resources::load_model(model_path, device)
                .await
                .unwrap();
            world.add_component_to_entity(world_entity, entity_model);

            let texture_path = entity["texture_path"].as_str().unwrap_or("debug.png");
            let entity_texture = resources::load_material(device, queue, texture_layout, texture_path)
                .await
                .unwrap();
            world.add_component_to_entity(world_entity, entity_texture);
        }

        if let Some(gltf_path) = entity["gltf_path"].as_str() {
            let entity_model = resources::load_gltf(gltf_path, device)
                .await
                .unwrap();
            world.add_component_to_entity(world_entity, entity_model);

            let texture_path = entity["texture_path"].as_str().unwrap_or("debug.png");
            let entity_texture = resources::load_material(device, queue, texture_layout, texture_path)
                .await
                .unwrap();
            world.add_component_to_entity(world_entity, entity_texture);
        }

        if let Some(transform_obj) = entity["transform"].as_object() {
            let position = transform_obj["position"].as_array().unwrap();
            let rotation = transform_obj["rotation"].as_array().unwrap();
            let scale = transform_obj["scale"].as_f64().unwrap() as f32;

            let entity_transform = transform::Transform {
                translation: cgmath::vec3(
                    position[0].as_f64().unwrap() as f32,
                    position[1].as_f64().unwrap() as f32,
                    position[2].as_f64().unwrap() as f32
                ),
                scale,
                rotation: cgmath::vec3(
                    rotation[0].as_f64().unwrap() as f32,
                    rotation[1].as_f64().unwrap() as f32,
                    rotation[2].as_f64().unwrap() as f32
                )
            };
            world.add_component_to_entity(world_entity, entity_transform);
        }

        // skybox filepaths array loading
        if let Some(skybox_files) = entity["skybox"].as_array() {
            let skybox_files = skybox_files.iter().filter_map(|f| f.as_str()).collect();
            let skybox = resources::load_cubemap_files(skybox_files, device, queue, cubemap_layout)
                .await
                .unwrap();
            world.add_component_to_entity(world_entity, skybox);
        }
    }

    world
}
