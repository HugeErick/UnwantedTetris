use crate::utils::color_palette::ColorPalette;
use bevy::prelude::*;

#[derive(Component)]
pub struct Board;

#[allow(dead_code)]
#[derive(Component)]
pub struct GridCell {
  pub x: i32,
  pub y: i32,
}

#[derive(Component)]
pub struct Occupied;

#[derive(Component)]
pub struct ResizeMessage;

pub fn setup_game_board(mut commands: Commands) {
  let color_palette = ColorPalette::default();

  // Main container (full screen)
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column, // Stack items vertically
        ..default()
      },
      ZIndex(20),
      BackgroundColor(color_palette.dark_primary_color()),
    ))
    .with_children(|parent| {
      // the "Window too small" Message (Hidden by default)
      parent.spawn((
        Text::new("Window too small to play"),
        TextFont {
          font_size: 20.0,
          ..default()
        },
        TextColor(color_palette.unwanted_gold_color()),
        Node {
          display: Display::None, // Hidden by default
          ..default()
        },
        ResizeMessage,
      ));

      // the Tetris Board
      parent.spawn((
        Node {
          display: Display::Grid, 
          width: Val::Px(308.0),  
          height: Val::Px(608.0), 
          border: UiRect::all(Val::Px(4.0)),
          padding: UiRect::all(Val::Px(0.0)),
          grid_template_columns: RepeatedGridTrack::px(10, 30.0),
          grid_template_rows: RepeatedGridTrack::px(20, 30.0),
          ..default()
        },
        ZIndex(40),
        BorderColor(color_palette.unwanted_gold_color()),
        BackgroundColor(color_palette.dark_secondary_color()),
        Board,
      ))
        .with_children(|grid| {
          for y in 0..20 {
            for x in 0..10 {
              grid.spawn((
                Node {
                  width: Val::Percent(100.0),
                  height: Val::Percent(100.0),
                  border: UiRect::all(Val::Px(1.0)),
                  ..default()
                },
                BorderColor(color_palette.blended_magenta_color().with_alpha(0.5)),
                GridCell { x, y },
                ZIndex(30),
              ));
            }
          }
        });
    });
}

pub fn check_window_size(
  windows: Query<&Window>, 
  mut board_query: Query<&mut Node, (With<Board>, Without<ResizeMessage>)>,
  mut message_query: Query<&mut Node, (With<ResizeMessage>, Without<Board>)>,
) {
  let Ok(window) = windows.single() else { return };
  let Ok(mut board_node) = board_query.single_mut() else { return };
  let Ok(mut message_node) = message_query.single_mut() else { return };

  let min_width = 350.0;
  let min_height = 650.0;

  if window.width() < min_width || window.height() < min_height {
    board_node.display = Display::None;
    message_node.display = Display::Flex; // Show message
  } else {
    board_node.display = Display::Grid;
    message_node.display = Display::None; // Hide message
  }
}
