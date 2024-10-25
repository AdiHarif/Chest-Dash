mod draw;
use draw::*;

use macroquad::prelude::*;

fn is_mouse_over_resource(resource_position: Vec2, resource_size: f32) -> bool {
    let (x, y) = mouse_position();

    let x1 = resource_position.x - resource_size / 2.0;
    let x2 = resource_position.x + resource_size / 2.0;
    let y1 = resource_position.y - resource_size / 2.0;
    let y2 = resource_position.y + resource_size / 2.0;

    x >= x1 && x <= x2 && y >= y1 && y <= y2
}
#[macroquad::main("MyGame")]
async fn main() {
    let mut player_position = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let resource_position = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let resource_size = 60.0;
    let speed = 5.0;

    let mut drill_flag = false;

    loop {
        clear_background(BROWN);
        let mut direction = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        player_position += direction * speed;

        if is_mouse_button_released(MouseButton::Left)
            && !drill_flag
            && is_mouse_over_resource(resource_position, resource_size)
        {
            drill_flag = true;
        }

        draw_resource(resource_position, resource_size);
        draw_player(player_position);
        if drill_flag {
            draw_drill(resource_position, resource_size);
        }

        draw_grid_lines(resource_size);

        next_frame().await
    }
}
