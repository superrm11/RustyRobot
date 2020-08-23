use gilrs::{Axis, Button, GamepadId, Gilrs};

/// A wrapper around GILRS library, designed exclusively for an Xbox controller.
/// This only provides a simple implementation for getting button and axis information.
pub struct XboxController {
  gilrs: Gilrs,
  id: Option<GamepadId>,
}

#[allow(dead_code)]
impl XboxController {
  /// Construct a new xbox controller, grabbing the first it one it finds from the OS.
  pub fn new() -> Self {
    let gilrs_tmp = Gilrs::new().unwrap();
    let mut id_name: Option<GamepadId> = None;

    for (_id, _gp) in gilrs_tmp.gamepads() {
      if _gp.name().to_lowercase().contains("xbox") {
        id_name = Some(_id);
      }
    }

    Self {
      gilrs: gilrs_tmp,
      id: id_name,
    }
  }

  /// Grabs whether a single button is pressed.
  /// Will only work if update() has been called beforehand to cache the data.
  pub fn get_button(&self, btn: Button) -> bool {
    let mut out = false;

    if let Some(i) = self.id {
      if let Some(xbox_con) = self.gilrs.connected_gamepad(i) {
        out = xbox_con.is_pressed(btn)
      }
    }

    out
  }

  /// Grabs an axis from the controller Returns between -1.0 and 1.0.
  /// Will only work if update() has been called beforehand to cache the data.
  pub fn get_axis(&self, axis: Axis) -> f32 {
    let mut out: f32 = 0.0;

    if let Some(i) = self.id {
      if let Some(xbox_con) = self.gilrs.connected_gamepad(i) {
        out = xbox_con.value(axis);
      }
    }

    out
  }

  /// Update the GILRS cache by fetching the new values from the controller.
  /// Must be run before accessing current data.
  pub fn update(&mut self) {
    while let Some(e) = self.gilrs.next_event() {
      self.gilrs.update(&e);
    }
  }
}
