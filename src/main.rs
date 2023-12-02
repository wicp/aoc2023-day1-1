fn sum_line(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    match digits.len() {
        0 => 0,
        1 => digits[0]*10 + digits[0],
        len => digits[0]*10 + digits[len-1],
    }
}

fn sum_lines(input: &str) -> u32 {
    input.lines()
        .map(sum_line)
        .sum()
}

fn main() {
   let input_file = std::fs::read_to_string("./input.txt").expect("could not read input.txt in current directory"); 
   println!("{}", sum_lines(&input_file));
}
