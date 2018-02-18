#[derive(Copy,Clone,Debug,PartialEq)]
pub enum State {
    DEAD,
    ALIVE,
}

pub struct Game {
    width: usize,
    height: usize,
    grid: Vec<State>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        if width == 0 || height == 0 {
            panic!("Cannot create a game with a zero size.");
        }
        Self {
            width: width,
            height: height,
            grid: vec![State::DEAD; width * height],
        }
    }
    // Read the state of a cell at a specific location
    pub fn get_state(&self, x: usize, y: usize) -> State {
        self.grid[self.x_y_to_idx(x, y)]
    }
    // Changes the state of a cell to the specified state
    pub fn set_state(&mut self, state: State, x: usize, y: usize) {
        let index = self.x_y_to_idx(x, y);
        self.grid[index] = state;
    }
    /*
    Updates the grid by one step following these rules:
    -------------------------
    * Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    * Any live cell with two or three live neighbours lives on to the next generation.
    * Any live cell with more than three live neighbours dies, as if by overpopulation.
    * Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */
    pub fn step(&mut self) {
        // 1. Create a new array to copy the next step into:
        let mut next_step = vec![State::DEAD; self.width * self.height];
        // 2. For each cell in the current grid:
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                next_step[self.x_y_to_idx(x, y)] = if neighbors < 2 {
                    State::DEAD
                } else if neighbors == 2 {
                    self.get_state(x, y)
                } else if neighbors == 3 {
                    State::ALIVE
                } else {
                    State::DEAD
                };
            }
        }
        self.grid = next_step;
    }
    // Counts the number of live neighbors AROUND the specified coordinate.
    // For coordinate on the edge of the board, out-of-bounds neighbors are considered DEAD.
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            panic!("Cannot count neighbors of an out-of-bounds cell.");
        }
        let mut count: u8 = 0;
        // For each neighbors:
        for j in -1_isize..2_isize {
            for i in -1_isize..2_isize {
                // If we are not the center tile:
                if i != 0 || j != 0 {
                    // If we are in bounds:
                    if x as isize + i >= 0 &&
                       y as isize + j >= 0 &&
                       x as isize + i < self.width as isize &&
                       y as isize + j < self.height as isize {
                        // If that neighbor is alive, count it:
                        if self.get_state((i + x as isize) as usize, (j + y as isize) as usize) == State::ALIVE {
                            count += 1;
                        }
                    }
                }
            }
        }
        // return the count
        count
    }
    // Converts an x/y coordinate to an index for the grid vector:
    // Used internally in the set_state and get_state methods
    fn x_y_to_idx(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!("Coordinates out of bounds");
        }
        x + (y * self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let width = 10;
        let height = 20;
        let game = Game::new(width, height);
        // Ensure all spots in the grid may be accessed and are DEAD
        // after creating a new game.
        for y in 0..height {
            for x in 0..width {
                assert_eq!(game.get_state(x, y), State::DEAD);
            }
        }
    }
    #[test]
    fn x_y_to_idx() {
        let game = Game::new(10, 20);
        assert_eq!(game.x_y_to_idx(0,0), 0);
        assert_eq!(game.x_y_to_idx(1,0), 1);
        assert_eq!(game.x_y_to_idx(2,0), 2);
        assert_eq!(game.x_y_to_idx(9,0), 9);
        assert_eq!(game.x_y_to_idx(0,1), 10);
        assert_eq!(game.x_y_to_idx(0,2), 20);
        assert_eq!(game.x_y_to_idx(2,2), 22);
    }
    #[test]
    fn get_set_state() {
        let mut game = Game::new(1, 1);
        game.set_state(State::ALIVE, 0, 0);
        assert_eq!(game.get_state(0, 0), State::ALIVE);
        game.set_state(State::DEAD, 0, 0);
        assert_eq!(game.get_state(0, 0), State::DEAD);
    }
    #[test]
    fn count_neighbors() {
        let mut game = Game::new(3, 3);
        /*
        The following pattern is used to test channge to the next one after an update:
        0 X 0
        X X X
        0 X X
        */
        game.set_state(State::ALIVE, 1, 0);
        game.set_state(State::ALIVE, 0, 1);
        game.set_state(State::ALIVE, 1, 1);
        game.set_state(State::ALIVE, 2, 1);
        game.set_state(State::ALIVE, 1, 2);
        game.set_state(State::ALIVE, 2, 2);
        assert_eq!(game.count_neighbors(0, 0), 3);
        assert_eq!(game.count_neighbors(1, 0), 3);
        assert_eq!(game.count_neighbors(2, 0), 3);
        assert_eq!(game.count_neighbors(0, 1), 3);
        assert_eq!(game.count_neighbors(1, 1), 5);
        assert_eq!(game.count_neighbors(2, 1), 4);
        assert_eq!(game.count_neighbors(0, 2), 3);
        assert_eq!(game.count_neighbors(1, 2), 4);
        assert_eq!(game.count_neighbors(2, 2), 3);
    }
    #[test]
    fn test_overpop() {
        let mut game = Game::new(3, 3);
        game.set_state(State::ALIVE, 0, 0);
        game.set_state(State::ALIVE, 1, 0);
        game.set_state(State::ALIVE, 2, 0);
        game.set_state(State::ALIVE, 0, 1);
        game.set_state(State::ALIVE, 1, 1);
        game.set_state(State::ALIVE, 2, 1);
        game.set_state(State::ALIVE, 0, 2);
        game.set_state(State::ALIVE, 1, 2);
        game.set_state(State::ALIVE, 2, 2);
        game.step();
        assert_eq!(game.get_state(0, 0), State::DEAD);
        assert_eq!(game.get_state(1, 0), State::DEAD);
        assert_eq!(game.get_state(2, 0), State::DEAD);
        assert_eq!(game.get_state(0, 1), State::DEAD);
        assert_eq!(game.get_state(1, 1), State::DEAD);
        assert_eq!(game.get_state(2, 1), State::DEAD);
        assert_eq!(game.get_state(0, 2), State::DEAD);
        assert_eq!(game.get_state(1, 2), State::DEAD);
        assert_eq!(game.get_state(2, 2), State::DEAD);
    }
    #[test]
    fn test_step_blink() {
        let mut game = Game::new(3, 3);
        /*
        The following pattern should channge to the next one after an update:
        0 0 0
        X X X
        0 0 0

        0 X 0
        0 X 0
        0 X 0
        */
        game.set_state(State::ALIVE, 0, 1);
        game.set_state(State::ALIVE, 1, 1);
        game.set_state(State::ALIVE, 2, 1);
        for _ in 0..2 {
            game.step();
            assert_eq!(game.get_state(0, 0), State::DEAD);
            assert_eq!(game.get_state(1, 0), State::ALIVE);
            assert_eq!(game.get_state(2, 0), State::DEAD);
            assert_eq!(game.get_state(0, 1), State::DEAD);
            assert_eq!(game.get_state(1, 1), State::ALIVE);
            assert_eq!(game.get_state(2, 1), State::DEAD);
            assert_eq!(game.get_state(0, 2), State::DEAD);
            assert_eq!(game.get_state(1, 2), State::ALIVE);
            assert_eq!(game.get_state(2, 2), State::DEAD);
            game.step();
            assert_eq!(game.get_state(0, 0), State::DEAD);
            assert_eq!(game.get_state(1, 0), State::DEAD);
            assert_eq!(game.get_state(2, 0), State::DEAD);
            assert_eq!(game.get_state(0, 1), State::ALIVE);
            assert_eq!(game.get_state(1, 1), State::ALIVE);
            assert_eq!(game.get_state(2, 1), State::ALIVE);
            assert_eq!(game.get_state(0, 2), State::DEAD);
            assert_eq!(game.get_state(1, 2), State::DEAD);
            assert_eq!(game.get_state(2, 2), State::DEAD);
        }
    }
}
