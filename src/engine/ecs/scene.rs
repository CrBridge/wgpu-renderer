// the world struct Is sort of a scene as it simply holds entities and their components
// This file would be a way to construct worlds (e.g. from parsing a file, etc.)
use super::ecs;

pub fn load_scene(file: &str) -> ecs::World {
    // parse json (or similar) to return world struct containing entities
    todo!();
}
