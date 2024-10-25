use macroquad::prelude::*;

pub fn draw_player(player_position: Vec2) {
    let left = vec2(player_position.x - 30.0, player_position.y + 30.0);
    let right = vec2(player_position.x + 30.0, player_position.y + 30.0);
    let top = vec2(player_position.x, player_position.y - 30.0);

    draw_triangle(left, right, top, PINK);
}

pub fn draw_drill(drill_position: Vec2, drill_size: f32) {
    let Vec2 { x, y } = drill_position;
    let r = drill_size / 2.0;
    let color = RED;
    draw_circle(x, y, r, color);
}

pub fn draw_resource(resource_position: Vec2, resource_size: f32) {
    draw_rectangle(
        resource_position.x - resource_size / 2.0,
        resource_position.y - resource_size / 2.0,
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
