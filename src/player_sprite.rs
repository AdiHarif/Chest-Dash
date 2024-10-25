use crate::TILE_SIZE;
use macroquad::experimental::animation::*;

const PLAYER_FRAME_SIZE: u32 = 48;
pub const PLAYER_DEST_SIZE: f32 = TILE_SIZE * 3.0;
const PLAYER_FPS: u32 = 12;

pub fn get_player_sprite() -> AnimatedSprite {
    return AnimatedSprite::new(
        PLAYER_FRAME_SIZE,
        PLAYER_FRAME_SIZE,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 6,
                fps: PLAYER_FPS,
            },
            Animation {
                name: "walk_down".to_string(),
                row: 3,
                frames: 6,
                fps: PLAYER_FPS,
            },
            Animation {
                name: "walk_left".to_string(),
                row: 4,
                frames: 6,
                fps: PLAYER_FPS,
            },
            Animation {
                name: "walk_up".to_string(),
                row: 5,
                frames: 6,
                fps: PLAYER_FPS,
            },
        ],
        true,
    );
}
