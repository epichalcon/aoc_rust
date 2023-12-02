use core::panic;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, time::Instant};

#[derive(PartialEq, Eq)]
struct Operation<'a> {
    input1: Option<&'a str>,
    input2: &'a str,
    operation: &'a str,
    output: &'a str,
}

impl Operation<'_> {
    fn parse(input: &str) -> Operation {
        let parts = input.split_whitespace().collect::<Vec<&str>>();

        if parts.len() == 3 {
            Operation {
                input1: None,
                operation: "NOP",
                input2: parts[0],
                output: parts[2],
            }
        } else if parts.len() == 4 {
            Operation {
                input1: None,
                operation: parts[0],
                input2: parts[1],
                output: parts[3],
            }
        } else if parts.len() == 5 {
            Operation {
                input1: Some(parts[0]),
                operation: parts[1],
                input2: parts[2],
                output: parts[4],
            }
        } else {
            panic!("Operation input not correct")
        }
    }
}
impl Debug for Operation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f,
                "{} {} {} -> {}",
                self.input1.unwrap_or("_"),
                self.operation,
                self.input2,
                self.output
        }
    }
}

#[derive(Debug)]
struct Machine {
    wires: HashMap<String, u16>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            wires: HashMap::new(),
        }
    }

    fn get_wire_value(&self, wire: &str) -> u16 {
        if self.wires.contains_key(wire) {
            *self.wires.get(wire).unwrap()
        } else {
            wire.parse::<u16>().unwrap_or(0)
        }
    }

    fn perform_operation(&mut self, operation: &Operation) {
        let result: u16 = match operation.operation {
            "AND" => {
                self.get_wire_value(operation.input1.unwrap())
                    & self.get_wire_value(operation.input2)
            }
            "OR" => {
                self.get_wire_value(operation.input1.unwrap())
                    | self.get_wire_value(operation.input2)
            }
            "LSHIFT" => {
                self.get_wire_value(operation.input1.unwrap())
                    << self.get_wire_value(operation.input2)
            }
            "RSHIFT" => {
                self.get_wire_value(operation.input1.unwrap())
                    >> self.get_wire_value(operation.input2)
            }
            "NOT" => !self.get_wire_value(operation.input2),
            "NOP" => self.get_wire_value(operation.input2),
            _ => panic!("operation not recognized"),
        };
        self.wires.insert(String::from(operation.output), result);
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", calculate_wire_value(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", rewire(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn calculate_wire_value(input: &str) -> u16 {
    // parsear el input e introducirlo en un mapa
    let mut machine = Machine::new();

    let operations =
        input
            .lines()
            .map(Operation::parse)
            .fold(HashMap::new(), |mut accum, operation| {
                accum.insert(operation.output, operation);
                accum
            });

    let mut wires: Vec<_> = operations.keys().map(|k| *k).collect::<Vec<&str>>();

    wires.sort();

    let singular = wires
        .iter()
        .filter(|w| w.len() == 1)
        .map(|w| operations.get(w).unwrap());

    let plural = wires
        .iter()
        .filter(|w| w.len() == 2)
        .map(|w| operations.get(w).unwrap());

    singular
        .chain(plural)
        .skip(1)
        .for_each(|op| machine.perform_operation(op));

    machine.perform_operation(operations.get("a").unwrap());

    machine.get_wire_value("a")
}

fn rewire(input: &str) -> u16 {
    let a_result = calculate_wire_value(input);
    // parsear el input e introducirlo en un mapa
    let mut machine = Machine::new();

    let mut operations =
        input
            .lines()
            .map(Operation::parse)
            .fold(HashMap::new(), |mut accum, operation| {
                accum.insert(operation.output, operation);
                accum
            });

    let resulting_operation = Operation {
        input1: None,
        operation: "NOP",
        input2: &a_result.to_string(),
        output: "b",
    };

    operations.insert("b", resulting_operation);

    let mut wires: Vec<_> = operations.keys().map(|k| *k).collect::<Vec<&str>>();

    wires.sort();

    let singular = wires
        .iter()
        .filter(|w| w.len() == 1)
        .map(|w| operations.get(w).unwrap());

    let plural = wires
        .iter()
        .filter(|w| w.len() == 2)
        .map(|w| operations.get(w).unwrap());

    singular
        .chain(plural)
        .skip(1)
        .for_each(|op| machine.perform_operation(op));

    machine.perform_operation(operations.get("a").unwrap());

    machine.get_wire_value("a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_one_line_assignation() {
        let result = Operation::parse("123 -> x");

        let conection = Operation {
            input1: None,
            input2: "123",
            operation: "NOP",
            output: "x",
        };

        assert_eq!(conection, result);
    }

    #[test]
    fn test_parser_one_line_not() {
        let result = Operation::parse("NOT y -> x");

        let conection = Operation {
            input1: None,
            input2: "y",
            operation: "NOT",
            output: "x",
        };

        assert_eq!(conection, result);
    }

    #[test]
    fn test_parser_one_line_and() {
        let result = Operation::parse("z AND y -> x");

        let conection = Operation {
            input1: Some("z"),
            input2: "y",
            operation: "AND",
            output: "x",
        };

        assert_eq!(conection, result);
    }

    #[test]
    fn full_test() {
        let circuit = "123 -> x
NOT 65079 -> y
x AND y -> a";
        let result = calculate_wire_value(circuit);

        assert_eq!(72, result);
    }
}
