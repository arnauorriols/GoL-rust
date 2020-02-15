#[derive(Debug)]
pub struct World {
	pub cells: Vec<Vec<CellState>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CellState {
	Dead,
	Alive,
}

#[derive(PartialEq, Eq, Hash)]
pub struct RuleForLife {
	pub neighbour_count: u8,
	pub cell_state: CellState,
}

#[derive(serde::Deserialize)]
pub struct Config {
	pub world: WorldConfig,
	pub patterns: Vec<PatternsConfig>,
}

#[derive(serde::Deserialize)]
pub struct WorldConfig {
	pub width: u16,
	pub height: u16,
	pub alive_symbol: String,
	pub dead_symbol: String,
	pub speed: f32,
}

#[derive(serde::Deserialize)]
pub struct PatternsConfig {
	pub start_x: u16,
	pub start_y: u16,
	pub living_cells: Vec<(u16, u16)>,
}
