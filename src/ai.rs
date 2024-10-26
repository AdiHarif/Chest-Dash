use macroquad::prelude::*;

use crate::Player;
use crate::Resource;
use crate::ResourceState;
use crate::TILE_SIZE;

pub fn get_closest_resource_index(resources: &Vec<Resource>, position: &Vec2) -> Option<usize> {
    let mut closest_resource_index = None;
    let mut closest_distance = f32::MAX;

    for (i, resource) in resources.into_iter().enumerate() {
        if resource.state != ResourceState::TakenByEnemy {
            let resource_position = Vec2 {
                x: (resource.position.x as f32 + 0.5) * TILE_SIZE,
                y: (resource.position.y as f32 + 0.5) * TILE_SIZE,
            };
            let distance = position.distance(resource_position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_resource_index = Some(i);
            }
        }
    }

    closest_resource_index
}

pub fn update_enemy(resources: &mut Vec<Resource>, enemy: &mut Player) {
    let closest_resource_index = get_closest_resource_index(resources, &enemy.position);

    if closest_resource_index.is_none() {
        return;
    }

    if let Some(closest_resource_index) = closest_resource_index {
        let closest_resource = &resources[closest_resource_index];
        let closest_resource_position = Vec2 {
            x: (closest_resource.position.x as f32 + 0.5) * TILE_SIZE,
            y: (closest_resource.position.y as f32 + 0.5) * TILE_SIZE,
        };
        if enemy.position.distance(closest_resource_position) < enemy.reach {
            resources[closest_resource_index].state = ResourceState::TakenByEnemy;
            enemy.update(&vec2(0.0, 0.0));
        } else {
            let direction = (closest_resource_position - enemy.position).normalize();
            enemy.update(&direction);
        }
    }
}
