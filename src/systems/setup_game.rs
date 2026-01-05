use crate::systems::game_board::setup_game_board;
use bevy::prelude::*;


pub fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Camera spawned");
    setup_game_board(commands);
}

