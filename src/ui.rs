use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

use crate::{get_tile_size, GRID_COLS_COUNT, GRID_ROWS_COUNT};

pub fn initialize_ui() {
    let style = root_ui().style_builder().font_size(40).color(WHITE).build();
    let default_skin = root_ui().default_skin().clone();
    root_ui().push_skin(&Skin {
        button_style: style.clone(),
        label_style: style,
        ..default_skin
    });
}

pub fn show_start_button() -> bool {
    let dimensions = root_ui().calc_size("Start");
    let tile_size = get_tile_size();
    return root_ui().button(
        Vec2 {
            x: (GRID_COLS_COUNT as f32 * tile_size - dimensions.x) / 2.0,
            y: (GRID_ROWS_COUNT as f32 * tile_size - dimensions.y) / 2.0,
        },
        "Start",
    );
}

pub fn show_game_over_button(label: &String) -> bool {
    let label_dimensions = root_ui().calc_size(&label);
    let tile_size = get_tile_size();
    let box_width = GRID_COLS_COUNT as f32 * tile_size;
    let box_height = GRID_ROWS_COUNT as f32 * tile_size;
    let label_position = Vec2 {
        x: (box_width - label_dimensions.x) / 2.0,
        y: (box_height / 2.0) - label_dimensions.y,
    };
    draw_rectangle(
        label_position.x,
        label_position.y,
        label_dimensions.x,
        label_dimensions.y,
        WHITE,
    );
    root_ui().label(label_position, &label);
    let button_label = "Click here to restart";
    let button_dimensions = root_ui().calc_size(&button_label);
    let button_position = Vec2 {
        x: (box_width - button_dimensions.x) / 2.0,
        y: (box_height / 2.0),
    };

    root_ui().button(button_position, button_label)
}

pub fn show_hud(player_score: f32, enemy_score: f32) {
    let player_text = format!("Player: {:>6.2}", player_score);
    let position = Vec2 { x: 10.0, y: 10.0 };
    root_ui().label(position, &player_text);

    let enemy_text = format!("Enemy: {:>6.2}", enemy_score);
    let dimensions = root_ui().calc_size(&player_text);
    let position = Vec2 {
        x: GRID_COLS_COUNT as f32 * get_tile_size() - dimensions.x,
        y: 10.0,
    };
    root_ui().label(position, &enemy_text);

    let fps_label = &format!("FPS: {}", get_fps());
    let dimensions = root_ui().calc_size(fps_label);
    let position = Vec2 {
        x: 10.0,
        y: (GRID_ROWS_COUNT as f32 * get_tile_size() - dimensions.y),
    };
    root_ui().label(position, fps_label);
}
