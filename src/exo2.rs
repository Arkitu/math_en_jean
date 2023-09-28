use std::collections::HashSet;

use grid::Grid;

fn evaluate_grid(grid: &Grid<i8>) -> bool {
    let mut set = HashSet::new();
    for row in grid.iter_rows() {
        let x = row.map(|x|*x).reduce(|a,b| a+b).unwrap();
        if set.contains(&x) {
            return false
        }
        set.insert(x);
    }
    true
}

fn compute_grid(size: usize) {
    let mut grid = Grid::init(size, size, -1);
    for i in 0..(size^2) {
        let
    }
}

pub async fn main() {
    
}
