use std::{process::exit, thread, time};

mod application;
mod domain;
mod io;
mod presentation;

fn main() {
	let rules = [
		domain::RuleForLife {
			neighbour_count: 2,
			cell_state: domain::CellState::Alive,
		},
		domain::RuleForLife {
			neighbour_count: 3,
			cell_state: domain::CellState::Alive,
		},
		domain::RuleForLife {
			neighbour_count: 3,
			cell_state: domain::CellState::Dead,
		},
	]
	.iter()
	.collect();
	let config = io::config::load();
	let mut world = application::build_world(&config);

	presentation::setup();
	ctrlc::set_handler(|| {
		presentation::teardown();
		exit(0)
	})
	.unwrap();
	presentation::intro(&config);
	presentation::render(&config, &world);
	loop {
		world = application::tick(&rules, &world);
		presentation::rerender(&config, &world);
		thread::sleep(time::Duration::from_millis(
			(50.0 / config.world.speed) as u64
		));
	}
}
