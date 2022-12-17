use macroquad::color::hsl_to_rgb;
use macroquad::prelude::*;
use ndarray::Array2;

const PIXEL_SCALE: usize = 10;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
	Alive,
	Dead,
}

impl Default for CellState {
	fn default() -> Self {
		Self::Dead
	}
}

fn window_conf() -> Conf {
	Conf {
		// fullscreen: true,
		window_resizable: true,
		window_height: 1200,
		window_width: 2000,
		..Default::default()
	}
}
#[macroquad::main(window_conf)]
async fn main() {
	let mut qq = 0.0;
	let mut tt = 0.01;

	let w = screen_width() as usize / PIXEL_SCALE;
	let h = screen_height() as usize / PIXEL_SCALE;

	dbg!(w, h);

	let mut cells = Array2::<CellState>::default((h, w));
	let mut buffer = Array2::<CellState>::default((h, w));
	let mut overlay = Array2::<bool>::default((h, w));

	let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
	let texture = Texture2D::from_image(&image);
	texture.set_filter(FilterMode::Nearest);

	for cell in cells.iter_mut() {
		if rand::gen_range(0, 5) == 0 {
			*cell = CellState::Alive;
		}
	}

	loop {
		if let Some('q') = get_char_pressed() {
			break;
		}

		// clear_background(WHITE);

		let w = image.width();
		let h = image.height();

		for y in 0..h as i32 {
			for x in 0..w as i32 {
				let mut neighbors_count = 0;

				for j in -1i32..=1 {
					for i in -1i32..=1 {
						// out of bounds
						if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
							continue;
						}
						// cell itself
						if i == 0 && j == 0 {
							continue;
						}

						let neighbor = cells[[(y + j) as usize, (x + i) as usize]];
						if neighbor == CellState::Alive {
							neighbors_count += 1;
						}
					}
				}

				let current_cell = cells[[y as usize, x as usize]];
				buffer[[y as usize, x as usize]] = match (current_cell, neighbors_count) {
					// Rule 1: Any live cell with fewer than two live neighbours
					// dies, as if caused by underpopulation.
					(CellState::Alive, x) if x < 2 => CellState::Dead,
					// Rule 2: Any live cell with two or three live neighbours
					// lives on to the next generation.
					(CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
					// Rule 3: Any live cell with more than three live
					// neighbours dies, as if by overpopulation.
					(CellState::Alive, x) if x > 3 => CellState::Dead,
					// Rule 4: Any dead cell with exactly three live neighbours
					// becomes a live cell, as if by reproduction.
					(CellState::Dead, 3) => CellState::Alive,
					// All other cells remain in the same state.
					(otherwise, _) => otherwise,
				};
			}
		}

		let pos = mouse_position();
		show_mouse(pos.1 < 0.0);
		let pos = [pos.1 as usize / PIXEL_SCALE, pos.0 as usize / PIXEL_SCALE];

		for ((y, x), &cellstate) in buffer.indexed_iter() {
			let i = [y, x];

			image.set_pixel(
				x as u32,
				y as u32,
				if i == pos {
					cells[i] = CellState::Alive;

					if is_mouse_button_down(MouseButton::Left) {
						overlay[i] = true;
					};

					WHITE
				} else {
					cells[i] = cellstate;

					match (overlay[i], cellstate) {
						(true, _) => WHITE,
						(false, CellState::Alive) => hsl_to_rgb(x as f32 / (w as f32) + qq, 1., 0.5),
						(false, CellState::Dead) => BLACK,
					}
				},
			);
		}

		texture.update(&image);

		qq += tt;
		if qq < 0.0 || qq > 1.0 {
			tt = -tt;
			qq += tt;
		}

		let params = DrawTextureParams {
			dest_size: Some(Vec2 { x: (w * PIXEL_SCALE) as f32, y: (h * PIXEL_SCALE) as f32 }),
			..Default::default()
		};

		draw_texture_ex(texture, 0., 0., WHITE, params);

		next_frame().await
	}
}
