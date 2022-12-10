use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
	Alive,
	Dead,
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

	let w = screen_width() as usize / 10;
	let h = screen_height() as usize / 10;

	let mut cells = vec![CellState::Dead; w * h];
	let mut buffer = vec![CellState::Dead; w * h];

	let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

	for cell in cells.iter_mut() {
		if rand::gen_range(0, 5) == 0 {
			*cell = CellState::Alive;
		}
	}
	let texture = Texture2D::from_image(&image);

	texture.set_filter(FilterMode::Nearest);

	loop {
		clear_background(WHITE);

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

						let neighbor = cells[(y + j) as usize * w + (x + i) as usize];
						if neighbor == CellState::Alive {
							neighbors_count += 1;
						}
					}
				}

				let current_cell = cells[y as usize * w + x as usize];
				buffer[y as usize * w + x as usize] = match (current_cell, neighbors_count) {
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

		for i in 0..buffer.len() {
			let x = (i % w) as u32;
			let y = (i / w) as u32;

			cells[i] = buffer[i];

			if (x, y) == (pos.0 as u32, pos.1 as u32) {
				cells[i] = CellState::Alive;
				image.set_pixel(x, y, WHITE);
			} else {
				image.set_pixel(
					x,
					y,
					match buffer[i] {
						CellState::Alive => macroquad::color::hsl_to_rgb((i % w) as f32 / (w as f32) + qq, 1., 0.5),
						CellState::Dead => BLACK,
					},
				);
			}
		}

		texture.update(&image);

		qq += tt;
		if qq < 0.0 || qq > 1.0 {
			tt = -tt;
			qq += tt;
		}

		let params = DrawTextureParams {
			dest_size: Some(Vec2 { x: screen_width(), y: screen_height() }),
			..Default::default()
		};

		draw_texture_ex(texture, 0., 0., WHITE, params);

		next_frame().await
	}
}
