use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    time::Instant,
};

use nom::{
    character::{
        self,
        complete::{char, newline, one_of},
    },
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult, Parser,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Button = Vec<u32>;
type Buttons = Vec<Button>;
type LightDiagram = Diagram<bool>;
type JoltDiagram = Diagram<u32>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Diagram<T> {
    diagram: Vec<T>,
}

impl<T: Modify + Clone + Default> Diagram<T> {
    fn new(diagram: Vec<T>) -> Self {
        Self { diagram }
    }

    fn len(&self) -> usize {
        self.diagram.len()
    }
}

impl<T: Clone + Default> Diagram<T> {
    fn default(len: usize) -> Self {
        Self {
            diagram: vec![T::default(); len],
        }
    }
}

impl<T: Modify> Diagram<T> {
    fn press_button(&mut self, button: &Button) {
        for pos in button {
            let pos = *pos as usize;
            if pos > self.diagram.len() {
                panic!("button index out of bounds")
            }
            self.diagram[pos] = self.diagram[pos].modify()
        }
    }

    fn should_continue(&self, objective: &Self) -> bool {
        self.diagram
            .iter()
            .zip(&objective.diagram)
            .all(|(a, ob)| a.can_arrive_to(ob))
    }
}

impl<T: Modify + Hash + Clone + Default + Eq + PartialEq> Diagram<T> {
    fn match_objective(objective: Self, buttons: Buttons) -> u32 {
        let mut visited: HashSet<Self> = HashSet::new();
        let mut queue: VecDeque<(Self, &Vec<u32>, u32)> = VecDeque::new();

        for new_button in &buttons {
            queue.push_back((Diagram::default(objective.len()), new_button, 0));
        }

        visited.insert(Diagram::default(objective.len()));

        while let Some((mut lights, button, mut pressed_before)) = queue.pop_front() {
            lights.press_button(button);
            pressed_before += 1;

            if visited.contains(&lights) || !lights.should_continue(&objective) {
                continue;
            }
            visited.insert(lights.clone());

            if lights == objective {
                return pressed_before;
            }

            for new_button in &buttons {
                queue.push_back((lights.clone(), new_button, pressed_before));
            }
        }
        panic!("should not be reachable");
    }
}

trait Modify {
    fn modify(&self) -> Self;
    fn can_arrive_to(&self, other: &Self) -> bool;
}

impl Modify for bool {
    fn modify(&self) -> Self {
        !self
    }
    fn can_arrive_to(&self, _other: &Self) -> bool {
        true
    }
}

impl Modify for u32 {
    fn modify(&self) -> Self {
        self + 1
    }
    fn can_arrive_to(&self, other: &Self) -> bool {
        self <= other
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn lights_parser(input: &str) -> IResult<&str, LightDiagram> {
    let (input, lights) = delimited(
        char('['),
        many1(one_of(".#").map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!(),
        })),
        char(']'),
    )(input)?;
    Ok((input, Diagram::new(lights)))
}

fn button_parser(input: &str) -> IResult<&str, Buttons> {
    delimited(
        char(' '),
        separated_list1(
            char(' '),
            delimited(
                char('('),
                separated_list0(char(','), character::complete::u32),
                char(')'),
            ),
        ),
        char(' '),
    )(input)
}

fn joltage_parser(input: &str) -> IResult<&str, JoltDiagram> {
    delimited(
        char('{'),
        separated_list0(char(','), character::complete::u32).map(Diagram::new),
        char('}'),
    )(input)
}

fn machine_parser(input: &str) -> IResult<&str, (LightDiagram, Buttons, JoltDiagram)> {
    let (input, lights) = lights_parser(input)?;
    let (input, buttons) = button_parser(input)?;
    let (input, joltage) = joltage_parser(input)?;

    Ok((input, (lights, buttons, joltage)))
}

fn parse(input: &str) -> IResult<&str, Vec<(LightDiagram, Buttons, JoltDiagram)>> {
    separated_list1(newline, machine_parser)(input)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u32 {
    let (_, machines) = parse(input).expect("parsing failed");
    machines
        .iter()
        .map(|(lights, buttons, _)| Diagram::match_objective(lights.clone(), buttons.to_vec()))
        .sum()
}

fn is_near(a: &JoltDiagram, b: &JoltDiagram) -> bool {
    a.diagram
        .iter()
        .zip(b.diagram.clone())
        .any(|(a, b)| *a + 10 > b)
}

fn is_button_available(i: usize, mask: u32) -> bool {
    mask & (1 << i) > 0
}

fn next_combination(combinations: &mut Vec<u32>) -> bool {
    let i = combinations.iter().rposition(|&v| v != 0).unwrap();
    let l = combinations.len();
    if i == 0 {
        return false;
    }
    let v = combinations[i];
    combinations[i - 1] += 1;
    combinations[i] = 0;
    combinations[l - 1] = v - 1;
    true
}

fn dfs_part2(joltage: &Vec<u32>, available_buttons_mask: u32, buttons: &Vec<Vec<u32>>) -> usize {
    if joltage.iter().all(|j| *j == 0) {
        return 0;
    }

    // Important optimization: Find the joltage value with the lowest number of
    // combinations of buttons to try. This allows us to prune branches as early
    // as possible.
    // Second optimization (not so important, but still quite good): If multiple
    // joltage values are affected by the same number of buttons, select the
    // highest value
    let (mini, &min) = joltage
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .min_by_key(|&(i, &v)| {
            (
                // lowest number of buttons
                buttons
                    .iter()
                    .enumerate()
                    .filter(|&(j, b)| {
                        is_button_available(j, available_buttons_mask) && b.contains(&(i as u32))
                    })
                    .count(),
                // highest joltage value (negative because we're using `min_by_key`)
                -(v as isize),
            )
        })
        .unwrap();

    // get the buttons that affect the joltage value at position `mini`
    let matching_buttons = buttons
        .iter()
        .enumerate()
        .filter(|&(i, b)| {
            is_button_available(i, available_buttons_mask) && b.contains(&(mini as u32))
        })
        .collect::<Vec<_>>();

    let mut result = usize::MAX;

    if !matching_buttons.is_empty() {
        // create new mask so only those buttons remain that do not affect the
        // joltage value at position `mini`
        let mut new_mask = available_buttons_mask;
        for (i, _) in &matching_buttons {
            new_mask &= !(1 << i);
        }

        // try all combinations of matching buttons
        let mut new_joltage = joltage.to_vec();
        let mut counts = vec![0; matching_buttons.len()];
        counts[matching_buttons.len() - 1] = min;
        loop {
            // count down joltage values and make sure we don't press a button
            // too often (i.e. that the number of button presses is not higher
            // than any of the values to decrease)
            let mut good = true;
            new_joltage.copy_from_slice(joltage);
            'buttons: for (bi, &cnt) in counts.iter().enumerate() {
                if cnt == 0 {
                    continue;
                }
                for &j in matching_buttons[bi].1 {
                    if new_joltage[j as usize] >= cnt {
                        new_joltage[j as usize] -= cnt;
                    } else {
                        good = false;
                        break 'buttons;
                    }
                }
            }

            if good {
                // recurse with decreased joltage values and with remaining buttons
                let r = dfs_part2(&new_joltage, new_mask, buttons);
                if r != usize::MAX {
                    result = result.min((min as usize) + r);
                }
            }

            // try next combination
            if !next_combination(&mut counts) {
                break;
            }
        }
    }

    result
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> usize {
    let (_, machines) = parse(input).expect("parsing failed");
    let results: Vec<_> = machines
        .par_iter()
        .map(|(_, buttons, joltage)| {
            let res = dfs_part2(&joltage.diagram, (1 << buttons.len()) - 1, buttons);
            dbg!(res);
            res
        })
        .collect();

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    /*
    #[rstest]
    #[case()]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {}
    */

    #[test]
    fn test_func1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let expected = 7;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let expected = 33;

        assert_eq!(func2(input), expected);
    }
}
