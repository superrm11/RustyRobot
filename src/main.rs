use gilrs::Axis;
use std::time::Duration;
use rppal::gpio::Gpio;


mod controller;
mod hardware;

use crate::controller::XboxController;

fn main() {

  // INITIALIZATION
  let mut con = XboxController::new();

  let gpio = Gpio::new().unwrap();
  let left_motor = hardware::Motor::new(&gpio, 1, 2);
  let right_motor = hardware::Motor::new(&gpio, 3, 4);

  loop {

    con.update();

    left_motor.set(con.get_axis(Axis::LeftStickY));
    right_motor.set(con.get_axis(Axis::RightStickY));

    std::thread::sleep(Duration::from_millis(10));
  }
}
