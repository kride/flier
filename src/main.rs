use piston_window::*;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

mod flier;

fn main() {
	let (width, height) = (800, 600);
	let opengl = OpenGL::V3_2;
	let mut window: PistonWindow =
		WindowSettings::new("Flier", (width, height))
		.exit_on_esc(true)
		.graphics_api(opengl)
		.vsync(true)
		.build()
		.unwrap();

	let flier = Arc::new(Mutex::new(flier::Flier::new(width as f64 / 2.0, height as f64 / 2.0, [1.0, 0.0, 0.5, 1.0])));
  let flier_clone = Arc::clone(&flier);
  thread::spawn(move || {
		loop {
			thread::sleep(Duration::from_millis(10));
      let mut flier = flier_clone.lock().unwrap();
			flier.move_to_target();
		}
	});
	  

	while let Some(e) = window.next() {
    let mut flier = flier.lock().unwrap();

    if let Some(pos) = e.mouse_cursor_args() {
      flier.set_target(pos[0] as f64, pos[1] as f64)
		}
		window.draw_2d(&e, |c, g, _| {
			clear([0.0, 0.0, 0.0, 1.0], g);
      flier.draw(&c, g);
		});
	}
}
