mod draw;
mod player;
mod player_sprite;

use draw::*;
use player::*;
use player_sprite::get_player_sprite;

use macroquad::prelude::*;

const TILE_SIZE: f32 = 64.0;

#[derive(PartialEq)]
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum ResourceState {
    Free,
    Taken,
}

struct Resource {
    position: GridPosition,
    state: ResourceState,
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
        window_width: GRID_COLS_COUNT as i32 * TILE_SIZE as i32,
        window_height: GRID_ROWS_COUNT as i32 * TILE_SIZE as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

const RESOURCES_ROWS_COUNT: u32 = 3;
const GRID_ROWS_COUNT: u32 = 15;
const GRID_COLS_COUNT: u32 = 15;

fn initalize_resources(grid_rows_count: u32, grid_cols_count: u32) -> Vec<Resource> {
    let resources_rows_spacing = grid_rows_count / (RESOURCES_ROWS_COUNT + 1) + 1;
    let resources_cols_spacing = grid_cols_count / (RESOURCES_ROWS_COUNT + 1) + 1;
    let mut resources = Vec::new();

    for i in (resources_rows_spacing - 1..grid_rows_count).step_by(resources_rows_spacing as usize)
    {
        for j in
            (resources_cols_spacing - 1..grid_cols_count).step_by(resources_cols_spacing as usize)
        {
            resources.push(Resource {
                position: GridPosition {
                    x: i as i32,
                    y: j as i32,
                },
                state: ResourceState::Free,
            });
        }
    }

    resources
}

#[macroquad::main(window_conf)]
async fn main() {
    let chest_texture = load_texture("assets/chest.png").await.unwrap();
    let gold_texture = load_texture("assets/gold.png").await.unwrap();
    let tile_decorations_texture = load_texture("assets/tile_decorations.png").await.unwrap();
    let player_texture = load_texture("assets/player_spritesheet.png").await.unwrap();
    build_textures_atlas();

    let player_sprite = get_player_sprite();

    let player_starting_position = vec2(1.5 * TILE_SIZE, screen_height() / 2.0);
    let mut player = Player::new(player_starting_position, player_texture, player_sprite);
    let mut resources = initalize_resources(GRID_ROWS_COUNT, GRID_COLS_COUNT);

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

        player.update(&direction);

        if is_mouse_button_released(MouseButton::Left) {
            for resource in &mut resources {
                if is_mouse_over_resource(&resource.position, TILE_SIZE) {
                    if resource.state == ResourceState::Free {
                        resource.state = ResourceState::Taken;
                    }
                }
            }
        }

        draw_terrain(TILE_SIZE, &tile_decorations_texture);
        for resource in &resources {
            draw_resource(&resource.position, TILE_SIZE, &gold_texture);
            if resource.state == ResourceState::Taken {
                draw_chest(&resource.position, TILE_SIZE, &chest_texture);
            }
        }
        draw_player(&player);

        draw_grid_lines(TILE_SIZE);

        next_frame().await
    }
}
