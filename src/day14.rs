#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum ObjectType {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Platform {
    objects: Vec<Vec<ObjectType>>,
}

fn main() {
    assert_eq!(0, part1(&test_input()));
    assert_eq!(0, part1(&input()));
    assert_eq!(0, part2(&test_input()));
    assert_eq!(0, part2(&input()));
}

fn part1(input: &str) -> usize {
    let tilted_platform = tilt_platform(&parse(input));

    let len = tilted_platform.objects.len();

    let mut total_load: usize = 0;

    for i in 0..len {
        let relative_load = i + 1;
        total_load += tilted_platform.objects[len - i]
            .iter()
            .filter(|obj| **obj == ObjectType::RoundedRock)
            .count()
            * relative_load;
    }

    total_load
}

fn part2(input: &str) -> usize {
    todo!();
}

fn tilt_platform(original: &Platform) -> Platform {
    original.clone()
}

fn parse(input: &str) -> Platform {
    let objects = input
        .split("\n")
        .map(|line| {
            line.split("")
                .map(|ch| match ch {
                    "O" => ObjectType::RoundedRock,
                    "#" => ObjectType::CubeShapedRock,
                    "." => ObjectType::EmptySpace,
                    other => panic!("'{}' isn't parsable to an object type", other),
                })
                .collect::<Vec<ObjectType>>()
        })
        .collect::<Vec<Vec<ObjectType>>>();

    Platform { objects }
}

#[allow(dead_code)]
fn test_input() -> String {
    "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "".to_string()
}
