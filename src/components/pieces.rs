use bevy::prelude::*;
use rand::prelude::*;

use crate::systems::game_board::Board;
use crate::utils::color_palette::ColorPalette;

#[derive(Resource)]
pub struct GameSpeed {
  pub timer: Timer,
}

impl Default for GameSpeed {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(0.8, TimerMode::Repeating), 
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
pub struct PieceBlock; // tag for the 4 entities that make up the visual piece

impl TetrominoType {
  // returns the relative (x, y) coordinates for each block of the piece.
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
            PieceBlock,
          ));
        }
      });
  });
}

pub fn update_piece_visuals(
  active_piece_query: Query<(&ActivePiece, &Children), Changed<ActivePiece>>,
  mut block_query: Query<&mut Node, With<PieceBlock>>,
) {
  for (piece, children) in active_piece_query.iter() {
    let shape = piece.piece_type.get_shape();

    for (child, offset) in children.iter().zip(shape.iter()) {
      if let Ok(mut node) = block_query.get_mut(child) {
        let final_x = (piece.position.x + offset.0) as f32 * 30.0;
        let final_y = (piece.position.y + offset.1) as f32 * 30.0;

        node.left = Val::Px(final_x);
        node.top = Val::Px(final_y);
      }
    }
  }
}


pub fn move_piece(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut query: Query<&mut ActivePiece>,
) {
  for mut piece in query.iter_mut() {
    let mut displacement = IVec2::ZERO;

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
      displacement.x -= 1;
      info!("arrow left pressed!");
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
      displacement.x += 1;
      info!("arrow right pressed!");
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
      displacement.y += 1;
      info!("arrow down pressed!");
    }

    if displacement != IVec2::ZERO {
      let new_position = piece.position + displacement;

      // Basic Boundary Check
      if is_within_bounds(new_position, piece.piece_type) {
        piece.position = new_position;
      }
    }
  }
}

fn is_within_bounds(pos: IVec2, piece_type: TetrominoType) -> bool {
  for offset in piece_type.get_shape() {
    let x = pos.x + offset.0;
    let y = pos.y + offset.1;

    if !(0..10).contains(&x) || !(0..20).contains(&y) {
      return false;
    }
  }
  true
}

pub fn apply_gravity(
  time: Res<Time>,
  mut game_speed: ResMut<GameSpeed>,
  mut query: Query<&mut ActivePiece>,
) {
  game_speed.timer.tick(time.delta());

  if game_speed.timer.just_finished() {
    for mut piece in query.iter_mut() {
      let next_pos = piece.position +IVec2::new(0, 1);
      if is_within_bounds(next_pos, piece.piece_type) {
        piece.position = next_pos;
      } else {
        // for now stops, later we lock the piece up
        info!("Piece hit bottom");
      }
    }
  }
}

