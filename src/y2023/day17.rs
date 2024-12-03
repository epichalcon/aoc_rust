use rstest::rstest;
use std::collections::{binary_heap, BinaryHeap};
use std::{collections::HashSet, fmt::Debug, time::Instant};

use crate::utils::coords::Coordinates;
use crate::utils::direction::{self, Direction};
use crate::utils::transposer::print_matrix;

pub fn solve(input: &str) {
    // let start_time = Instant::now();
    // println!("First star: {}", func1(input).unwrap());
    // println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input).unwrap());
    println!("\t time:{:?}", start_time.elapsed());
}

#[derive(Eq, Hash)]
struct Node {
    coord: Coordinates<i32>,
    heat_loss: i32,
    strait_steps: i32,
    direction: Option<Direction>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("coord", &self.coord)
            .field("heat_loss", &self.heat_loss)
            .field("strait_steps", &self.strait_steps)
            .field("direction", &self.direction)
            .finish()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
            && self.strait_steps == other.strait_steps
            && self.direction == other.direction
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat_loss.partial_cmp(&self.heat_loss)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.partial_cmp(&other.heat_loss).unwrap()
    }
}

fn is_in_bounds(coord: Coordinates<i32>, input: &str) -> bool {
    !(coord.x() < 0
        || coord.y() < 0
        || coord.x() >= input.lines().next().unwrap().len().try_into().unwrap()
        || coord.y() >= input.lines().count().try_into().unwrap())
}

fn func1(input: &str) -> Option<usize> {
    let origin = Coordinates::new(0, 0);
    let last = Coordinates::new(
        input.lines().next().unwrap().len().try_into().unwrap(),
        input.lines().count().try_into().unwrap(),
    );
    let dest = last - Coordinates::new(1, 1);

    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<(Coordinates<i32>, i32, Option<Direction>)> = HashSet::new();
    open_list.push(Node {
        coord: origin,
        heat_loss: 0,
        strait_steps: 0,
        direction: None,
    });

    while open_list.len() > 0 {
        let actual_node = open_list.pop().unwrap();

        if actual_node.coord == dest {
            return Some(actual_node.heat_loss as usize);
        }

        if close_list.contains(&(
            actual_node.coord,
            actual_node.strait_steps,
            actual_node.direction,
        )) {
            continue;
        }

        close_list.insert((
            actual_node.coord,
            actual_node.strait_steps,
            actual_node.direction,
        ));

        if actual_node.strait_steps < 3 && actual_node.direction != None {
            let new_coords = actual_node.coord.step(actual_node.direction.unwrap());
            if is_in_bounds(new_coords, input) {
                let heat = input
                    .lines()
                    .nth(new_coords.y() as usize)
                    .unwrap()
                    .chars()
                    .nth(new_coords.x() as usize)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();

                open_list.push(Node {
                    coord: new_coords,
                    heat_loss: actual_node.heat_loss + heat,
                    strait_steps: actual_node.strait_steps + 1,
                    direction: actual_node.direction,
                })
            }
        }

        for new_direction in Direction::get_directions() {
            if actual_node.direction == None
                || (new_direction != actual_node.direction.unwrap()
                    && new_direction != actual_node.direction.unwrap().reverse_direction())
            {
                let new_coords = actual_node.coord.step(new_direction);
                if is_in_bounds(new_coords, input) {
                    let heat = input
                        .lines()
                        .nth(new_coords.y() as usize)
                        .unwrap()
                        .chars()
                        .nth(new_coords.x() as usize)
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .unwrap();

                    open_list.push(Node {
                        coord: new_coords,
                        heat_loss: actual_node.heat_loss + heat,
                        strait_steps: 1,
                        direction: Some(new_direction),
                    })
                }
            }
        }
    }
    return None;
}

fn print_path(input: &str, path: Vec<Coordinates<i32>>, actual_coord: Coordinates<i32>) {
    let mut map = vec![vec!['.'; input.lines().next().unwrap().len()]; input.lines().count()];
    for coord in path {
        map[coord.y() as usize][coord.x() as usize] = '#';
    }
    map[actual_coord.y() as usize][actual_coord.x() as usize] = '#';

    print_matrix(map);
}

fn func2(input: &str) -> Option<usize> {
    let origin = Coordinates::new(0, 0);
    let last = Coordinates::new(
        input.lines().next().unwrap().len().try_into().unwrap(),
        input.lines().count().try_into().unwrap(),
    );
    let dest = last - Coordinates::new(1, 1);

    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<(Coordinates<i32>, i32, Option<Direction>)> = HashSet::new();
    open_list.push(Node {
        coord: origin,
        heat_loss: 0,
        strait_steps: 0,
        direction: None,
    });

    while open_list.len() > 0 {
        let actual_node = open_list.pop().unwrap();

        if actual_node.coord == dest
            && (4 <= actual_node.strait_steps && actual_node.strait_steps < 10)
        {
            return Some(actual_node.heat_loss as usize);
        }

        if close_list.contains(&(
            actual_node.coord,
            actual_node.strait_steps,
            actual_node.direction,
        )) {
            continue;
        }

        close_list.insert((
            actual_node.coord,
            actual_node.strait_steps,
            actual_node.direction,
        ));

        if actual_node.strait_steps < 10 && actual_node.direction != None {
            let new_coords = actual_node.coord.step(actual_node.direction.unwrap());
            if is_in_bounds(new_coords, input) {
                let heat = input
                    .lines()
                    .nth(new_coords.y() as usize)
                    .unwrap()
                    .chars()
                    .nth(new_coords.x() as usize)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();

                open_list.push(Node {
                    coord: new_coords,
                    heat_loss: actual_node.heat_loss + heat,
                    strait_steps: actual_node.strait_steps + 1,
                    direction: actual_node.direction,
                })
            }
        }

        for new_direction in Direction::get_directions() {
            if actual_node.direction == None
                || (actual_node.strait_steps >= 4
                    && new_direction != actual_node.direction.unwrap()
                    && new_direction != actual_node.direction.unwrap().reverse_direction())
            {
                let new_coords = actual_node.coord.step(new_direction);
                if is_in_bounds(new_coords, input) {
                    let heat = input
                        .lines()
                        .nth(new_coords.y() as usize)
                        .unwrap()
                        .chars()
                        .nth(new_coords.x() as usize)
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .unwrap();

                    open_list.push(Node {
                        coord: new_coords,
                        heat_loss: actual_node.heat_loss + heat,
                        strait_steps: 1,
                        direction: Some(new_direction),
                    })
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let input = "19
24";

        let expected = 6;

        assert_eq!(expected, func1(input).unwrap());
    }

    #[test]
    fn test_func1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let expected = 102;

        assert_eq!(expected, func1(input).unwrap());
    }

    #[test]
    fn test_func2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let expected = 94;

        assert_eq!(expected, func2(input).unwrap());
    }

    #[test]
    fn test_func2_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let expected = 71;

        assert_eq!(expected, func2(input).unwrap());
    }
}
