use bevy::prelude::*; 
use crate::systems::setup_game::setup_game;
use crate::systems::game_board::check_window_size;
use crate::components::pieces::{
  spawn_piece, update_piece_visuals, move_piece,
  apply_gravity, clear_lines,
  GameSpeed
};

pub struct GameInstance;

impl Plugin for GameInstance {
  fn build(&self, app: &mut App) {
    app.add_plugins(DefaultPlugins.set(
      WindowPlugin {
        primary_window: Some(Window {
          title: "Tetris".to_string(),
          ..default()
        }),
        ..default()
      }
    ));

    // initialice resources
    app.init_resource::<GameSpeed>();

    app.add_systems(Startup, (setup_game, spawn_piece).chain());

    app.add_systems(Update, (
      check_window_size,
      move_piece,
      apply_gravity,
      clear_lines,
      update_piece_visuals
    ));
  }
}

