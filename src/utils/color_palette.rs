use bevy::prelude::*;

#[allow(dead_code)]
pub struct ColorPalette {
  pub dark_primary: Srgba,
  pub dark_secondary: Srgba,
  pub unwanted_gold: Srgba,
  pub blended_magenta: Srgba,
}

impl Default for ColorPalette {
  fn default() -> Self {
    Self {

      // #151515
      dark_primary: Srgba {
        red: 21.0 / 255.0,
        green: 21.0 / 255.0,
        blue: 21.0 / 255.0,
        alpha: 1.0,
      },

      // #232326
      dark_secondary: Srgba {
        red: 35.0 / 255.0,
        green: 35.0 / 255.0,
        blue: 38.0 / 255.0,
        alpha: 1.0,
      },

      // #FAD643
      unwanted_gold: Srgba {
        red: 250.0 / 255.0,
        green: 214.0 / 255.0,
        blue: 67.0 / 255.0,
        alpha: 1.0,
      },

      // #FF0087
      blended_magenta: Srgba {
        red: 1.0,
        green: 0.0 / 255.0,
        blue: 135.0 / 255.0,
        alpha: 1.0,
      }
    }
  }
}

#[allow(dead_code)]
impl ColorPalette {
  pub fn dark_primary_color(&self) -> Color {
    Color::srgba(
      self.dark_primary.red,
      self.dark_primary.green,
      self.dark_primary.blue,
      self.dark_primary.alpha
    )
  }
  pub fn dark_secondary_color(&self) -> Color {
    Color::srgba(
      self.dark_secondary.red,
      self.dark_secondary.green,
      self.dark_secondary.blue,
      self.dark_secondary.alpha
    )
  }
  pub fn unwanted_gold_color(&self) -> Color {
    Color::srgba(
      self.unwanted_gold.red,
      self.unwanted_gold.green,
      self.unwanted_gold.blue,
      self.unwanted_gold.alpha
    )
  }

  pub fn blended_magenta_color(&self) -> Color {
    Color::srgba(
      self.blended_magenta.red,
      self.blended_magenta.green,
      self.blended_magenta.blue,
      self.blended_magenta.alpha
    )
  }
}
