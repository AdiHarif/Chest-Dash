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
