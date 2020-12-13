use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    changes: Vec<bool>,
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Universe {
    fn get_index_i(&self, row: i32, column: i32) -> usize {
        (
            modulo(row, self.height as i32) * (self.width as i32) +
            modulo(column, self.width as i32)
        ) as usize
    }
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        self.get_index_i(row as i32, column as i32)
    }

    fn get_i(&self, row: i32, column: i32) -> Cell {
        self.cells[self.get_index_i(row, column)]
    }
    pub fn get(&self, row: u32, column: u32) -> Cell {
        self.get_i(row as i32, column as i32)
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 { Cell::Alive }
            else { Cell::Dead }
        });
        let changes = cells.clone().map(|c| {
            if c == Cell::Alive { true }
            else { false }
        }).collect();
        Universe {
            width, height, changes,
            cells: cells.collect(),
        }
    }

    fn live_neighbor_cnt(&self, row: u32, column: u32) -> u8 {
        let row_i = row as i32;
        let column_i = column as i32;
        let mut cnt = 0;
        for y in (row_i - 1)..(row_i + 2) {
            for x in (column_i - 1)..(column_i + 2) {
                if y == row_i && x == column_i { continue; }
                cnt += self.get_i(y, x) as u8;
            }
        }
        cnt
    }

    pub fn tick(&mut self) {
        let mut cells_next_tick = self.cells.clone();
        let mut changes_next_tick = self.changes.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.get_index(y, x);
                let cell = self.get(y, x);
                let cell_next_tick = match (cell, self.live_neighbor_cnt(y, x)) {
                    (Cell::Alive, cnt) if cnt < 2 || cnt > 3 => Cell::Dead,
                    (Cell::Alive, _) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };

                changes_next_tick[i] = cell != cell_next_tick;
                cells_next_tick[i] = cell_next_tick;
            }
        }

        self.changes = changes_next_tick;
        self.cells = cells_next_tick;
    }

    pub fn changed(&self, i: usize) -> bool {
        self.changes[i]
    }
    pub fn toggle(&mut self, i: usize) {
        let cell = self.cells[i];
        if cell == Cell::Alive {
            self.cells[i] = Cell::Dead;
        } else {
            self.cells[i] = Cell::Alive;
        }
    }
    pub fn sum(&self) -> u32 {
        self.cells.iter().map(|c| *c as u32).sum()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(y, x))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = if *self == Cell::Alive { 'A' } else { '_' };
        write!(f, "{}", symbol)
    }
}
