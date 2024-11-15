#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub enum CellState {
    Dead,
    Alive,
    Unknown
}

pub struct CellPosition {
    pub x: usize,
    pub y: usize
}

#[derive(Clone)]
#[derive(Default)]
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Field {
    width: usize,
    height: usize,
    pub cells: Vec<CellState>
}

impl Field {
    pub fn new(width: usize, height: usize) -> Field {
        let n_cells: usize = width * height;
        let cells: Vec<CellState> = vec![CellState::Dead; n_cells];
        Field{width, height, cells}
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_cell_neighbour_positions(&self, x: usize, y: usize, consider_extremes_adjacent: bool) -> Vec<CellPosition> {
        let mut neighbour_positions: Vec<CellPosition> = Vec::new();
        
        let mut possible_x_coordinates: Vec<usize> = Vec::new();
        let mut possible_y_coordinates: Vec<usize> = Vec::new();

        // todo: logic could be reused somehow
        // obs: this is obviously not close to the most efficient way of checking neighbours
        if x > 0 {
            possible_x_coordinates.push(x - 1);
        }
        else { // x == 0 
            if consider_extremes_adjacent {
                possible_x_coordinates.push(self.get_width() - 1);
            }
        }
        possible_x_coordinates.push(x);
        if x + 1 < self.get_width() {
            possible_x_coordinates.push(x + 1);
        }
        else if x + 1 == self.get_width() {
            if consider_extremes_adjacent {
                possible_x_coordinates.push(0);
            }
        }

        if y > 0 {
            possible_y_coordinates.push(y - 1);
        }
        else { // y == 0 
            if consider_extremes_adjacent {
                possible_y_coordinates.push(self.get_height() - 1);
            }
        }
        possible_y_coordinates.push(y);
        if y + 1 < self.get_height() {
            possible_y_coordinates.push(y + 1);
        }
        else if y + 1 == self.get_height() {
            if consider_extremes_adjacent {
                possible_y_coordinates.push(0);
            }
        }

        for i in &possible_x_coordinates {
            for j in &possible_y_coordinates {
                let is_the_cell_position_itself = *i == x && *j == y;
                if !is_the_cell_position_itself {
                    neighbour_positions.push(CellPosition{x: i.clone(), y: j.clone()});
                }
            }
        }

        neighbour_positions
    }

    pub fn is_within_boundaries(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_cell_state(&self, x: usize, y: usize) -> CellState {
        if self.is_within_boundaries(x, y) {
            return self.cells[x + y * self.width]
        }
        CellState::Unknown
    }

    pub fn set_cell_state(&mut self, x: usize, y: usize, cell_state: CellState) {
        if self.is_within_boundaries(x, y) {
            self.cells[x + y * self.width] = cell_state
        }
    }

    pub fn get_number_of_neighbours_alive(&self, x: usize, y: usize, consider_extremes_adjacent: bool) -> usize {
        let mut number_of_neighbours_alive: usize = 0;
        let cell_neighbour_positions: Vec<CellPosition> = self.get_cell_neighbour_positions(x, y, consider_extremes_adjacent);
        for cell_position in cell_neighbour_positions {
            if self.get_cell_state(cell_position.x, cell_position.y)  == CellState::Alive{
                number_of_neighbours_alive += 1
            }
        }
        number_of_neighbours_alive
    }
}