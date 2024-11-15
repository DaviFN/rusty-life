pub mod field;
use field::{Field, CellState};
use rand::Rng;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Game {
    field: Field,
    generation: usize
}

impl Game {

    pub fn new(width: usize, height: usize) -> Game {
        Game{field: Field::new(width, height), generation: 0}
    }

    pub fn get_generation(&self) -> usize {
        self.generation
    }

    pub fn get_field(&mut self) -> &mut Field {
        &mut self.field
    }

    pub fn advance_generation(&mut self, consider_extremes_adjacent: bool)
    {
        let previous_field = self.field.clone();
        for i in 0..self.field.get_width() {
            for j in 0..self.field.get_height() {
                let number_of_neighbours_alive = previous_field.get_number_of_neighbours_alive(i, j, consider_extremes_adjacent);
                if number_of_neighbours_alive < 2 || number_of_neighbours_alive > 3 {
                    self.field.set_cell_state(i, j, CellState::Dead);
                }
                else if number_of_neighbours_alive == 3 {
                    self.field.set_cell_state(i, j, CellState::Alive);
                }
            }
        }
        self.generation = self.generation + 1;
    }

    pub fn clear(&mut self)
    {
        for i in 0..self.field.get_width() {
            for j in 0..self.field.get_height() {
                    self.field.set_cell_state(i, j, CellState::Dead);
            }
        }
    }

    pub fn randomize(&mut self, probability_living_cell: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.field.get_width() {
            for j in 0..self.field.get_height() {
                let is_alive = rng.gen_bool(probability_living_cell / 100.0);
                let state = if is_alive {CellState::Alive} else {CellState::Dead};
                self.field.set_cell_state(i, j, state);
            }
        }
    }

    pub fn apply_seed_1_to_field(&mut self)
    {
        let seed_x: usize = 32;
        let seed_y: usize = 32;

        let x = seed_x;
        let y = seed_y;
        self.field.set_cell_state(x, y, CellState::Alive);
        self.field.set_cell_state(x - 1, y, CellState::Alive);
        self.field.set_cell_state(x, y - 1, CellState::Alive);
        self.field.set_cell_state(x, y + 1, CellState::Alive);
        self.field.set_cell_state(x + 1, y + 1, CellState::Alive);
    }

}