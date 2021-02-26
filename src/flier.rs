pub struct Flier {
  pub pos_x: f64,
  pub pos_y: f64,
  pub rotation: f64,
  pub color: [f32; 4],
  pub shape: [[f64; 2]; 3],
  target_x: f64,
  target_y: f64
}

impl Flier {
  pub fn new(pos_x: f64, pos_y: f64, color: [f32; 4]) -> Flier {
    let shape = [
      [0.0, -10.0],
      [-5.0, 10.0],
      [5.0, 10.0]
    ];
    Flier { pos_x, pos_y, rotation: 0.0, color, shape, target_x: pos_x, target_y: pos_y }
  }

  pub fn set_target(&mut self, x: f64, y: f64) {
    self.target_x = x;
    self.target_y = y;
  }

  pub fn move_to_target(&mut self) {
    let delta_x = self.target_x - self.pos_x;
    let delta_y = self.target_y - self.pos_y;
    let speed = (delta_x.powi(2) + delta_y.powi(2)).sqrt() / 100.0;
    self.rotation = delta_y.atan2(delta_x);
    self.pos_x += self.rotation.cos() * speed;
    self.pos_y += self.rotation.sin() * speed;
  }
}
