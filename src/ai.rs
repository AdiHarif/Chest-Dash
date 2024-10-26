use macroquad::prelude::*;

use crate::get_tile_size;
use crate::Player;
use crate::Resource;
use crate::ResourceState;

pub fn get_closest_resource_index(resources: &Vec<Resource>, position: &Vec2) -> Option<usize> {
    let mut closest_resource_index = None;
    let mut closest_distance = f32::MAX;

    for (i, resource) in resources.into_iter().enumerate() {
        if resource.state != ResourceState::TakenByEnemy {
            let tile_size = get_tile_size();
            let resource_position = Vec2 {
                x: (resource.position.x as f32 + 0.5) * tile_size,
                y: (resource.position.y as f32 + 0.5) * tile_size,
            };
            let distance = (*position * get_tile_size()).distance(resource_position);
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
        let tile_size = get_tile_size();
        let closest_resource_position = Vec2 {
            x: (closest_resource.position.x as f32 + 0.5) * tile_size,
            y: (closest_resource.position.y as f32 + 0.5) * tile_size,
        };
        if (enemy.position * tile_size).distance(closest_resource_position)
            < enemy.reach * tile_size
        {
            resources[closest_resource_index].state = ResourceState::TakenByEnemy;
            enemy.update(&vec2(0.0, 0.0));
        } else {
            let direction = (closest_resource_position - (enemy.position * tile_size)).normalize();
            enemy.update(&direction);
        }
    }
}
