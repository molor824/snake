use crate::*;
use glm::IVec2;
use std::fs::OpenOptions;
use std::fmt::Write as FmtWrite;
use std::io::{Write as IoWrite, Read};

const GRID: IVec2 = IVec2{x: 20, y: 20};
const GRID_LEN: i32 = 20;
const START: IVec2 = IVec2{
	x: WINDOW.x / 2 - GRID.x / 2 * GRID_LEN - GRID_LEN / 2,
	y: WINDOW.y / 2 - GRID.y / 2 * GRID_LEN - GRID_LEN / 2
};
const FONT_SIZE: i32 = 20;
const UP: IVec2 = IVec2{x: 0, y: 1};
const THICKNESS: i32 = 2;
const SAVE_PATH: &str = "best_score";
const FPS: f32 = 10.0;

fn rotate(a: IVec2, dir: u8) -> IVec2 {
	if dir == 1 {IVec2::new(-a.y, a.x)}
	else if dir == 2 {IVec2::new(-a.x, -a.y)}
	else if dir == 3 {IVec2::new(a.y, -a.x)}
	else {a}
}
fn set_grid_pos(mut a: IVec2) -> IVec2 {
	while a.x >= GRID.x {a.x -= GRID.x;}
	while a.x < 0 {a.x += GRID.x}
	while a.y >= GRID.y {a.y -= GRID.y;}
	while a.y < 0 {a.y += GRID.y;}
	a
}
pub struct App {
	dirs: [u8; (GRID.x * GRID.y) as usize],
	dir_l: usize,
	head_pos: IVec2,
	food_pos: IVec2,
	game_over: bool,
	pressed: bool,
	pressed_keys: [bool; 4],
	crnt_dir: u8,
	score: u32,
	best_score: u32,
	elapsed: f32,
	txt_buf: String
}
impl App {
	pub fn new() -> Self {
		Self {
			dirs: [1; (GRID.x * GRID.y) as usize],
			dir_l: 2,
			head_pos: IVec2::new(GRID.x / 2, GRID.y / 2),
			food_pos: IVec2::new(
				get_random_value(0, GRID.x - 1),
				get_random_value(0, GRID.y - 1)
			),
			game_over: false,
			pressed: false,
			pressed_keys: [false; 4],
			crnt_dir: 1,
			score: 0,
			best_score: {
				if let Ok(mut file) = OpenOptions::new().read(true).open(SAVE_PATH) {
					let mut txt = String::new();
					if let Ok(_) = file.read_to_string(&mut txt) {
						if let Ok(n) = txt.parse() {n}
						else {0}
					}
					else {0}
				}
				else {0}
			},
			elapsed: 0.0,
			txt_buf: String::new()
		}
	}
	pub fn update(&mut self, rdh: &mut RaylibDrawHandle) {
		if self.game_over {
			if rdh.is_key_pressed(KeyboardKey::KEY_ENTER) {
				let temp = self.best_score;
				*self = App::new();
				self.best_score = temp;
			}
			return;
		}

		self.elapsed -= rdh.get_frame_time();

		if !self.pressed {
			if !self.pressed_keys[0] && rdh.is_key_pressed(KeyboardKey::KEY_UP) && self.crnt_dir != 0 {
				self.crnt_dir = 2; self.pressed = true;
			}
			else if !self.pressed_keys[1] && rdh.is_key_pressed(KeyboardKey::KEY_DOWN) && self.crnt_dir != 2 {
				self.crnt_dir = 0; self.pressed = true;
			}
			else if !self.pressed_keys[2] && rdh.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.crnt_dir != 1 {
				self.crnt_dir = 3; self.pressed = true;
			}
			else if !self.pressed_keys[3] && rdh.is_key_pressed(KeyboardKey::KEY_LEFT) && self.crnt_dir != 3 {
				self.crnt_dir = 1; self.pressed = true;
			}

			self.pressed_keys[0] = rdh.is_key_pressed(KeyboardKey::KEY_UP);
			self.pressed_keys[1] = rdh.is_key_pressed(KeyboardKey::KEY_DOWN);
			self.pressed_keys[2] = rdh.is_key_pressed(KeyboardKey::KEY_RIGHT);
			self.pressed_keys[3] = rdh.is_key_pressed(KeyboardKey::KEY_LEFT);
		}

		if self.elapsed > 0.0 {return;}
		self.elapsed += 1.0 / FPS;

		self.pressed = false;

		let mut first = true;
		let mut crnt_pos = self.head_pos;
		for dir in &self.dirs[0..self.dir_l] {
			if self.head_pos == crnt_pos && !first {self.game_over = true; return}

			crnt_pos = set_grid_pos(crnt_pos - rotate(UP, *dir));
			first = false;
		}

		let ate = self.food_pos == self.head_pos;
		if ate {
			self.score += 1;
			if self.score > self.best_score {self.best_score = self.score;}

			let mut touched_snake = true;
			while touched_snake {
				self.food_pos = IVec2::new(
					get_random_value(0, GRID.x - 1),
					get_random_value(0, GRID.y - 1)
				);

				touched_snake = false;
				crnt_pos = self.head_pos;
				for dir in &self.dirs[0..self.dir_l] {
					if self.food_pos == crnt_pos {touched_snake = true; break;}

					crnt_pos = set_grid_pos(crnt_pos - rotate(UP, *dir));
				}
			}
		}

		self.head_pos = set_grid_pos(self.head_pos + rotate(UP, self.crnt_dir));

		self.dirs[self.dir_l] = self.crnt_dir;
		self.dir_l += 1;
		for i in (1..self.dir_l).rev() {
			self.dirs.swap(i, i - 1);
		}

		if !ate {self.dir_l -= 1;}
	}
	pub fn render(&mut self, rdh: &mut RaylibDrawHandle) {
		self.txt_buf.clear();
		write!(&mut self.txt_buf, "Score: {}\nBest Score: {}", self.score, self.best_score).unwrap();

		rdh.clear_background(Color::BLACK);

		rdh.draw_rectangle(
			START.x - GRID_LEN,
			START.y - GRID_LEN,
			GRID.x * GRID_LEN + GRID_LEN * 2,
			GRID.y * GRID_LEN + GRID_LEN * 2,
			Color::WHITE
		);
		rdh.draw_rectangle(
			START.x,
			START.y,
			GRID.x * GRID_LEN,
			GRID.y * GRID_LEN,
			Color::BLACK
		);
		rdh.draw_text(
			self.txt_buf.as_str(),
			START.x, START.y - FONT_SIZE * 3 - GRID_LEN,
			FONT_SIZE, Color::WHITE
		);

		let mut crnt_pos = IVec2::new(START.x, START.y) + self.food_pos * GRID_LEN + IVec2::new(1, 1) * THICKNESS;
		rdh.draw_rectangle(
			crnt_pos.x,
			crnt_pos.y,
			GRID_LEN - THICKNESS * 2,
			GRID_LEN - THICKNESS * 2,
			Color::GREEN
		);

		crnt_pos = self.head_pos;
		for i in 0..self.dir_l {
			let mut start_pos = IVec2::new(START.x, START.y) + crnt_pos * GRID_LEN + IVec2::new(1, 1) * THICKNESS;
			let mut size = IVec2::new(1, 1) * (GRID_LEN - THICKNESS * 2);
			let next_pos = crnt_pos - rotate(UP, self.dirs[i]);

			if i != self.dir_l - 1 && next_pos == set_grid_pos(next_pos) {
				if self.dirs[i] == 0 {start_pos.y -= THICKNESS * 2; size.y += THICKNESS * 2;}
				else if self.dirs[i] == 1 {size.x += THICKNESS * 2;}
				else if self.dirs[i] == 2 {size.y += THICKNESS * 2;}
				else {start_pos.x -= THICKNESS * 2; size.x += THICKNESS * 2;}
			}

			rdh.draw_rectangle(start_pos.x, start_pos.y, size.x, size.y, Color::WHITE);
			crnt_pos = set_grid_pos(next_pos);
		}

		crnt_pos = IVec2::new(START.x, START.y) + self.head_pos * GRID_LEN + IVec2::new(1, 1) * THICKNESS;
		rdh.draw_rectangle(
			crnt_pos.x,
			crnt_pos.y,
			GRID_LEN - THICKNESS * 2,
			GRID_LEN - THICKNESS * 2,
			Color::RED
		);

		if self.game_over {
			rdh.draw_rectangle(0, 0, WINDOW.x, WINDOW.y, Color::new(0, 0, 0, 127));
			self.txt_buf.clear();
			write!(&mut self.txt_buf, "Game Over!\nLast Score: {}\nBest Score: {}\nPress 'Enter' to restart!", self.score, self.best_score).unwrap();
			rdh.draw_text(
				self.txt_buf.as_str(),
				WINDOW.x / 2 - 6 * FONT_SIZE,
				WINDOW.y / 2 - FONT_SIZE / 2 * 4,
				FONT_SIZE, Color::WHITE
			);
		}
	}
}
impl Drop for App {
	fn drop(&mut self) {
		write!(
			&mut OpenOptions::new().write(true).truncate(true).create(true).open(SAVE_PATH).unwrap(),
			"{}", self.best_score
		).unwrap();
	}
}