use std::cell::RefCell;
use std::net::ToSocketAddrs;
use std::{collections::HashMap, sync::atomic::compiler_fence, time::Instant};

#[derive(Eq, PartialEq, Debug, Clone)]
enum Operation {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Nop,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Connection {
    input1: String,
    input2: String,
    operation: Operation,
}

fn parse_input(input: &str) -> HashMap<String, RefCell<Connection>> {
    let mut circuit = HashMap::<String, RefCell<Connection>>::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if parts.len() == 3 {
            let conection = Connection {
                input1: parts[0].to_string(),
                input2: "".to_string(),
                operation: { Operation::Nop },
            };

            let _ = circuit.insert(parts[2].to_string(), conection.into());
        } else if parts.len() == 4 {
            let conection = Connection {
                input1: parts[1].to_string(),
                input2: "".to_string(),
                operation: {
                    match parts[0] {
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        "LSHIFT" => Operation::Lshift,
                        "RSHIFT" => Operation::Rshift,
                        "NOT" => Operation::Not,
                        _ => panic!("no operation recognized"),
                    }
                },
            };

            circuit.insert(parts[3].to_string(), conection.into());
        } else if parts.len() == 5 {
            let conection = Connection {
                input1: parts[0].to_string(),
                input2: parts[2].to_string(),
                operation: {
                    match parts[1] {
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        "LSHIFT" => Operation::Lshift,
                        "RSHIFT" => Operation::Rshift,
                        "NOT" => Operation::Not,
                        _ => panic!("no operation recognized"),
                    }
                },
            };

            circuit.insert(parts[4].to_string(), conection.into());
        }
    }

    circuit
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", calculate_wire_value(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2());
    println!("\t time:{:?}", start_time.elapsed());
}

fn calculate_wire_value(input: &str) -> String {
    // parsear el input e introducirlo en un mapa

    let circuit = parse_input(input);

    // empezar desde el a e ir hacia atras hasta que se calcule todo lo necesario
    let resolved_circuit = resolve_circuit(circuit, "a".to_string());

    // devolver el valor de a

    let connection = resolved_circuit.get("a").unwrap().borrow();
    connection.clone().input1
}

fn resolve_circuit<'a>(
    mut circuit: HashMap<String, RefCell<Connection>>,
    current_key: String,
) -> HashMap<String, RefCell<Connection>> {
    let mut circuit_2 = circuit.clone();
    let mut circuit_1 = circuit.clone();
    {
        let connection = circuit.get(&current_key).unwrap().borrow();
        let input1 = connection.input1.clone();

        if !input1.chars().all(|c| c.is_digit(10)) {
            let input1 = connection.input1.clone();
            circuit_1 = resolve_circuit(circuit.clone(), input1);
        }

        if !input1.chars().all(|c| c.is_digit(10)) {
            let input2 = connection.input2.clone();
            circuit_2 = resolve_circuit(circuit_1.clone(), input2);
        }

        let connection_result = Connection {
            input1: execute_instruction(connection.clone()),
            input2: "".to_string(),
            operation: Operation::Nop,
        };

        circuit_2.insert(current_key, connection_result.into());
    }

    circuit = circuit_2.clone();
    circuit
}

fn execute_instruction(connection: Connection) -> String {
    match connection.operation {
        Operation::And => {
            let in1 = connection.input1.parse::<u16>().unwrap();
            let in2 = connection.input2.parse::<u16>().unwrap();
            let result = in1 & in2;
            result.to_string()
        }
        Operation::Or => {
            let in1 = connection.input1.parse::<u16>().unwrap();
            let in2 = connection.input2.parse::<u16>().unwrap();
            let result = in1 | in2;
            result.to_string()
        }
        Operation::Lshift => {
            let in1 = connection.input1.parse::<u16>().unwrap();
            let in2 = connection.input2.parse::<u16>().unwrap();
            let result = in1 << in2;
            result.to_string()
        }
        Operation::Rshift => {
            let in1 = connection.input1.parse::<u16>().unwrap();
            let in2 = connection.input2.parse::<u16>().unwrap();
            let result = in1 >> in2;
            result.to_string()
        }
        Operation::Not => {
            let in1 = connection.input1.parse::<u16>().unwrap();
            let result = !in1;
            result.to_string()
        }
        Operation::Nop => "".to_string(),
    }
}

fn func2() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_one_line_assignation() {
        let result = parse_input("123 -> x");
        assert!(result.contains_key("x"));

        let conection = Connection {
            input1: "123".to_string(),
            input2: "".to_string(),
            operation: { Operation::Nop },
        };

        let resulting_connection = result.get("x").unwrap().borrow();

        assert_eq!(conection, resulting_connection.clone());
    }

    #[test]
    fn test_parser_one_line_not() {
        let result = parse_input("NOT y -> x");
        assert!(result.contains_key("x"));

        let conection = Connection {
            input1: "y".to_string(),
            input2: "".to_string(),
            operation: { Operation::Not },
        };

        let resulting_connection = result.get("x").unwrap().borrow();

        assert_eq!(conection, resulting_connection.clone())
    }

    #[test]
    fn test_parser_one_line_and() {
        let result = parse_input("z AND y -> x");
        assert!(result.contains_key("x"));

        let conection = Connection {
            input1: "z".to_string(),
            input2: "y".to_string(),
            operation: { Operation::And },
        };

        let resulting_connection = result.get("x").unwrap().borrow();

        assert_eq!(conection, resulting_connection.clone());
    }

    #[test]
    fn test_parser_multiple_lines() {
        let result = parse_input("123 -> a\nz AND y -> x");
        assert!(result.contains_key("x"));
        assert!(result.contains_key("a"));

        let conection_x = Connection {
            input1: "z".to_string(),
            input2: "y".to_string(),
            operation: { Operation::And },
        };

        let conection_a = Connection {
            input1: "123".to_string(),
            input2: "".to_string(),
            operation: { Operation::Nop },
        };

        assert_eq!(conection_x, *result.get("x").unwrap().borrow());
        assert_eq!(conection_a, *result.get("a").unwrap().borrow());
    }

    #[test]
    fn test_execute_instruction_and() {
        let conection = Connection {
            input1: "123".to_string(),
            input2: "456".to_string(),
            operation: { Operation::And },
        };

        assert_eq!("72", execute_instruction(conection));
    }
    #[test]
    fn test_execute_instruction_or() {
        let conection = Connection {
            input1: "123".to_string(),
            input2: "456".to_string(),
            operation: { Operation::Or },
        };

        assert_eq!("507", execute_instruction(conection));
    }
    #[test]
    fn test_execute_instruction_lshift() {
        let conection = Connection {
            input1: "123".to_string(),
            input2: "2".to_string(),
            operation: { Operation::Lshift },
        };

        assert_eq!("492", execute_instruction(conection));
    }

    #[test]
    fn test_execute_instruction_rshift() {
        let conection = Connection {
            input1: "456".to_string(),
            input2: "2".to_string(),
            operation: { Operation::Rshift },
        };

        assert_eq!("114", execute_instruction(conection));
    }

    #[test]
    fn test_execute_instruction_not() {
        let conection = Connection {
            input1: "123".to_string(),
            input2: "".to_string(),
            operation: { Operation::Not },
        };

        assert_eq!("65412", execute_instruction(conection));
    }

    #[test]
    fn test_execute_instruction_not2() {
        let conection = Connection {
            input1: "456".to_string(),
            input2: "".to_string(),
            operation: { Operation::Not },
        };

        assert_eq!("65079", execute_instruction(conection));
    }
    //
    // #[test]
    // fn test_func2() {}
}
