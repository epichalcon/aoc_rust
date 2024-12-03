pub fn traspose_string(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let transposed: Vec<String> = (0..max_len)
        .map(|col| {
            lines
                .iter()
                .filter_map(|line| line.chars().nth(col))
                .collect::<String>()
        })
        .collect();

    transposed.join("\n")
}

pub fn traspose_string_vec(input: Vec<String>) -> String {
    let max_len = input.iter().map(|line| line.len()).max().unwrap_or(0);

    let transposed: Vec<String> = (0..max_len)
        .map(|col| {
            input
                .iter()
                .filter_map(|line| line.chars().nth(col))
                .collect::<String>()
        })
        .collect();

    transposed.join("\n")
}

pub fn print_matrix(matrix: Vec<Vec<char>>) {
    for row in matrix.iter() {
        for ch in row.iter() {
            print!("{ch}");
        }
        println!("");
    }
}
