#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| {
            let chars = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();
            let first = chars.first().unwrap().clone();
            let last = chars.last().unwrap().clone();
            let combined = [first, last].iter().collect::<String>();
            combined.parse::<u16>().unwrap()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u16]) -> u16 {
    input.iter().fold(0, |acc, val| acc + val)
}
