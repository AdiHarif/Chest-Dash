use macroquad::prelude::*;

use crate::player_sprite::PLAYER_DEST_SIZE;
use crate::GridPosition;
use crate::Player;

fn draw_player(player: &Player) {
    let Player {
        position,
        texture,
        sprite,
        ..
    } = player;

    let Vec2 { x, y } = *position - vec2(PLAYER_DEST_SIZE / 2.0, PLAYER_DEST_SIZE / 2.0);
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(sprite.frame().source_rect),
            dest_size: Some(vec2(PLAYER_DEST_SIZE, PLAYER_DEST_SIZE)),
            flip_x: player.flip_x,
            ..Default::default()
        },
    );
}

fn draw_chest(grid_position: &GridPosition, chest_size: f32, chest_texture: &Texture2D) {
    let screen_position = Vec2 {
        x: grid_position.x as f32 * chest_size,
        y: grid_position.y as f32 * chest_size,
    };
    let Vec2 { x, y } = screen_position;
    draw_texture_ex(
        chest_texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(chest_size, chest_size)),
            ..Default::default()
        },
    );
}

fn draw_resource(grid_position: &GridPosition, resource_size: f32, resource_texture: &Texture2D) {
    draw_texture_ex(
        resource_texture,
        grid_position.x as f32 * resource_size,
        grid_position.y as f32 * resource_size,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(resource_size, resource_size)),
            ..Default::default()
        },
    );
}

fn draw_grid_lines(cell_size: f32) {
    let grid_color = Color::new(0.0, 0.0, 0.0, 0.5);

    let rows_count = (screen_height() / cell_size) as u32;
    let cols_count = (screen_width() / cell_size) as u32;

    for i in 0..rows_count + 1 {
        let y = i as f32 * cell_size;
        draw_line(0.0, y, screen_width(), y, 1.0, grid_color);
    }

    for i in 0..cols_count + 1 {
        let x = i as f32 * cell_size;
        draw_line(x, 0.0, x, screen_height(), 1.0, grid_color);
    }
}

fn draw_terrain(cell_size: f32, tile_decorations_texture: &Texture2D) {
    let tile_rows_count = 1;
    let tile_cols_count = 4;
    let grid_rows_count = (screen_height() / cell_size) as i32 + tile_rows_count;
    let grid_cols_count = (screen_width() / cell_size) as i32 + tile_cols_count;

    for i in (0..grid_rows_count).step_by(tile_rows_count as usize) {
        for j in (0..grid_cols_count).step_by(tile_cols_count as usize) {
            let mut row_stagger = i % 4;
            if row_stagger > 1 {
                row_stagger = 5 - row_stagger;
            }
            let x = (j - row_stagger) as f32 * cell_size;
            let y = i as f32 * cell_size;
            draw_texture_ex(
                tile_decorations_texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        cell_size * tile_cols_count as f32,
                        cell_size * tile_rows_count as f32,
                    )),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_scores(player_score: f32, enemy_score: f32) {
    let score_font_size = 50.0;
    let score_height = 50.0;

    let player_text = format!("Player: {:>6.2}", player_score);
    draw_text(&player_text, 10.0, score_height, score_font_size, BLACK);

    let enemy_text = format!("Enemy: {:>6.2}", enemy_score);
    draw_text(
        &enemy_text,
        screen_width() - 300.0,
        score_height,
        score_font_size,
        BLACK,
    );
}

use crate::texture_manager::TextureManager;
use crate::TILE_SIZE;
use crate::{Resource, ResourceState};

pub fn draw_frame(
    player: &Player,
    enemy: &Player,
    resources: &Vec<Resource>,
    texture_manager: &TextureManager,
) {
    draw_terrain(TILE_SIZE, &texture_manager.get("tile_decorations").unwrap());
    for resource in resources {
        draw_resource(
            &resource.position,
            TILE_SIZE,
            &texture_manager.get("gold").unwrap(),
        );
        if resource.state == ResourceState::TakenByPlayer {
            draw_chest(
                &resource.position,
                TILE_SIZE,
                &texture_manager.get("chest").unwrap(),
            );
        }
        if resource.state == ResourceState::TakenByEnemy {
            draw_chest(
                &resource.position,
                TILE_SIZE,
                &texture_manager.get("enemy_chest").unwrap(),
            );
        }
    }
    draw_grid_lines(TILE_SIZE);

    draw_player(&enemy);
    draw_player(&player);

    draw_scores(player.score, enemy.score);
}
