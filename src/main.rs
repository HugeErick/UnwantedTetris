use bevy::prelude::*;
mod game_instance;
mod systems;
mod utils;
mod components;
use game_instance::GameInstance;

fn main() {
  App::new().add_plugins(GameInstance).run();
}
