use macroquad::prelude::*;

use crate::GridPosition;

pub fn draw_player(player_position: Vec2) {
    let left = vec2(player_position.x - 30.0, player_position.y + 30.0);
    let right = vec2(player_position.x + 30.0, player_position.y + 30.0);
    let top = vec2(player_position.x, player_position.y - 30.0);

    draw_triangle(left, right, top, PINK);
}

pub fn draw_drill(grid_position: &GridPosition, drill_size: f32) {
    let screen_position = Vec2 {
        x: grid_position.x as f32 * drill_size + drill_size / 2.0,
        y: grid_position.y as f32 * drill_size + drill_size / 2.0,
    };
    let Vec2 { x, y } = screen_position;
    let r = drill_size / 2.0;
    let color = RED;
    draw_circle(x, y, r, color);
}

pub fn draw_resource(grid_position: &GridPosition, resource_size: f32) {
    draw_rectangle(
        grid_position.x as f32 * resource_size,
        grid_position.y as f32 * resource_size,
        resource_size,
        resource_size,
        YELLOW,
    );
}

pub fn draw_grid_lines(cell_size: f32) {
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
