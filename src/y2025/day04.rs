use std::{slice::Iter, time::Instant};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult,
};

use crate::utils::coords::Coordinates;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

#[derive(Clone, PartialEq, Eq, Copy)]
enum Cell {
    Paper,
    Empty,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Paper => write!(f, "@"), // or "x", "█", etc.
            Cell::Empty => write!(f, "."),
        }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the same representation as Display for Debug
        write!(f, "{}", self)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    map: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(map: Vec<Vec<Cell>>) -> Self {
        Self { map }
    }
    fn rows(&self) -> usize {
        self.map.len()
    }
    fn cols(&self) -> usize {
        self.map[0].len()
    }

    fn get_coord(&self, c: &Coordinates<usize>) -> Option<Cell> {
        Some(*self.map.get(c.x())?.get(c.y())?)
    }

    fn get_removed_papers(&self, other: &Grid) -> usize {
        self.into_iter().zip(other).fold(0, |acc, (s_row, o_row)| {
            acc + s_row
                .iter()
                .zip(o_row)
                .filter(|(s_cell, o_cell)| s_cell != o_cell)
                .count()
        })
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?; // New line after each row
        }
        Ok(())
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid({}x{}):", self.rows(), self.cols())?;

        for row in self.map.iter() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Iterator over rows (returns references to rows)
impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Vec<Cell>;
    type IntoIter = Iter<'a, Vec<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

// Or implement Iterator directly for Grid (consuming)
impl IntoIterator for Grid {
    type Item = Vec<Cell>;
    type IntoIter = std::vec::IntoIter<Vec<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    let (s, t) = alt((tag("."), tag("@")))(input)?;
    match t {
        "." => Ok((s, Cell::Empty)),
        "@" => Ok((s, Cell::Paper)),
        x => panic!("unknown character {x}"),
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell)(input)
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (res, map) = separated_list1(line_ending, parse_line)(input)?;

    Ok((res, Grid { map }))
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u32 {
    let (_, map) = parse_grid(input).unwrap();

    (0..map.rows()).fold(0, |acc, row| {
        let row_papers = (0..map.cols()).fold(0, |acc1, col| {
            let c = Coordinates::new(row, col);

            if map.get_coord(&c) == Some(Cell::Paper) {
                let paper_count = c
                    .all_neighbors()
                    .iter()
                    .filter(|coord| map.get_coord(coord) == Some(Cell::Paper))
                    .count();

                acc1 + (paper_count < 4) as u32
            } else {
                acc1
            }
        });
        acc + row_papers
    })
}

fn func2(input: &str) -> u32 {
    let (_, mut map) = parse_grid(input).unwrap();

    let mut paper_removed = 0;

    loop {
        let new_map = Grid::new(
            map.clone()
                .into_iter()
                .enumerate()
                .map(|(n_row, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(n_col, cell)| match cell {
                            Cell::Paper => {
                                let paper_count = Coordinates::new(n_row, n_col)
                                    .all_neighbors()
                                    .iter()
                                    .filter(|coord| map.get_coord(coord) == Some(Cell::Paper))
                                    .count();
                                if paper_count < 4 {
                                    Cell::Empty
                                } else {
                                    Cell::Paper
                                }
                            }
                            other => *other,
                        })
                        .collect()
                })
                .collect(),
        );

        if new_map == map {
            break;
        } else {
            paper_removed += new_map.get_removed_papers(&map);
            map = new_map;
        }
    }

    paper_removed as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected = 13;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected = 43;

        assert_eq!(func2(input), expected);
    }
}
