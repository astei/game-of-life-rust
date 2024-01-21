use rand::Rng;

extern crate rand;

pub(crate) struct ConwayBoard {
    pub width: usize,
    pub height: usize,
    drylife: bool,
    cells: Vec<bool>,
}

impl ConwayBoard {
    pub fn new(width: usize, height: usize) -> ConwayBoard {
        ConwayBoard {
            width: width,
            height: height,
            drylife: false,
            cells: vec![false; width * height],
        }
    }

    pub fn new_drylife(width: usize, height: usize) -> ConwayBoard {
        ConwayBoard {
            width: width,
            height: height,
            drylife: true,
            cells: vec![false; width * height],
        }
    }

    pub fn randomize(&mut self) {
        let mut cells = vec![false; self.width * self.height];
        rand::thread_rng().fill(&mut cells[..]);
        self.cells = cells;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.cells[y * self.width + x] = value;
    }

    fn count_live_neighbors_with_wrapround(&self, x: usize, y: usize) -> u32 {
        let mut neighboring = 0;
    
        let wrap_around = |coord, max, delta| -> usize {
            (((coord as isize) + delta + (max as isize)) % (max as isize)) as usize
        };
    
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
    
                let adjx = wrap_around(x, self.width, dx);
                let adjy = wrap_around(y, self.height, dy);
    
                if self.get(adjx, adjy) {
                    neighboring += 1;
                }
            }
        }
    
        neighboring
    }

    pub fn simulate(&mut self) {
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let neighboring = self.count_live_neighbors_with_wrapround(x, y);
                let idx = y * self.width + x;

                new_cells[idx] = match (self.cells[idx], neighboring, self.drylife) {
                    (true, 2, _) | (true, 3, _) => true,
                    (false, 3, _) => true,
                    (false, 7, true) => true,
                    _ => false,
                }
            }
        }

        self.cells = new_cells;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::ConwayBoard;

    #[test]
    fn test_empty_board() {
        let mut board = ConwayBoard::new(3, 3);
        board.simulate();

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(board.get(x, y), false);
            }
        }
    }

    #[test]
    fn test_single_cell_death() {
        let mut board = ConwayBoard::new(3, 3);
        board.set(0, 1, true);
        board.simulate();

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(board.get(x, y), false);
            }
        }
    }

    #[test]
    fn test_reproduction() {
        let mut board = ConwayBoard::new(3, 3);

        // Set up three neighbors
        board.set(0, 0, true);
        board.set(0, 1, true);
        board.set(1, 0, true);
        board.simulate();

        assert_eq!(board.get(1, 1), true);
    }

    #[test]
    fn test_still_life_block() {
        let mut board = ConwayBoard::new(3, 3);

        board.set(0, 0, true);
        board.set(0, 1, true);
        board.set(1, 0, true);
        board.set(1, 1, true);
        board.simulate();

        assert_eq!(board.get(1, 1), true);
    }
}