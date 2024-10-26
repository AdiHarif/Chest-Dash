mod ai;
mod draw;
mod player;
mod player_sprite;
mod texture_manager;
mod ui;

use draw::*;
use player::*;
use player_sprite::get_player_sprite;
use texture_manager::TextureManager;
use ui::*;

use macroquad::{
    audio::*,
    prelude::*,
    ui::{root_ui, Skin},
};

#[derive(PartialEq)]
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum ResourceState {
    Free,
    TakenByPlayer,
    TakenByEnemy,
}

struct Resource {
    position: GridPosition,
    state: ResourceState,
}

fn get_x_offset() -> f32 {
    if screen_height() < screen_width() {
        return (screen_width() - GRID_COLS_COUNT as f32 * get_tile_size()) / 2.0;
    }
    0.0
}

fn is_mouse_over_resource(resource_position: &GridPosition, resource_size: f32) -> bool {
    let mouse_vector = Vec2 {
        x: mouse_position().0 - get_x_offset(),
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

const RESOURCES_ROWS_COUNT: u32 = 3;
const GRID_ROWS_COUNT: u32 = 15;
const GRID_COLS_COUNT: u32 = 15;
const PLAYER_REACH_DISTANCE: f32 = 2.5; // tiles
const PLAYER_SPEED: f32 = 3.0; // tiles per second
const WIN_CONDITION: f32 = 50.0;
const BGM_VOLUME: f32 = 0.25;

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

fn update_score(player: &mut Player, enemy: &mut Player, resources: &Vec<Resource>) {
    let player_chests = resources
        .iter()
        .filter(|r| r.state == ResourceState::TakenByPlayer)
        .count();

    let enemy_chests = resources
        .iter()
        .filter(|r| r.state == ResourceState::TakenByEnemy)
        .count();

    let frame_duration = get_frame_time();
    player.score += player_chests as f32 * frame_duration;
    enemy.score += enemy_chests as f32 * frame_duration;
}

enum GameStatus {
    Starting,
    Running,
    GameOver,
}

fn get_tile_size() -> f32 {
    f32::min(
        screen_width() / GRID_COLS_COUNT as f32,
        screen_height() / GRID_ROWS_COUNT as f32,
    )
}

#[macroquad::main("MyGame")]
async fn main() {
    draw_loading_screen();
    next_frame().await;

    let mut texture_manager = TextureManager::new();
    texture_manager.load_all_textures().await;

    initialize_ui();

    let success_sound = load_sound("assets/sfx/success.wav").await.unwrap();
    let failure_sound = load_sound("assets/sfx/failure.wav").await.unwrap();
    let win_sound = load_sound("assets/sfx/win.wav").await.unwrap();
    let lose_sound = load_sound("assets/sfx/lose.wav").await.unwrap();
    let bgm = load_sound("assets/sfx/bgm.ogg").await.unwrap();

    play_sound(
        &bgm,
        PlaySoundParams {
            volume: BGM_VOLUME,
            looped: true,
        },
    );
    let mut end_sound_flag = false;

    let player_sprite = get_player_sprite();
    let player_starting_position = vec2(1.5, GRID_COLS_COUNT as f32 / 2.0);
    let mut player = Player::new(
        player_starting_position,
        texture_manager.get("player").unwrap().clone(),
        player_sprite,
        PLAYER_SPEED,
        PLAYER_REACH_DISTANCE,
    );

    let enemy_sprite = get_player_sprite();
    let enemy_starting_position = vec2(
        GRID_COLS_COUNT as f32 - player_starting_position.x,
        player_starting_position.y,
    );
    let mut enemy = Player::new(
        enemy_starting_position,
        texture_manager.get("enemy").unwrap().clone(),
        enemy_sprite,
        PLAYER_SPEED / 2.0,
        PLAYER_REACH_DISTANCE / 2.0,
    );

    let mut resources = initalize_resources(GRID_ROWS_COUNT, GRID_COLS_COUNT);

    let mut status = GameStatus::Starting;

    loop {
        clear_background(Color::from_hex(0x9d7658));
        show_hud(player.score, enemy.score);
        match status {
            GameStatus::Starting => {
                let clicked = show_start_button();
                if clicked {
                    status = GameStatus::Running;
                }
            }
            GameStatus::Running => {
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
                if (direction.x != 0.0 || direction.y != 0.0) {
                    direction = direction.normalize();
                }

                player.update(&direction);
                ai::update_enemy(&mut resources, &mut enemy);

                let tile_size = get_tile_size();
                if is_mouse_button_released(MouseButton::Left) {
                    let mut success = false;
                    for resource in &mut resources {
                        if is_mouse_over_resource(&resource.position, tile_size) {
                            if ResourceState::TakenByPlayer == resource.state {
                                continue;
                            }

                            let resource_screen_position = vec2(
                                (resource.position.x as f32 + 0.5) * tile_size,
                                (resource.position.y as f32 + 0.5) * tile_size,
                            );
                            let distance =
                                (resource_screen_position - (player.position * tile_size)).length();

                            if distance < player.reach * tile_size {
                                resource.state = ResourceState::TakenByPlayer;
                                success = true;
                                break;
                            }
                        }
                    }
                    if success {
                        play_sound_once(&success_sound);
                    } else {
                        play_sound_once(&failure_sound);
                    }
                }

                update_score(&mut player, &mut enemy, &resources);

                if player.score >= WIN_CONDITION || enemy.score >= WIN_CONDITION {
                    status = GameStatus::GameOver;
                    end_sound_flag = false;
                }

                draw_frame(&player, &enemy, &resources, &texture_manager);
            }
            GameStatus::GameOver => {
                draw_frame(&player, &enemy, &resources, &texture_manager);
                let player_won = player.score >= WIN_CONDITION;
                let winner_label = if player_won {
                    "Player wins!"
                } else {
                    "Enemy wins!"
                };
                if !end_sound_flag {
                    stop_sound(&bgm);
                    if player_won {
                        play_sound_once(&win_sound);
                    } else {
                        play_sound_once(&lose_sound);
                    }
                    end_sound_flag = true;
                }
                let label = "Game Over, ".to_owned() + winner_label;
                let clicked = show_game_over_button(&label);
                if clicked {
                    play_sound(
                        &bgm,
                        PlaySoundParams {
                            volume: BGM_VOLUME,
                            looped: true,
                        },
                    );
                    player.position = player_starting_position;
                    player.score = 0.0;
                    enemy.position = enemy_starting_position;
                    enemy.score = 0.0;
                    resources = initalize_resources(GRID_ROWS_COUNT, GRID_COLS_COUNT);
                    status = GameStatus::Running;
                }
            }
        }
        draw_bounding_box();
        next_frame().await
    }
}
