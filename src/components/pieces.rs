use bevy::prelude::*;
use rand::prelude::*;

use crate::systems::game_board::{Board, GridCell, Occupied};
use crate::utils::color_palette::ColorPalette;

#[derive(Resource)]
pub struct GameSpeed {
  pub timer: Timer,
  pub move_timer: Timer,
}

impl Default for GameSpeed {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(0.8, TimerMode::Repeating), 

      move_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TetrominoType {
  I, J, L, O, S, T, Z,
}

#[derive(Component)]
pub struct ActivePiece {
  pub piece_type: TetrominoType,
  pub position: IVec2,
  pub _rotation: usize,
}

#[derive(Component)]
pub struct PieceBtarget; // tag for the 4 entities that make up the visual piece

impl TetrominoType {
  // returns the relative (x, y) coordinates for each btarget of the piece.
  pub fn get_shape(&self) -> [(i32, i32); 4] {
    match self {
      // defined relative to a center point
      TetrominoType::I => [(0, -1), (0, 0), (0, 1), (0, 2)],
      TetrominoType::J => [(-1, 1), (0, 1), (0, 0), (0, -1)],
      TetrominoType::L => [(1, 1), (0, 1), (0, 0), (0, -1)],
      TetrominoType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
      TetrominoType::S => [(1, 0), (0, 0), (0, -1), (-1, -1)],
      TetrominoType::Z => [(-1, 0), (0, 0), (0, -1), (1, -1)],
      TetrominoType::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
    }
  }

  pub fn get_color(&self) -> Color {
    match self {
      TetrominoType::I => Color::srgb(0.0, 1.0, 1.0), // Cyan
      TetrominoType::J => Color::srgb(0.0, 0.0, 1.0), // Blue
      TetrominoType::L => Color::srgb(1.0, 0.5, 0.0), // Orange
      TetrominoType::O => Color::srgb(1.0, 1.0, 0.0), // Yellow
      TetrominoType::S => Color::srgb(0.0, 1.0, 0.0), // Green
      TetrominoType::T => Color::srgb(0.5, 0.0, 0.5), // Purple
      TetrominoType::Z => Color::srgb(1.0, 0.0, 0.0), // Red
    }
  }
}

pub fn spawn_piece(mut commands: Commands, board_query: Query<Entity, With<Board>>) {
  let Ok(board_entity) = board_query.single() else { return };
  let color_palette = ColorPalette::default();

  let types = [
    TetrominoType::I, TetrominoType::J, TetrominoType::L,
    TetrominoType::O, TetrominoType::S, TetrominoType::T, TetrominoType::Z,
  ];

  let mut rng = rand::rng();
  let piece_type = types[rng.random_range(0..types.len())];
  let color = piece_type.get_color();

  commands.entity(board_entity).with_children(|parent| {
    parent.spawn((
      ActivePiece {
        piece_type,
        position: IVec2::new(4, 0),
        _rotation: 0,
      },
      Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
      },
      ZIndex(50),
    ))
      .with_children(|piece_node| {
        for _ in 0..4 {
          piece_node.spawn((
            Node {
              position_type: PositionType::Absolute,
              width: Val::Px(30.0),
              height: Val::Px(30.0),
              border: UiRect::all(Val::Px(2.0)),
              ..default()
            },
            BackgroundColor(color),
            BorderColor(color_palette.dark_primary_color().with_alpha(0.3)),
            PieceBtarget,
          ));
        }
      });
  });
}

pub fn update_piece_visuals(
  active_piece_query: Query<(&ActivePiece, &Children), Changed<ActivePiece>>,
  mut btarget_query: Query<&mut Node, With<PieceBtarget>>,
) {
  for (piece, children) in active_piece_query.iter() {
    let shape = piece.piece_type.get_shape();

    for (child, offset) in children.iter().zip(shape.iter()) {
      if let Ok(mut node) = btarget_query.get_mut(child) {
        let final_x = (piece.position.x + offset.0) as f32 * 30.0;
        let final_y = (piece.position.y + offset.1) as f32 * 30.0;

        node.left = Val::Px(final_x);
        node.top = Val::Px(final_y);
      }
    }
  }
}


pub fn move_piece(
  time: Res<Time>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut game_speed: ResMut<GameSpeed>,
  mut query: Query<&mut ActivePiece>,
  occupied_cells: Query<&GridCell, With<Occupied>>
) {
  // 1. Tick the timer
  game_speed.move_timer.tick(time.delta());

  if let Ok(mut piece) = query.single_mut() {
    let mut displacement = IVec2::ZERO;

    // check for "Instant" presses first
    let just_left = keyboard_input.just_pressed(KeyCode::ArrowLeft);
    let just_right = keyboard_input.just_pressed(KeyCode::ArrowRight);
    let just_down = keyboard_input.just_pressed(KeyCode::ArrowDown);

    // check for "Held" repeat
    let hold_left = keyboard_input.pressed(KeyCode::ArrowLeft) && game_speed.move_timer.just_finished();
    let hold_right = keyboard_input.pressed(KeyCode::ArrowRight) && game_speed.move_timer.just_finished();
    let hold_down = keyboard_input.pressed(KeyCode::ArrowDown) && game_speed.move_timer.just_finished();

    if just_left || hold_left { displacement.x -= 1; }
    if just_right || hold_right { displacement.x += 1; }
    if just_down || hold_down { displacement.y += 1; }

    if displacement != IVec2::ZERO {
      let new_pos = piece.position + displacement;
      if is_valid_move(new_pos, piece.piece_type, &occupied_cells) {
        piece.position = new_pos;

        if just_left || just_right || just_down {
          game_speed.move_timer.reset();
        }
      }
    }
  }
}

fn is_valid_move(
  pos: IVec2,
  piece_type: TetrominoType,
  occupied_cells: &Query<&GridCell, With<Occupied>>
) -> bool {
  for offset in piece_type.get_shape() {
    let x = pos.x + offset.0;
    let y = pos.y + offset.1;

    // boundary check
    if !(0..10).contains(&x) || !(0..20).contains(&y) {
      return false;
    }

    // occupied check
    for cell in occupied_cells.iter() {
      if cell.x == x && cell.y == y {
        return false;
      }
    }
  }
  true

}

pub fn apply_gravity(
  mut commands: Commands,
  time: Res<Time>,
  mut game_speed: ResMut<GameSpeed>,
  mut query: Query<(Entity, &mut ActivePiece)>,
  board_query: Query<Entity, With<Board>>,
  mut cell_query: Query<(Entity, &GridCell, &mut BackgroundColor)>,
  occupied_cells: Query<&GridCell, With<Occupied>>,
) {
  game_speed.timer.tick(time.delta());

  if game_speed.timer.just_finished() && let Ok((entity, mut piece)) = query.single_mut() {

    let next_pos = piece.position + IVec2::new(0, 1);

    if is_valid_move(next_pos, piece.piece_type, &occupied_cells) {
      piece.position = next_pos;
    } else {
      let color = piece.piece_type.get_color();
      let shape = piece.piece_type.get_shape();
      let pos = piece.position;

      for offset in shape {
        let target_x = pos.x + offset.0;
        let target_y = pos.y + offset.1;

        for (cell_entity, cell, mut bg) in cell_query.iter_mut() {
          if cell.x == target_x && cell.y == target_y {
            commands.entity(cell_entity).insert(Occupied);
            bg.0 = color;            }
        }
      }
      commands.entity(entity).despawn();
      spawn_piece(commands, board_query);
      info!("Piece targeted, spawning new one");
    }

  }
}

