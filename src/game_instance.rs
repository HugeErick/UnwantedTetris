use bevy::prelude::*; 
use crate::systems::setup_camera::setup_camera;
use crate::systems::game_board::check_window_size;
use crate::systems::menu::{setup_menu, menu_button_interaction, teardown_menu};
use crate::components::pieces::{
  spawn_piece, update_piece_visuals, move_piece,
  apply_gravity, clear_lines,
  GameSpeed
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
  #[default]
  Menu,
  Playing,
}

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
    app.init_state::<GameState>();
    app.init_resource::<GameSpeed>();

    // start systems
    app.add_systems(Startup, setup_camera);
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_systems(
      Update,
      menu_button_interaction.run_if(in_state(GameState::Menu)),
    );
    app.add_systems(OnExit(GameState::Menu), teardown_menu);

    // playing state
    app.add_systems(
      OnEnter(GameState::Playing),
      (setup_game_board_entry, spawn_piece).chain(),
    );
    app.add_systems(Update, (
      check_window_size,
      move_piece,
      apply_gravity,
      clear_lines,
      update_piece_visuals
    )
      .run_if(in_state(GameState::Playing)),
    );
  }
}

fn setup_game_board_entry(commands: Commands) {
  crate::systems::game_board::setup_game_board(commands);
}

