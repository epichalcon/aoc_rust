use nom::combinator::value;
use rstest::rstest;
use std::{collections::BTreeMap, time::Instant};

enum Operation {
    Add(String, u32),
    Remove(String),
}

#[derive(Debug, Clone)]
struct LensBox {
    lens_map: BTreeMap<String, usize>,
    lens_array: Vec<u32>,
}

impl LensBox {
    fn new() -> Self {
        LensBox {
            lens_map: BTreeMap::new(),
            lens_array: Vec::new(),
        }
    }

    fn introduce_lens(&mut self, label: &str, focal_length: u32) {
        match self.lens_map.get(label) {
            Some(index) => {
                self.lens_array[*index] = focal_length;
            }
            None => {
                self.lens_map
                    .insert(label.to_string(), self.lens_array.len());
                self.lens_array.push(focal_length);
            }
        }
    }

    fn remove_lens(&mut self, label: &str) {
        if self.lens_map.contains_key(label) {
            let index = self.lens_map.get(label).unwrap().clone();
            self.lens_array.remove(index);
            self.lens_map.remove(label);
            let cloned_map = self.lens_map.clone();
            for (key, value) in cloned_map.iter() {
                if value > &index {
                    self.lens_map.insert(key.to_string(), value - 1);
                }
            }
        }
    }

    fn get_lens(&self, label: &str) -> Option<(usize, u32)> {
        match self.lens_map.get(label) {
            Some(index) => {
                let focal = self.lens_array[*index];
                Some((*index + 1, focal))
            }
            None => None,
        }
    }

    fn get_all(&self) -> Vec<(usize, u32)> {
        self.lens_map
            .values()
            .map(|index| {
                let focal = self.lens_array[*index];
                (*index + 1, focal)
            })
            .collect()
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input.split(",").map(holyday_hash).sum::<u32>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", configure_lenses(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn holyday_hash(input: &str) -> u32 {
    input
        .replace("\n", "")
        .chars()
        .fold(0, |mut current_value, ch| {
            current_value += ch as u32;
            current_value *= 17;
            current_value %= 256;
            current_value
        })
}
fn configure_lenses(input: &str) -> usize {
    //1. calcular el hash de la etiqueta

    let boxes = input
        .split(",")
        .fold(vec![LensBox::new(); 256], |mut boxes, instruction| {
            match process_instruction(instruction) {
                Operation::Add(lable, focal) => {
                    let hash = holyday_hash(lable.as_str());
                    boxes[hash as usize].introduce_lens(&lable, focal);
                }
                Operation::Remove(lable) => {
                    let hash = holyday_hash(lable.as_str());
                    boxes[hash as usize].remove_lens(&lable);
                }
            }
            boxes
        });

    let mut total = 0;
    for (i, lens_box) in boxes.iter().enumerate() {
        for (slot, focal) in lens_box.get_all() {
            total += (i + 1) * slot * focal as usize;
        }
    }

    total
}

fn process_instruction(input: &str) -> Operation {
    if input.contains("=") {
        let mut splitted = input.split("=");
        Operation::Add(
            splitted.next().unwrap().to_string(),
            splitted.next().unwrap().parse::<u32>().unwrap(),
        )
    } else {
        Operation::Remove(input[..input.find("-").unwrap()].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(52, "HASH")]
    #[case(30, "rn=1")]
    #[case(253, "cm-")]
    #[case(97, "qp=3")]
    #[case(47, "cm=2")]
    #[case(14, "qp-")]
    #[case(180, "pc=4")]
    #[case(9, "ot=9")]
    #[case(197, "ab=5")]
    #[case(48, "pc-")]
    #[case(214, "pc=6")]
    #[case(231, "ot=7")]
    fn test_hollyday_hash_cases(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, holyday_hash(input));
    }

    #[test]
    fn test_hollyday_hash_complete() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected = 1320;
        assert_eq!(expected, input.split(",").map(holyday_hash).sum::<u32>())
    }

    #[test]
    fn test_LensBox() {
        let mut lens_box = LensBox::new();

        lens_box.introduce_lens("a", 3);
        lens_box.introduce_lens("b", 4);

        assert_eq!(Some((1, 3)), lens_box.get_lens("a"));
        assert_eq!(Some((2, 4)), lens_box.get_lens("b"));
        assert_eq!(None, lens_box.get_lens("c"));

        lens_box.introduce_lens("a", 2);
        assert_eq!(Some((1, 2)), lens_box.get_lens("a"));

        lens_box.remove_lens("c");
        assert_eq!(Some((1, 2)), lens_box.get_lens("a"));
        assert_eq!(Some((2, 4)), lens_box.get_lens("b"));
        lens_box.remove_lens("a");
        assert_eq!(None, lens_box.get_lens("a"));
    }

    #[test]
    fn test_configure_lenses() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected = 145;
        assert_eq!(expected, configure_lenses(input));
    }
}
