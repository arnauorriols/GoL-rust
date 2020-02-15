use super::domain;

pub fn render(config: &domain::Config, world: &domain::World) {
	let h_border = "━".repeat((config.world.width * 2) as usize);
	println!("┏{}┓", h_border);
	for row in world.cells.iter() {
		for (x, cell) in row.iter().enumerate() {
			if x == 0 {
				print!("┃");
			}
			print!(
				"{} ",
				if cell == &domain::CellState::Alive {
					&config.world.alive_symbol
				} else {
					&config.world.dead_symbol
				}
			);
		}
		println!("┃");
	}
	println!("┗{}┛", h_border);
}

pub fn rerender(config: &domain::Config, world: &domain::World) {
	print!("{}", ansi_escapes::EraseLines(world.cells.len() as u16 + 3));
	render(config, world);
}

pub fn intro(config: &domain::Config) {
	println!();
	println!(" {title_bar} {title} {title_bar}", title = ansi_term::Style::new().bold().paint("GAME OF LIFE"), title_bar = "┅".repeat((config.world.width - 7) as usize));
	println!();
}

pub fn setup() {
	print!("{}", ansi_escapes::CursorHide);
}

pub fn teardown() {
	print!("{}", ansi_escapes::CursorShow);
}
