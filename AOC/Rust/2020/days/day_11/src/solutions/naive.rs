use aoclib::*;
use itertools::Itertools;
use std::convert::TryInto;

pub struct Puzzle;

type Grid = Vec<Vec<char>>;

impl InputParsable for Puzzle {
    type Input = Grid;
    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let input: Self::Input = raw_input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|row| row.chars().collect())
            .collect();

        Ok(input)
    }
}

impl Solvable for Puzzle {
    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::count_occupied(&Self::update(input)))
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::count_occupied(&Self::update2(input)))
    }
}

impl Puzzle {
    fn update(grid: &Grid) -> Grid {
        let mut new_grid = grid.clone();

        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, cell) in row.iter().enumerate() {
                let adj_occupied_count = Self::count_adjacent_occupied(&grid, row_num, col_num);
                match cell {
                    'L' if adj_occupied_count == 0 => new_grid[row_num][col_num] = '#',
                    '#' if adj_occupied_count >= 4 => new_grid[row_num][col_num] = 'L',
                    _ => {}
                };
            }
        }
        // Self::print_grid(&new_grid);
        if grid == &new_grid {
            return new_grid;
        }
        Self::update(&new_grid)
    }
    fn update2(grid: &Grid) -> Grid {
        let mut new_grid = grid.clone();

        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, cell) in row.iter().enumerate() {
                let adj_cansee_count = Self::count_cansee_occupied(&grid, row_num, col_num);
                match cell {
                    'L' if adj_cansee_count == 0 => new_grid[row_num][col_num] = '#',
                    '#' if adj_cansee_count >= 5 => new_grid[row_num][col_num] = 'L',
                    _ => {}
                };
            }
        }
        // Self::print_grid(&new_grid);
        if grid == &new_grid {
            return new_grid;
        }
        Self::update2(&new_grid)
    }

    fn print_grid(grid: &Grid) {
        for row in grid.iter() {
            for cell in row.iter() {
                print!("{}", cell)
            }
            println!()
        }
    }

    fn count_cansee_occupied(grid: &Grid, row_num: usize, col_num: usize) -> u32 {
        let mut count = 0;

        ([-1, 0, 1].iter())
            .cartesian_product([-1, 0, 1].iter())
            .filter(|(row_delta, col_delta)| (0, 0) != (**row_delta, **col_delta))
            .for_each(|(row_delta, col_delta)| {
                let mut l = 1;
                while let Some(adjacent_cell) = grid
                    .get((row_num as i32 + row_delta * l) as usize)
                    .and_then(|row| row.get((col_num as i32 + col_delta * l) as usize))
                {
                    l += 1;
                    match adjacent_cell {
                        '#' => {
                            count += 1;
                            break;
                        }
                        'L' => break,
                        _ => continue,
                    }
                }
            });

        count
    }

    fn count_occupied(grid: &Grid) -> u32 {
        grid.iter()
            .map(|row| row.iter().filter(|cell| **cell == '#').count() as u32)
            .sum()
    }

    fn count_adjacent_occupied(grid: &Grid, row_num: usize, col_num: usize) -> u32 {
        let mut count = 0;

        ([-1, 0, 1].iter())
            .cartesian_product([-1, 0, 1].iter())
            .filter(|(row_delta, col_delta)| (0, 0) != (**row_delta, **col_delta))
            .map(|(row_delta, col_delta)| {
                (row_num as isize + row_delta, col_num as isize + col_delta)
            })
            .filter_map(|(row_num, col_num)| -> Option<(usize, usize)> {
                if let (Ok(row_num), Ok(col_num)) = (row_num.try_into(), col_num.try_into()) {
                    return Some((row_num, col_num));
                }
                None
            })
            .for_each(|(row_num, col_num)| {
                if let Some(adjacent_cell) = grid.get(row_num).and_then(|row| row.get(col_num)) {
                    if *adjacent_cell == '#' {
                        count += 1;
                    }
                }
            });

        count
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
