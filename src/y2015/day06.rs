use crate::utils::parser;
use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    On,
    Off,
    Toggle,
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", lights_on(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", lights_gradual(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn lights_on(input: &str) -> usize {
    let mut on_lights: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        let instruction = {
            if line.starts_with("turn on") {
                Instruction::On
            } else if line.starts_with("turn off") {
                Instruction::Off
            } else {
                Instruction::Toggle
            }
        };

        on_lights = apply_lights_instructions(
            instruction,
            parser::get_i32_from_line(line),
            on_lights.clone(),
        );
    }

    on_lights
        .concat()
        .into_iter()
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len()
}

fn apply_lights_instructions(
    instruction: Instruction,
    corners: Vec<i32>,
    mut lights: Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    match instruction {
        Instruction::On => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    lights[i as usize][j as usize] = true;
                }
            }
        }
        Instruction::Off => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    lights[i as usize][j as usize] = false;
                }
            }
        }
        Instruction::Toggle => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    lights[i as usize][j as usize] = !lights[i as usize][j as usize];
                }
            }
        }
    }
    lights
}

fn lights_gradual(input: &str) -> u32 {
    let mut on_lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let instruction = {
            if line.starts_with("turn on") {
                Instruction::On
            } else if line.starts_with("turn off") {
                Instruction::Off
            } else {
                Instruction::Toggle
            }
        };

        on_lights = apply_gradual_lights_instructions(
            instruction,
            parser::get_i32_from_line(line),
            on_lights.clone(),
        );
    }

    on_lights.concat().into_iter().sum::<u32>()
}
fn apply_gradual_lights_instructions(
    instruction: Instruction,
    corners: Vec<i32>,
    mut lights: Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    match instruction {
        Instruction::On => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    lights[i as usize][j as usize] += 1;
                }
            }
        }
        Instruction::Off => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    if lights[i as usize][j as usize] != 0 {
                        lights[i as usize][j as usize] -= 1;
                    }
                }
            }
        }
        Instruction::Toggle => {
            for i in corners[0]..corners[2] + 1 {
                for j in corners[1]..corners[3] + 1 {
                    lights[i as usize][j as usize] += 2;
                }
            }
        }
    }

    lights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_on() {
        assert_eq!(2, lights_on("turn on 0,0 through 0,1"));
    }

    #[test]
    fn test_lights_on_repeted() {
        assert_eq!(
            2,
            lights_on("turn on 0,0 through 0,1\nturn on 0,0 through 0,1")
        );
    }

    #[test]
    fn test_lights_off() {
        assert_eq!(
            1,
            lights_on("turn on 0,0 through 0,1\nturn off 0,1 through 0,1")
        );
    }
    #[test]
    fn test_lights_off_repeted() {
        assert_eq!(
            1,
            lights_on(
                "turn on 0,0 through 0,1\nturn off 0,1 through 0,1\nturn off 0,1 through 0,2"
            )
        );
    }
    #[test]
    fn test_lights_toggle_on() {
        assert_eq!(2, lights_on("toggle 0,0 through 0,1"));
    }

    #[test]
    fn test_lights_toggle_off() {
        assert_eq!(
            1,
            lights_on("toggle 0,0 through 0,1\n toggle 0,1 through 0,1")
        );
    }

    #[test]
    fn test_lights_on_final() {
        assert_eq!(1000000, lights_on("turn on 0,0 through 999,999"));

        assert_eq!(
            1000000 - 1000,
            lights_on("turn on 0,0 through 999,999\nturn off 0,0 through 999,0")
        );

        assert_eq!(
            1000000 - 1000 - 4,
            lights_on("turn on 0,0 through 999,999\nturn off 0,0 through 999,0\ntoggle 499,499 through 500,500")
        );
    }

    #[test]
    fn test_gradual_one() {
        assert_eq!(1, lights_gradual("turn on 0,0 through 0,0"));
    }

    #[test]
    fn test_gradual_many() {
        assert_eq!(2000000, lights_gradual("toggle 0,0 through 999,999"));
    }
}
