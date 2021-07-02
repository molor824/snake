mod app;

use raylib::prelude::*;
use glm::IVec2;
use app::*;

const WINDOW: IVec2 = IVec2{x: 800, y: 600};

fn main() {
	let (mut rh, thread) = raylib::init().size(WINDOW.x, WINDOW.y).title("test").build();

	let mut app = App::new();

	while !rh.window_should_close() {
		let mut rdh = rh.begin_drawing(&thread);

		app.update(&mut rdh);
		app.render(&mut rdh);
	}
}