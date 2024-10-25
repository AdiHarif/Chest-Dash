mod draw;

use draw::*;

use macroquad::prelude::*;

#[derive(PartialEq)]
struct GridPosition {
    x: i32,
    y: i32,
}

fn is_mouse_over_resource(resource_position: &GridPosition, resource_size: f32) -> bool {
    let mouse_vector = Vec2 {
        x: mouse_position().0,
        y: mouse_position().1,
    };
    let mouse_grid_position = GridPosition::from_screen_coordinates(mouse_vector, resource_size);

    return *resource_position == mouse_grid_position;
}

impl GridPosition {
    fn from_screen_coordinates(coordinates: Vec2, cell_size: f32) -> GridPosition {
        GridPosition {
            x: (coordinates.x / cell_size) as i32,
            y: (coordinates.y / cell_size) as i32,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MyGame".to_owned(),
        window_width: 640,
        window_height: 640,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let chest_texture = load_texture("assets/chest.png").await.unwrap();
    let gold_texture = load_texture("assets/gold.png").await.unwrap();
    let tile_decorations_texture = load_texture("assets/tile_decorations.png").await.unwrap();
    build_textures_atlas();

    let mut player_position = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let resource_size = 64.0;
    let resource_position = GridPosition::from_screen_coordinates(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        resource_size,
    );
    let speed = 5.0;

    let mut chest_flag = false;

    loop {
        clear_background(Color::from_hex(0x9d7658));
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
            && !chest_flag
            && is_mouse_over_resource(&resource_position, resource_size)
        {
            chest_flag = true;
        }

        draw_terrain(resource_size, &tile_decorations_texture);
        draw_resource(&resource_position, resource_size, &gold_texture);
        draw_player(player_position);
        if chest_flag {
            draw_chest(&resource_position, resource_size, &chest_texture);
        }

        draw_grid_lines(resource_size);

        next_frame().await
    }
}
