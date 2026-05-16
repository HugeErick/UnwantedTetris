use bevy::prelude::*;
use crate::utils::color_palette::ColorPalette;

#[derive(Component)]
pub struct GameOverRoot;

pub fn setup_game_over(mut commands: Commands) {
  let palette = ColorPalette::default();

  commands
    .spawn((
        GameOverRoot,
        Node {
          position_type: PositionType::Absolute,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          flex_direction: FlexDirection::Column,
          row_gap: Val::Px(24.0),
          ..default()
        },
        BackgroundColor(palette.dark_primary_color().with_alpha(0.92)),
        ZIndex(200),
    ))
    .with_children(|parent| {
      // "GAME OVER" — blended_magenta
      parent.spawn((
          Text::new("GAME OVER"),
          TextFont {
            font_size: 64.0,
            ..default()
          },
          TextColor(palette.blended_magenta_color()),
      ));

      // hint
      parent.spawn((
          Text::new("press R to restart"),
          TextFont {
            font_size: 18.0,
            ..default()
          },
          TextColor(palette.unwanted_gold_color().with_alpha(0.7)),
      ));
    });
}

pub fn game_over_input(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut next_state: ResMut<NextState<crate::game_instance::GameState>>,
) {
  if keyboard.just_pressed(KeyCode::KeyR) {
    // TODO: GameState::Playing does not restart an actual game had to go the menu fix this
    next_state.set(crate::game_instance::GameState::Menu);
  }
}

pub fn teardown_game_over(
  mut commands: Commands,
  query: Query<Entity, With<GameOverRoot>>,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn();
  }
}
