use std::{collections::HashSet, vec};

fn print_grid(grid: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<char>>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell.into());
        }
        println!();
    }
}

fn copied_inner_iter<'a, INNER, T>(
    grid: impl IntoIterator<Item = INNER>,
) -> impl Iterator<Item = impl Iterator<Item = T>>
where
    INNER: IntoIterator<Item = &'a T> + Clone,
    T: 'a + Copy,
{
    grid.into_iter().map(|row| row.into_iter().cloned())
}

fn make_grid_from_sprase_grid(
    sparse_grid: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
    (0..rows).map(move |i| (0..cols).map(move |j| sparse_grid.get(&(i, j)).map_or('.', |_| 'X')))
}

fn main() {
    let grid = [
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        ['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', 'X', 'X', '.', '.', '.', '.'],
        ['.', '.', '.', '.', 'X', 'X', '.', '.', '.', '.'],
    ];
    println!("2D grid:");
    print_grid(copied_inner_iter(&grid));

    let vec_grid = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'X', 'X', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', 'X', 'X', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', 'X', 'X', '.', '.', '.', '.'],
    ];
    println!("2D vec grid:");
    print_grid(copied_inner_iter(&vec_grid));

    let sparse_grid = HashSet::<(usize, usize)>::from_iter([
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 4),
        (3, 5),
        (4, 4),
        (4, 5),
    ]);

    let sparse_grid_iter = make_grid_from_sprase_grid(&sparse_grid, grid.len(), grid[0].len());

    println!("Sparse grid:");
    print_grid(sparse_grid_iter);
}
