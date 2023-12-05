use regex::Regex;

#[derive(Debug)]
struct PotentialPartNumber<'a> {
    number: i32,
    current_line: &'a str,
    start_index_in_line: i32,
    end_index_in_line: i32,
    line_above: Option<&'a str>,
    line_below: Option<&'a str>,
}


impl PotentialPartNumber<'_> {

    fn touches_symbol(&self) -> bool {
        let symbols: Vec<&'static str> = vec!["*", "$", "#", "+"];

        let touches_horizontal: bool = {

            if self.start_index_in_line == 0 { return false };

            symbols.iter().any(|symbol| **symbol == self.current_line.as_bytes()[(self.start_index_in_line - 1) as usize] as str)
        };

        false
    }
}

fn main() {
    part1();
}

fn part1() {
    let total = parse(input()).iter().fold(0, |acc, num| acc + num.number);

    println!("{}", total);
}

fn parse(input: Vec<&'static str>) -> Vec<PotentialPartNumber> {
    let mut vec = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for (i, line) in input.iter().enumerate() {
        for capture in re.captures_iter(line) {
            let (_, [number]) = capture.extract();
            let m = capture.get(0).unwrap();

            vec.push(PotentialPartNumber {
                number: number.parse().unwrap(),
                current_line: line,
                start_index_in_line: m.start() as i32,
                end_index_in_line: m.end() as i32,
                line_above: match i {
                    0 => None,
                    _ => Some(input.get(i - 1).copied().unwrap()),
                },
                line_below: input.get(i + 1).copied(),
            });
        }
    }

    return vec;
}

fn input() -> Vec<&'static str> {
    vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ]
}
