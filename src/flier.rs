use piston_window::*;
use std::f64::consts::PI;

pub struct Flier {
  pos_x: f64,
  pos_y: f64,
  rotation: f64,
  color: [f32; 4],
  steering_left: bool,
  steering_right: bool,
  thrusting: bool
}

impl Flier {
  const STEER_RATIO: f64 = 100.0; // The bigger the slower
  const VELOCITY: f64 = 1.0;

  pub fn new(pos_x: f64, pos_y: f64, color: [f32; 4]) -> Flier {
    Flier { 
      pos_x, pos_y, rotation: 0.0, color,
      steering_left: false,
      steering_right: false,
      thrusting: false
    }
  }

  pub fn on_input(&mut self, button: Button, state: ButtonState) {
    let pressed = if state == ButtonState::Press { true } else { false };
    match button {
      Button::Keyboard(Key::Up) => {
        self.thrusting = pressed;
      }
      Button::Keyboard(Key::Left) => {
        self.steering_left = pressed;
      }
      Button::Keyboard(Key::Right) => {
        self.steering_right = pressed;
      }
      _ => {}
    }
  }

  pub fn mover(&mut self) {
    if self.steering_left { self.rotation -= PI / Flier::STEER_RATIO }
    if self.steering_right { self.rotation += PI / Flier::STEER_RATIO }
    if self.thrusting {
      self.pos_x += self.rotation.sin() * Flier::VELOCITY;
      self.pos_y -= self.rotation.cos() * Flier::VELOCITY;
    }
  }

  pub fn draw<G: Graphics>(&mut self, c: &Context, g: &mut G) {
    let shape = [
      [0.0, -10.0],
      [-5.0, 10.0],
      [5.0, 10.0]
    ];
    let thrust = [
      [-3.0, 11.0],
      [0.0, 13.0],
      [3.0, 11.0]
    ];
    let thrust_color = [1.0, 0.0, 0.0, 0.8];

    let transform = c
      .transform
      .trans(self.pos_x, self.pos_y)
      .rot_rad(self.rotation);
    polygon(self.color, &shape, transform, g);
    if self.thrusting {
      polygon(thrust_color, &thrust, transform, g);
    }
  }
}
