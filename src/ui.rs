use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

pub fn initialize_ui() {
    let style = root_ui().style_builder().font_size(50).color(WHITE).build();
    let default_skin = root_ui().default_skin().clone();
    root_ui().push_skin(&Skin {
        button_style: style.clone(),
        label_style: style,
        ..default_skin
    });
}

pub fn show_start_button() -> bool {
    let dimensions = root_ui().calc_size("Start");
    return root_ui().button(
        Vec2 {
            x: (screen_width() - dimensions.x) / 2.0,
            y: (screen_height() - dimensions.y) / 2.0,
        },
        "Start",
    );
}

pub fn show_game_over_button(label: &String) -> bool {
    let label_dimensions = root_ui().calc_size(&label);
    let label_position = Vec2 {
        x: (screen_width() - label_dimensions.x) / 2.0,
        y: (screen_height() / 2.0) - label_dimensions.y,
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
        x: (screen_width() - button_dimensions.x) / 2.0,
        y: (screen_height() / 2.0),
    };

    root_ui().button(button_position, button_label)
}
