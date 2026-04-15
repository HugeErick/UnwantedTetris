use bevy::prelude::*;
use rand::prelude::*;

use crate::systems::game_board::{Board, GridCell, Occupied, CELL_SIZE};
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
  pub rotation_index: usize,
  pub color: Color,
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

  pub fn get_rotated_shape(&self, rotation: usize) -> [(i32, i32); 4] {
    let mut shape = self.get_shape();

    if *self == TetrominoType::O { return shape; }
    let steps = rotation % 4;
    for _ in 0..steps {
      for i in 0..4 {
        let (x, y) = shape[i];
        shape[i] = (y, -x);
      }
    }
    shape
  }

  pub fn get_wall_kick_offset(&self) -> Vec<IVec2> {
    match self {
      TetrominoType::I => vec![
        IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0),
        IVec2::new(-2, 0), IVec2::new(2, 0)
      ],
      _ => vec![
        IVec2::new(0, 0),  // original position
        IVec2::new(-1, 0), // kick left
        IVec2::new(1, 0),  // kick right
        IVec2::new(0, -1), // kick up (floor kick)
        IVec2::new(-1, -1),
        IVec2::new(1, -1),
      ],
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

  let colors = [
    Color::srgb(0.0, 1.0, 1.0), Color::srgb(0.0, 0.0, 1.0),
    Color::srgb(1.0, 0.5, 0.0), Color::srgb(1.0, 1.0, 0.0),
    Color::srgb(0.0, 1.0, 0.0), Color::srgb(0.5, 0.0, 0.5),
  ];

  let mut rng = rand::rng();
  let piece_type = types[rng.random_range(0..types.len())];
  let color = colors[rng.random_range(0..colors.len())];

  commands.entity(board_entity).with_children(|parent| {
    parent.spawn((
      ActivePiece {
        piece_type,
        position: IVec2::new(4, 0),
        rotation_index: 0,
        color,
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
              width: Val::Px(CELL_SIZE),
              height: Val::Px(CELL_SIZE),
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
    let shape = piece.piece_type.get_rotated_shape(piece.rotation_index);

    for (child, offset) in children.iter().zip(shape.iter()) {
      if let Ok(mut node) = btarget_query.get_mut(child) {
        let final_x = (piece.position.x + offset.0) as f32 * CELL_SIZE;
        let final_y = (piece.position.y + offset.1) as f32 * CELL_SIZE;

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

    // handle rotation
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
      let next_rotation = (piece.rotation_index + 1) % 4;
      let kicks = piece.piece_type.get_wall_kick_offset();
      
      for offset in kicks {
        let test_pos = piece.position + offset;
        if is_valid_move(piece.position, piece.piece_type, next_rotation, &occupied_cells) {
          piece.position = test_pos;
          piece.rotation_index = next_rotation;
          break;
        }
      }
    }

    // handle movement
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
      if is_valid_move(new_pos, piece.piece_type, piece.rotation_index, &occupied_cells) {
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
  rotation: usize,
  occupied_cells: &Query<&GridCell, With<Occupied>>
) -> bool {
  let shape = piece_type.get_rotated_shape(rotation);
  for offset in shape {
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
  mut cell_query: Query<(Entity, &GridCell, &mut BackgroundColor, Option<&Occupied>)>,
  occupied_cells: Query<&GridCell, With<Occupied>>,
) {
  game_speed.timer.tick(time.delta());

  if game_speed.timer.just_finished() && let Ok((entity, mut piece)) = query.single_mut() {

    let next_pos = piece.position + IVec2::new(0, 1);

    if is_valid_move(next_pos, piece.piece_type, piece.rotation_index, &occupied_cells) {
      piece.position = next_pos;
    } else {
      let color = piece.color;
      let shape = piece.piece_type.get_rotated_shape(piece.rotation_index);
      let pos = piece.position;

      for offset in shape {
        let target_x = pos.x + offset.0;
        let target_y = pos.y + offset.1;

        for (cell_entity, cell, mut bg, _) in cell_query.iter_mut() {
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

pub fn clear_lines(
  mut commands: Commands,
  mut cell_query: Query<(Entity, &GridCell, &mut BackgroundColor, Option<&Occupied>)>
) {
  let color_palette = crate::utils::color_palette::ColorPalette::default();
  let mut row_to_clear = None;

  for y in (0..20).rev() {
    let mut count = 0;
    for (_, cell, _, occupied) in cell_query.iter() {
      if cell.y == y && occupied.is_some() {
        count += 1;
      }
    }
    if count == 10 {
      row_to_clear = Some(y);
      break;
    }
  }

  if let Some(row) = row_to_clear {
    // clear completed row
    for (entity, cell, mut bg, occupied) in cell_query.iter_mut() {
      if cell.y == row && occupied.is_some() {
        commands.entity(entity).remove::<Occupied>();
        bg.0 = color_palette.dark_secondary_color();
      }
    }

    // identify the blocks above that need to be shifted
    let mut shifts = Vec::new();
    for (_, cell, bg, occupied) in cell_query.iter() {
      if cell.y < row && occupied.is_some() {
        shifts.push((cell.x, cell.y, bg.0));
      }
    }


    // apply the shifts
    for (x, y , color) in shifts {
      for (entity, cell, mut bg, _) in cell_query.iter_mut() {
        if cell.x == x && cell.y == y {
          commands.entity(entity).remove::<Occupied>();
          bg.0 = color_palette.dark_secondary_color();
        }
      }

      for (entity, cell, mut bg, _) in cell_query.iter_mut() {
        if cell.x == x && cell.y == y + 1 {
          commands.entity(entity).insert(Occupied);
          bg.0 = color;
        }
      }
    }
  }
}
