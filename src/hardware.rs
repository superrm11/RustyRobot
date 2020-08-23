use rppal::gpio::Level;
use rppal::gpio::Gpio;

use std::sync::mpsc::{channel, Sender};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

const SOFTPWM_PERIOD_US: u64 = 100;

/// Software PWM object
#[allow(dead_code)]
struct SoftPWM
{
  thread: JoinHandle<()>,
  setval_tx: Sender<u64>,
  end_tx: Sender<bool>,
}

/// Software PWM Implementation
#[allow(dead_code)]
impl SoftPWM
{
  ///
  /// Construct a software pwm pin. Emulates a hardware PWM by spawning a thread and quickly toggling a pin on and off.
  /// Inherently inaccurate, since execution is based on the OS's thread scheduling. Perfectly fine for motors and LED's, however.
  ///
  pub fn new(_gpio: &Gpio, _pin: u8, _resolution: u64) -> Self
  {
    let mut pin = _gpio.get(_pin).unwrap().into_output();
    let res = _resolution;
    let (setval_tx, setval_rx) = channel();
    let (end_tx, end_rx) = channel();

    // Spawn the thread, and save the join handle for later deconstruction.
    let thread = std::thread::spawn(move || {
      let mut setval: u64 = 0;

      loop {
        if let Ok(s) = setval_rx.try_recv() {
          setval = s;
        }

        // Break whenever the "end message" is sent to the thread
        if let Ok(e) = end_rx.try_recv() {
          if e {
            break;
          }
        }
        pin.write(Level::High);
        sleep(Duration::from_micros(setval * SOFTPWM_PERIOD_US));

        pin.write(Level::Low);
        sleep(Duration::from_micros((res - setval) * SOFTPWM_PERIOD_US));
      }
    });

    Self {
      thread,
      setval_tx,
      end_tx,
    }
  }

  /// Sets the current PWM value to _setval by sending it to the running thread.
  /// If this send fails, it results in a panic.
  pub fn set(&self, _setval: u64)
  {
    assert!(
      self.setval_tx.send(_setval).is_ok(),
      "Failed to send setpoint!"
    );
  }

  /// End the thread software PWM thread. This will render this SoftPWM useless, and free up system resources.
  /// If this send fails, it results in a panic.
  pub fn end(&self)
  {
    assert!(
      self.end_tx.send(true).is_ok(),
      "Failed to send 'join thread' message!"
    );
  }
}

/// Defines a standard L298-style motor controller, with a "Forward" pwm pin and a "Reverse" pin.
/// Mixing these results in bi-directional movement.
#[allow(dead_code)]
pub struct Motor
{
  fwd_pin: SoftPWM,
  rev_pin: SoftPWM,
}

#[allow(dead_code)]
impl Motor
{
  /// Construct a motor controller using software PWM on the specified pins.
  /// If the motor is spinning opposite from what is expected, either flip the physical pins
  /// or flip _fwdpin and _revpin
  pub fn new(_gpio: &Gpio, _fwdpin: u8, _revpin: u8) -> Self
  {
    let fwd_pin = SoftPWM::new(_gpio, _fwdpin, 255);
    let rev_pin = SoftPWM::new(_gpio, _revpin, 255);

    Self { fwd_pin, rev_pin }
  }

  /// Calculate the speed needed for the foward and reverse pins, and set the software pwm.
  pub fn set(&self, _speed: f32)
  {
    // Constrain speed between -1.0 and 1.0
    let speed_fixed = if _speed > 1.0 {
      1.0
    } else if _speed < -1.0 {
      -1.0
    } else {
      _speed
    };

    let out: u64 = (((speed_fixed + 1.0) / 2.0) * 255.0) as u64;
    self.fwd_pin.set(out);
    self.rev_pin.set(255 - out);
  }
}
