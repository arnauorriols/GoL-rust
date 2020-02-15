use std::collections::HashSet;

use super::domain;

pub fn build_world(config: &domain::Config) -> domain::World {
	let mut world = domain::World {
		cells: vec![
			vec![domain::CellState::Dead; config.world.width as usize];
			config.world.height as usize
		],
	};
	for pattern in config.patterns.iter() {
		for (x, y) in pattern.living_cells.iter() {
			world.cells[(pattern.start_y + y) as usize][(pattern.start_x + x) as usize] =
				domain::CellState::Alive;
		}
	}
	world
}

pub fn tick(rules: &HashSet<&domain::RuleForLife>, world: &domain::World) -> domain::World {
	let mut new_world = domain::World { cells: vec![] };
	for y in 0..world.cells.len() {
		for x in 0..world.cells[y].len() {
			if x == 0 {
				new_world.cells.push(vec![]);
			}
			let current_state = world.cells[y][x];
			let neighbour_count = count_alive_neighbours(x as i32, y as i32, world);
			let new_state = calculate_new_state(rules, neighbour_count, current_state);
			new_world.cells[y].push(new_state);
		}
	}
	new_world
}

fn count_alive_neighbours(x: i32, y: i32, world: &domain::World) -> u8 {
	let upper_left = (abs_y(y - 1, world), abs_x(x - 1, world));
	let upper = (abs_y(y - 1, world), abs_x(x, world));
	let upper_right = (abs_y(y - 1, world), abs_x(x + 1, world));
	let left = (abs_y(y, world), abs_x(x - 1, world));
	let right = (abs_y(y, world), abs_x(x + 1, world));
	let lower_left = (abs_y(y + 1, world), abs_x(x - 1, world));
	let lower = (abs_y(y + 1, world), abs_x(x, world));
	let lower_right = (abs_y(y + 1, world), abs_x(x + 1, world));
	let neighbour_cells: [domain::CellState; 8] = [
		world.cells[upper_left.0 as usize][upper_left.1 as usize],
		world.cells[upper.0 as usize][upper.1 as usize],
		world.cells[upper_right.0 as usize][upper_right.1 as usize],
		world.cells[left.0 as usize][left.1 as usize],
		world.cells[right.0 as usize][right.1 as usize],
		world.cells[lower_left.0 as usize][lower_left.1 as usize],
		world.cells[lower.0 as usize][lower.1 as usize],
		world.cells[lower_right.0 as usize][lower_right.1 as usize],
	];
	neighbour_cells
		.iter()
		.filter(|&c| *c == domain::CellState::Alive)
		.count() as u8
}

fn abs_y(y: i32, world: &domain::World) -> i32 {
	let n_rows = world.cells.len() as i32;
	abs_coord(y, n_rows)
}

fn abs_x(x: i32, world: &domain::World) -> i32 {
	let n_columns = world.cells[0].len() as i32;
	abs_coord(x, n_columns)
}

fn abs_coord(coord: i32, size: i32) -> i32 {
	let last_pos = size - 1;
	let overflow_pos = size;
	match coord {
		coord if coord < 0 => overflow_pos + coord,
		coord if coord > last_pos => coord - overflow_pos,
		_ => coord,
	}
}

fn calculate_new_state(
	rules: &HashSet<&domain::RuleForLife>,
	neighbour_count: u8,
	current_state: domain::CellState,
) -> domain::CellState {
	if rules.contains(&domain::RuleForLife {
		neighbour_count,
		cell_state: current_state,
	}) {
		domain::CellState::Alive
	} else {
		domain::CellState::Dead
	}
}
