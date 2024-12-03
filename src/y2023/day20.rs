use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric0, newline},
    multi::{fold_many1, separated_list0},
    sequence::terminated,
    IResult,
};
use rstest::rstest;
use std::{
    collections::{HashMap, VecDeque},
    os::unix::raw::uid_t,
    time::Instant,
    usize,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjuntion,
    Broadcast,
    End,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    input: HashMap<String, bool>,
    module_type: ModuleType,
    out: Vec<(bool, String)>,
}

impl Module {
    fn process_signal(&mut self, signal: bool, other: String) -> bool {
        match &self.module_type {
            ModuleType::Broadcast => {
                self.out.iter_mut().for_each(|out| out.0 = signal);
                signal
            }
            ModuleType::FlipFlop => {
                if !signal {
                    self.out.iter_mut().for_each(|out| out.0 = !out.0);
                    true
                } else {
                    false
                }
            }
            ModuleType::Conjuntion => {
                self.input.insert(other, signal);
                let out_signal = self.input.values().any(|value| !value);

                self.out.iter_mut().for_each(|out| out.0 = out_signal);
                out_signal
            }
            ModuleType::End => false,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Module>> {
    fold_many1(
        terminated(parse_module, newline),
        HashMap::new,
        |mut acc, (name, module)| {
            acc.insert(name, module);
            acc
        },
    )(input)
}

fn parse_module(input: &str) -> IResult<&str, (String, Module)> {
    let (input, name_type) = take_until(" -> ")(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, out) = separated_list0(tag(", "), alphanumeric0)(input)?;
    let mod_type = match name_type.chars().next().unwrap() {
        '%' => ModuleType::FlipFlop,
        '&' => ModuleType::Conjuntion,
        'b' => ModuleType::Broadcast,
        _ => panic!(),
    };

    let name = match mod_type {
        ModuleType::Broadcast => name_type.to_string(),
        _ => name_type[1..].to_string(),
    };

    Ok((
        input,
        (
            name,
            Module {
                input: HashMap::new(),
                module_type: mod_type,
                out: out
                    .iter()
                    .fold(Vec::<(bool, String)>::new(), |mut acc, out| {
                        acc.push((false, out.to_string()));
                        acc
                    })
                    .clone(),
            },
        ),
    ))
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> usize {
    let (_, mut modules) = parse_input(input).expect("Input malformed");

    let mut ins: HashMap<String, Vec<String>> = HashMap::new();

    for (name, module) in &modules {
        for (_, out_names) in &module.out {
            let list_of_in = ins.entry(out_names.clone()).or_insert(vec![]);
            list_of_in.push(name.clone());
        }
    }

    for (module_name, input_modules) in ins {
        let mut actual_module = (*modules.get(&module_name).unwrap_or(&Module {
            input: HashMap::new(),
            module_type: ModuleType::End,
            out: Vec::new(),
        }))
        .clone();

        actual_module.input = input_modules
            .iter()
            .fold(HashMap::new(), |mut acc, input_module| {
                acc.insert(input_module.to_string(), false);
                acc
            });

        modules.insert(module_name, actual_module);
    }

    let mut i = 0;

    let mut low_total = 0;
    let mut high_total = 0;

    while i < 1000 {
        let (low, high, new_modules) = press_button(modules);
        low_total += low;
        high_total += high;
        modules = new_modules;
        i += 1;
    }

    low_total * high_total
}

fn press_button(mut modules: HashMap<String, Module>) -> (usize, usize, HashMap<String, Module>) {
    let mut q = VecDeque::new();

    q.push_back((false, "broadcaster".to_string(), "broadcaster".to_string()));

    let mut low_pulses = 1;
    let mut high_pulses = 0;

    while !q.is_empty() {
        let (in_signal, module_name, previous) = q.pop_front().unwrap();
        let mut actual_module = (*modules
            .get(&module_name)
            .expect(format!("modules does not have {}", module_name).as_str()))
        .clone();

        if actual_module.module_type == ModuleType::End {
            continue;
        }

        let _ = actual_module.process_signal(in_signal, previous);
        modules.insert(module_name.clone(), actual_module.clone());

        if actual_module.module_type == ModuleType::FlipFlop && in_signal == true {
            continue;
        }

        for (out_signal, _) in &actual_module.out {
            if *out_signal {
                high_pulses += 1
            } else {
                low_pulses += 1
            }
        }

        for out_singal in &actual_module.out {
            q.push_back((out_singal.0, out_singal.1.clone(), module_name.clone()));
        }
    }

    (low_pulses, high_pulses, modules)
}

fn func2(input: &str) -> usize {
    let (_, mut modules) = parse_input(input).expect("Input malformed");

    let mut ins: HashMap<String, Vec<String>> = HashMap::new();

    for (name, module) in &modules {
        for (_, out_names) in &module.out {
            let list_of_in = ins.entry(out_names.clone()).or_insert(vec![]);
            list_of_in.push(name.clone());
        }
    }

    for (module_name, input_modules) in ins {
        let mut actual_module = (*modules.get(&module_name).unwrap_or(&Module {
            input: HashMap::new(),
            module_type: ModuleType::End,
            out: Vec::new(),
        }))
        .clone();

        actual_module.input = input_modules
            .iter()
            .fold(HashMap::new(), |mut acc, input_module| {
                acc.insert(input_module.to_string(), false);
                acc
            });

        modules.insert(module_name, actual_module);
    }

    let mut i = 0;

    loop {
        let (pulse, new_modules) = press_button_2(modules);

        if pulse {
            break;
        }
        modules = new_modules;
        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {

    use num::Float;

    use super::*;

    #[rstest]
    #[case(
        32000000,
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"
    )]
    #[case(
        11687500,
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"
    )]
    fn test_func1(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func1(input));
    }

    #[test]
    fn test_processing_ff() {
        let mut input = HashMap::new();
        input.insert("broadcast".to_string(), false);

        let mut module = Module {
            input: input.clone(),
            module_type: ModuleType::FlipFlop,
            out: vec![(false, "c".to_string())],
        };

        let expected = Module {
            input: input.clone(),
            module_type: ModuleType::FlipFlop,
            out: vec![(true, "c".to_string())],
        };

        let res = module.process_signal(false, "broadcast".to_string());

        assert!(res);

        assert_eq!(expected, module);

        let expected = Module {
            input,
            module_type: ModuleType::FlipFlop,
            out: vec![(false, "c".to_string())],
        };

        let res = module.process_signal(false, "broadcast".to_string());

        assert_eq!(expected, module);
    }

    #[test]
    fn test_processing_con_1() {
        let mut input = HashMap::new();
        input.insert("broadcast".to_string(), false);

        let mut module = Module {
            input: input.clone(),
            module_type: ModuleType::Conjuntion,
            out: vec![(false, "c".to_string())],
        };

        let expected = Module {
            input,
            module_type: ModuleType::Conjuntion,
            out: vec![(true, "c".to_string())],
        };

        let res = module.process_signal(false, "broadcast".to_string());

        assert!(res);

        assert_eq!(expected, module);
    }

    #[test]
    fn test_processing_con_2() {
        let mut input = HashMap::new();
        input.insert("a".to_string(), false);
        input.insert("b".to_string(), false);

        let mut module = Module {
            input: input.clone(),
            module_type: ModuleType::Conjuntion,
            out: vec![(false, "c".to_string())],
        };

        input.insert("a".to_string(), true);

        let expected = Module {
            input,
            module_type: ModuleType::Conjuntion,
            out: vec![(true, "c".to_string())],
        };

        let res = module.process_signal(true, "a".to_string());

        assert!(res);

        assert_eq!(expected, module);
    }

    #[test]
    fn test_processing_con_3() {
        let mut input = HashMap::new();
        input.insert("a".to_string(), true);
        input.insert("b".to_string(), true);

        let mut module = Module {
            input: input.clone(),
            module_type: ModuleType::Conjuntion,
            out: vec![(true, "c".to_string())],
        };

        input.insert("a".to_string(), true);

        let expected = Module {
            input,
            module_type: ModuleType::Conjuntion,
            out: vec![(false, "c".to_string())],
        };

        let res = module.process_signal(true, "a".to_string());

        assert!(!res);

        assert_eq!(expected, module);
    }
}
