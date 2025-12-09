use std::time::Instant;

use glam::{u64, U64Vec2};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list1,
    IResult, Parser,
};
use tracing::info;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn parse(input: &str) -> IResult<&str, Vec<U64Vec2>> {
    separated_list1(
        newline,
        separated_list1(tag(","), character::complete::u64).map(|v| U64Vec2::new(v[0], v[1])),
    )(input)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u64 {
    let (_, points) = parse(input).expect("Error in parsing");

    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("Iterator should not be empty")
}

fn print_tiles(width: u64, height: u64, tiles: &[U64Vec2]) {
    let mut s = String::new();
    for y in 0..height {
        for x in 0..width {
            if tiles.contains(&U64Vec2::new(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn point_in_polygon(point: &U64Vec2, edges: &Vec<(&U64Vec2, &U64Vec2)>) -> bool {
    let mut intersect = false;
    for (p1, p2) in edges {
        if point_on_edge(point, p1, p2) {
            return true;
        }
        if (p1.y > point.y) != (p2.y > point.y) && point.x < p1.x {
            intersect = !intersect;
        }
    }
    intersect
}

fn point_on_edge(point: &U64Vec2, a: &U64Vec2, b: &U64Vec2) -> bool {
    if a.x == b.x && point.x == a.x && a.y < b.y {
        a.y <= point.y && point.y <= b.y
    } else if a.x == b.x && point.x == a.x && a.y > b.y {
        b.y <= point.y && point.y < a.y
    } else if a.y == b.y && point.y == a.y && a.x < b.x {
        a.x <= point.x && point.x < b.x
    } else if a.y == b.y && point.y == a.y && a.x > b.x {
        b.x <= point.x && point.x < a.x
    } else {
        point == a || point == b
    }
}

fn are_vertical(a: &U64Vec2, b: &U64Vec2) -> bool {
    a.x == b.x
}

fn are_horizontal(a: &U64Vec2, b: &U64Vec2) -> bool {
    a.y == b.y
}

fn edge_is_crossed(a: &U64Vec2, b: &U64Vec2, edges: &Vec<(&U64Vec2, &U64Vec2)>) -> bool {
    edges.iter().any(|edge| {
        let res = if are_vertical(a, b) == are_vertical(edge.0, edge.1) {
            false
        } else if are_vertical(a, b) {
            if edge.0.x == a.x || edge.1.x == a.x {
                false
            } else {
                (edge.0.x > a.x) != (edge.1.x > a.x) && (a.y > edge.0.y) != (b.y > edge.0.y)
            }
        } else {
            if edge.0.y == a.y || edge.1.y == a.y {
                false
            } else {
                (edge.0.y > a.y) != (edge.1.y > a.y) && (a.x > edge.0.x) != (b.x > edge.0.x)
            }
        };
        if res {
            info!(?a, ?b, ?edge);
        }
        res
    })
}

fn test_edge(a: &U64Vec2, b: &U64Vec2, edges: &Vec<(&U64Vec2, &U64Vec2)>) -> bool {
    if a.x == b.x && a.y < b.y {
        (a.y..=b.y)
            .map(|y| a.with_y(y))
            .all(|v| point_in_polygon(&v, edges))
    } else if a.x == b.x && a.y > b.y {
        (b.y..=a.y)
            .map(|y| a.with_y(y))
            .all(|v| point_in_polygon(&v, edges))
    } else if a.y == b.y && a.x < b.x {
        (a.x..=b.x)
            .map(|x| a.with_x(x))
            .all(|v| point_in_polygon(&v, edges))
    } else if a.y == b.y && a.x > b.x {
        (b.x..=a.x)
            .map(|x| a.with_x(x))
            .all(|v| point_in_polygon(&v, edges))
    } else if a == b {
        true
    } else {
        panic!("not a valid option a:{:?}, b:{:?}", a, b);
    }
}


fn can_be_changed<'a>(
    edges: &'a Vec<(&U64Vec2, &U64Vec2)>,
) -> impl Fn(&(&U64Vec2, &U64Vec2)) -> bool + 'a {
    move |(a, b)| {
        let c = &a.with_x(b.x);
        let d = &a.with_y(b.y);
        if !point_in_polygon(c, edges) || !point_in_polygon(d, edges) ||
         a.x == b.x || a.y == b.y {
            false
        } else {
            let res = [a, c, b, d]
                .iter()
                .circular_tuple_windows()
                .all(|(p1, p2)| {
                    test_edge(p1, p2, edges)
                });
            res
        }
    }
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u64 {
    let (_, points) = parse(input).expect("Error in parsing");

    let edges = points
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&U64Vec2, &U64Vec2)>>();

    points
        .iter()
        .tuple_combinations()
        .filter(can_be_changed(&edges))
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("Iterator should not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let expected = 50;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let expected = 24;

        assert_eq!(func2(input), expected);
    }
}
