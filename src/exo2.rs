use std::collections::HashSet;
use grid::Grid;

fn evaluate_grid(grid: &Vec<i8>, size: usize) -> bool {
    let mut set = HashSet::new();
    let mut cols = vec![0; size];
    for row in grid.chunks(size) {
        for (i, a) in row.iter().enumerate() {
            *cols.get_mut(i).unwrap() += a;
        }
        let x = row.iter().map(|x|*x).reduce(|a,b| a+b).unwrap();
        if set.contains(&x) {
            return false
        }
        set.insert(x);
    }
    for col in cols {
        if set.contains(&col) {
            return false
        }
        set.insert(col);
    }
    true
}

// nombre de recursion après lequel on crée un nouveau thread
const THREAD_RECURSION_COUNT: usize = 15;

#[async_recursion::async_recursion]
async fn compute_line(grid: &mut Vec<i8>, size: usize, i: usize) -> bool {
    let new_treads = i % THREAD_RECURSION_COUNT == THREAD_RECURSION_COUNT-1;
    let base_value = grid.get(i).unwrap().to_owned();
    let mut threads = if new_treads {
        Some(Vec::new())
    } else {None};
    for val in -1..=1 {
        if new_treads {
            let mut grid = grid.clone();
            threads.as_mut().unwrap().push(
                tokio::spawn(async move {
                    *grid.get_mut(i).unwrap() = val;
                    if i > 0 {
                        if compute_line(&mut grid, size, i-1).await {
                            return true
                        }
                    } else {
                        if evaluate_grid(&grid, size) {
                            return true
                        }
                    }
                    false
                })
            )
        } else {
            *grid.get_mut(i).unwrap() = val;
            if i > 0 {
                if compute_line(grid, size, i-1).await {
                    return true
                }
            } else {
                if evaluate_grid(grid, size) {
                    return true
                }
            }
        }
    }
    match threads {
        Some(t) => for t in t {
            if t.await.unwrap() {
                return true
            }
        },
        None => {}
    }
    *grid.get_mut(i).unwrap() = base_value;
    false
}

async fn compute_grid(size: usize) -> bool {
    let mut grid: Vec<i8> = vec![-1; size.pow(2)];
    let result = compute_line(&mut grid, size, size.pow(2)-1).await;
    dbg!(Grid::from_vec(grid, size));
    result
}

pub async fn main(args: &[String]) {
    let size = args.get(0)
        .expect("No size gived !")
        .parse::<usize>()
        .expect("Invalid size !");
    dbg!(compute_grid(size).await);
}
