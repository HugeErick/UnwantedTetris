use bevy::prelude::*;
use crate::game_instance::GameState;
use crate::utils::color_palette::ColorPalette;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct PlayButton;

pub fn setup_menu(mut commands: Commands) {
  let palette = ColorPalette::default();
  const TITLE_FONT_SIZE: f32 = 64.0;

  commands
    .spawn((
        MenuRoot,
        Node {
          position_type: PositionType::Absolute,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          flex_direction: FlexDirection::Column,
          row_gap: Val::Px(32.0),
          ..default()
        },
        BackgroundColor(palette.dark_primary_color()),
        ZIndex(100),
    ))
    .with_children(|parent| {

      // Title
      parent.spawn((
          Text::default(),
          TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
          },
      ))
      .with_children(|title| {
        title.spawn((
            TextSpan::new("Unwanted"),
            TextFont {
              font_size: TITLE_FONT_SIZE,
              ..default()
            },
            TextColor(palette.dark_secondary_color()),
        ));

        title.spawn((
            TextSpan::new("TETRIS"),
            TextFont {
              font_size: TITLE_FONT_SIZE,
              ..default()
            },
            TextColor(palette.unwanted_gold_color()),
        ));
      });

      // Subtitle / flavour
      parent.spawn((
          Text::new("stack. clear. survive."),
          TextFont {
            font_size: 16.0,
            ..default()
          },
          TextColor(palette.blended_magenta_color()),
      ));

      // Play button
      parent
        .spawn((
            PlayButton,
            Button,
            Node {
              padding: UiRect::axes(Val::Px(48.0), Val::Px(14.0)),
              border: UiRect::all(Val::Px(2.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
            },
            BorderColor(palette.unwanted_gold_color()),
            BackgroundColor(palette.dark_secondary_color()),
        ))
        .with_children(|btn| {
          btn.spawn((
              Text::new("PLAY"),
              TextFont {
                font_size: 24.0,
                ..default()
              },
              TextColor(palette.unwanted_gold_color()),
          ));
        });
    });
}

pub fn menu_button_interaction(
  mut next_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor),
  (Changed<Interaction>, With<PlayButton>),
  >,
) {
  let palette = ColorPalette::default();

  for (interaction, mut bg, mut border) in interaction_query.iter_mut() {
    match interaction {
      Interaction::Pressed => {
        next_state.set(GameState::Playing);
      }
      Interaction::Hovered => {
        bg.0 = palette.unwanted_gold_color().with_alpha(0.15);
        border.0 = palette.unwanted_gold_color();
      }
      Interaction::None => {
        bg.0 = palette.dark_secondary_color();
        border.0 = palette.unwanted_gold_color();
      }
    }
  }
}

pub fn teardown_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuRoot>>) {
  for entity in menu_query.iter() {
    commands.entity(entity).despawn();
  }
}

