// rust impl for http://norvig.com/sudoku.html
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Loc {
    row: usize,
    col: usize,
}

struct Possible {
    b: [bool; 9],
}

impl Possible {
    fn is_on(&self, i: u8) -> bool {
        return self.b[i as usize - 1];
    }

    fn count(&self) -> u8 {
        self.b.iter().fold(0, |acc, &x| {
            acc + (x != false) as u8
        })
    }

    fn eliminate(&mut self, i: u8) {
        self.b[i as usize - 1] = false;
    }

    fn val(&self) -> u8 {
        (self.b.iter().position(|&x| x == true).unwrap() + 1) as u8
    }
}

struct Sudoku {
    cells: Vec<Vec<Possible>>,
    units: HashMap<Loc, Vec<Vec<Loc>>>,
    peers: HashMap<Loc, Vec<Loc>>,
}

impl Sudoku {
    fn assign(&mut self, grid: &[Vec<u8>], row: usize, col: usize, d: u8) -> bool {
        for i in 1..10 {
            if i != d {
                if !self.eliminate(grid, row, col, i) { return false; }
            }
        }
        true
    }

    fn eliminate(&mut self, grid: &[Vec<u8>], row: usize, col: usize, d: u8) -> bool {
        let loc: Loc = Loc{row, col};

        if self.cells[row][col].is_on(d) {
            return true;
        }

        self.cells[row][col].eliminate(d);

        // 1. If a square s is reduced to one value d2, then eliminate d2 from peers.
        match self.cells[row][col].count() {
            0 => return false,
            1 => {
                let v = self.cells[row][col].val();
                let peer = self.peers[&loc].clone();
                if !peer.iter().all(|x| self.eliminate(grid, x.row, x.col, v)) {
                    return false;
                }
            },
            _ => println!(),
        };

        // 2. If a unit u is reduced to only one place for a value d, then put it there.
        for u in self.units[&loc].clone() {
            let dplaces: Vec<&Loc> = u.iter().filter(|x| self.cells[x.row][x.col].is_on(d)).collect();
            match dplaces.len() {
                0 => return false,
                1 => {
                    if !self.assign(grid, dplaces[0].row, dplaces[0].col, d) { return false; }
                },
                _ => println!(),
            }
        }
        true
    }
}

fn find_unassigned_location(grid: &[Vec<u8>]) -> Option<Loc> {
    for (row, row_grid) in grid.iter().enumerate() {
        match row_grid.iter().position(|&x| x == 0) {
            Some(col) => return Some(Loc{row, col}),
            None => continue,
        }
    }
    None
}

fn used_in_row(grid: &[Vec<u8>], row: usize, num: u8) -> bool {
    grid[row].iter().any(|&x| x == num)
}

fn used_in_col(grid: &[Vec<u8>], col: usize, num: u8) -> bool {
    grid.iter().any(|x| x[col] == num)
}

fn used_in_box(grid: &[Vec<u8>], row_start: usize, col_start: usize, num: u8) -> bool {
    grid.iter().skip(row_start).take(3).any(|x| x.iter().skip(col_start).take(3).any(|&y| y == num))
}

fn is_safe(grid: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
    !used_in_row(grid, row, num) &&
    !used_in_col(grid, col, num) &&
    !used_in_box(grid, row / 3 * 3, col / 3 * 3, num)
}

fn solve_sudoku(grid: &mut Vec<Vec<u8>>) -> bool {
    let loc = match find_unassigned_location(grid) {
        Some(x) => x,
        None => return true,
    };

    for num in 1..10 {
        if is_safe(grid, loc.row, loc.col, num) {
            grid[loc.row][loc.col] = num;

            if solve_sudoku(grid) { return true; }

            grid[loc.row][loc.col] = 0;
        }
    }
    false
}

fn print_grid(grid: &[Vec<u8>]) {
    for row_grid in grid.iter() {
        println!("{:?}", row_grid);
    }
    println!();
}

fn main() {
    let mut grid = vec![
        vec![3, 0, 6, 5, 0, 8, 4, 0, 0],
        vec![5, 2, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 8, 7, 0, 0, 0, 0, 3, 1],
        vec![0, 0, 3, 0, 1, 0, 0, 8, 0],
        vec![9, 0, 0, 8, 6, 3, 0, 0, 5],
        vec![0, 5, 0, 0, 9, 0, 6, 0, 0],
        vec![1, 3, 0, 0, 0, 0, 2, 5, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 7, 4],
        vec![0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    print_grid(&grid);

    if solve_sudoku(&mut grid) {
        print_grid(&grid);
    } else {
        println!("No solution exists");
    }
}
