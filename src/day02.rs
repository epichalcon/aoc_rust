use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    let reslut: i32 = input.lines().map(calculate_wrapping_paper).sum();
    println!("First star: {}", reslut);
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    let reslut: i32 = input.lines().map(calculate_ribbon).sum();
    println!("Second star: {}", reslut);
    println!("\t time:{:?}", start_time.elapsed());
}

fn calculate_wrapping_paper(input: &str) -> i32 {
    let dimensions: Vec<i32> = input
        .split('x')
        .filter_map(|dimension| dimension.parse().ok())
        .collect();

    let mut base = 2 * dimensions[0] * dimensions[1];
    let mut altura = 2 * dimensions[0] * dimensions[2];
    let mut ancho = 2 * dimensions[1] * dimensions[2];

    if base <= altura && base <= ancho {
        base += dimensions[0] * dimensions[1];
    } else if altura <= base && altura <= ancho {
        altura += dimensions[0] * dimensions[2];
    } else if ancho <= base && ancho <= altura {
        ancho += dimensions[1] * dimensions[2];
    }

    ancho + base + altura
}

fn calculate_ribbon(input: &str) -> i32 {
    let mut dimensions: Vec<i32> = input
        .split('x')
        .filter_map(|dimension| dimension.parse().ok())
        .collect();

    dimensions.sort_by(|a, b| a.cmp(b));

    // let bow_length = dimensions.iter().fold(1, |acc, &x| acc * x);
    let bow_length = dimensions[0] * dimensions[1] * dimensions[2]; //puede que sea mas rapido en este caso

    2 * dimensions[0] + 2 * dimensions[1] + bow_length
}

#[cfg(test)]
mod tests {
    use crate::day02::{calculate_ribbon, calculate_wrapping_paper};

    #[test]
    fn test_calculate_wrapping_paper() {
        assert_eq!(58, calculate_wrapping_paper("2x3x4"));

        assert_eq!(43, calculate_wrapping_paper("1x1x10"));
    }

    #[test]
    fn test_calculate_ribbon() {
        assert_eq!(34, calculate_ribbon("2x3x4"));

        assert_eq!(14, calculate_ribbon("1x1x10"));
    }
}
