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
    layout: &wgpu::BindGroupLayout
) -> ecs::World {
    let mut world = ecs::World::new();

    let scene: Value = serde_json::from_str(file).unwrap();
    let entities = scene["entities"].as_array().unwrap();

    for entity in entities {
        let world_entity = world.new_entity();

        let model_path = entity["model_path"].as_str().unwrap();
        let texture_path = entity["texture_path"].as_str().unwrap();
        let transform_obj = entity["transform"].as_object().unwrap();
        let position = transform_obj["position"].as_array().unwrap();
        let rotation = transform_obj["rotation"].as_array().unwrap();
        let scale = transform_obj["scale"].as_f64().unwrap() as f32;

        let entity_model = resources::load_model(model_path, device)
            .await
            .unwrap();
        let entity_texture = resources::load_material(device, queue, layout, texture_path)
            .await
            .unwrap();

        let mut entity_transform = transform::Transform::new();
        entity_transform.translation = cgmath::vec3(
            position[0].as_f64().unwrap() as f32,
            position[1].as_f64().unwrap() as f32,
            position[2].as_f64().unwrap() as f32
        );
        entity_transform.rotation = cgmath::vec3(
            rotation[0].as_f64().unwrap() as f32,
            rotation[1].as_f64().unwrap() as f32,
            rotation[2].as_f64().unwrap() as f32
        );
        entity_transform.scale = scale;

        world.add_component_to_entity(world_entity, entity_model);
        world.add_component_to_entity(world_entity, entity_texture);
        world.add_component_to_entity(world_entity, entity_transform);
    }

    world
}
