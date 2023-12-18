use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ObjectType {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Platform {
    objects: Vec<Vec<ObjectType>>,
}

impl Platform {
    fn swap(&mut self, from: (usize, usize), to: (usize, usize)) {
        let obj = self.objects[to.1][to.0];
        self.objects[to.1][to.0] = self.objects[from.1][from.0];
        self.objects[from.1][from.0] = obj;
    }

    fn get_total_load(&self) -> usize {
        let len = self.objects.len();

        let mut total_load: usize = 0;

        for i in 0..len {
            let relative_load = i + 1;
            total_load += self.objects[len - i - 1]
                .iter()
                .filter(|obj| **obj == ObjectType::RoundedRock)
                .count()
                * relative_load;
        }

        total_load
    }
}

fn main() {
    assert_eq!(136, part1(&test_input()));
    assert_eq!(107430, part1(&input()));
    assert_eq!(64, part2(&test_input()));
    assert_eq!(0, part2(&input()));
}

fn part1(input: &str) -> usize {
    let tilted_platform = tilt_platform(&parse(input), Direction::North);

    tilted_platform.get_total_load()
}

fn part2(input: &str) -> usize {
    let mut cycled_platform = parse(input);
    let mut seen_states: HashMap<Platform, usize> = HashMap::new();

    for cycle in 0..1_000_000_000 {
        cycled_platform = tilt_platform(&cycled_platform, Direction::North);
        cycled_platform = tilt_platform(&cycled_platform, Direction::West);
        cycled_platform = tilt_platform(&cycled_platform, Direction::South);
        cycled_platform = tilt_platform(&cycled_platform, Direction::East);

        if seen_states.contains_key(&cycled_platform) {
            println!("We've seen this exact state before, meaning we found a loop! We can now easily extrapolate to determine what a future state will look like.");

            let loop_len = cycle - seen_states.get(&cycled_platform).unwrap();
            let offset = (1_000_000_000 - cycle - 1) % loop_len;
            let index = seen_states.get(&cycled_platform).unwrap() + offset;

            return seen_states
                .iter()
                .find(|(_, i)| **i == index)
                .unwrap()
                .0
                .get_total_load();
        }

        seen_states.insert(cycled_platform.clone(), cycle);
    }

    cycled_platform.get_total_load()
}

fn tilt_platform(original: &Platform, direction: Direction) -> Platform {
    let mut tilted_platform = original.clone();

    let width = tilted_platform
        .objects
        .first()
        .expect("There should be at least a single row of platform")
        .len();
    let height = tilted_platform.objects.len();

    loop {
        let mut did_any_rocks_move = false;

        for y in 0..height {
            for x in 0..width {
                if tilted_platform.objects[y][x] != ObjectType::RoundedRock {
                    continue;
                }

                let (dest_x, dest_y): (i32, i32) = match direction {
                    Direction::North => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::South => (x as i32, y as i32 + 1),
                    Direction::West => (x as i32 - 1, y as i32),
                };

                if dest_x < 0
                    || dest_x as usize > width - 1
                    || dest_y < 0
                    || dest_y as usize > height - 1
                {
                    continue;
                }

                let (dest_x, dest_y): (usize, usize) = (dest_x as usize, dest_y as usize);

                if tilted_platform.objects[dest_y][dest_x] == ObjectType::EmptySpace {
                    tilted_platform.swap((x, y), (dest_x, dest_y));
                    did_any_rocks_move = true;
                }
            }
        }

        if !did_any_rocks_move {
            break;
        }
    }

    tilted_platform
}

fn parse(input: &str) -> Platform {
    let objects = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'O' => ObjectType::RoundedRock,
                    '#' => ObjectType::CubeShapedRock,
                    '.' => ObjectType::EmptySpace,
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
    ".OO.#.....#..O.O.O#....O....#....O....#..##.#...#..O....#.....O...##O...O#.O.#O.O....O......O.....OO
.......#........O...O..#.....O....O.#..#O#.#.##O.#O.#..O.O....O.###O.........#.O.O......O....#O.....
O..O.OOO....#O..#.#......OOO.#O#O.OO..#.O#.O.....O....#O.O####...##O..O....O#..O#...#..#......O..#..
.#.#.##...#.O.#...O##..O....#........#..#.#.O...O.......#.OO...#........#...O#.#.......O..OO.OO.OO##
....O.O.O#.....O#.OO..O...........O.OO..#.........O...O........#.OOOO..##.#....#.#..O..O#..O#.#O..OO
#.#...O##.##...#.#O.OO..O.##O.#.##O...#O#....#OOOO.O........#......O.....O#....O....O..O#.OO#.O.....
......O..O.O...O...#..OOO##...O.OO....#......OO.#.....OOO##..##.#.#..#OOO.......#O.#....O....O...#.#
O..........OOO#.O.O.#.....OO...O.#..#...OO....OO#O.#OOO..OO..#.OO.....#O##.O..#.OOO...#.#O#...#.OO..
....O..#O...........#.O..OO##....#..................O....O.O.O.#.O.......O.O.#.....O..#O..#..#......
....##OO.#.....#OO...O......O#.....O.OO#....O..O.###.##.OO.#O....#.OO#O#.O......##O#..O...O#.#.O..O.
#....OO.##...#.....O.O....#O.O#...OO#..#.O.O.#..O......###.O..O......#..........O...#.O.#O#OO..O.O.#
........#....O..O###.#.#...OO##.#.....#O.O..........#...O..O...OO##....O#..O.#O......#..O..O...#..O#
..O.#..O......#......O..#......#...O.#O...O...O.#..#.OO.O.O...O...O.O.#....#O...O#..##..O..O......O.
...OO.............O....#O.O..O...OOO....O..#...O....#..#.O#.#..OO..#......O.....OOO....O..##..O#....
O.#O#.#O#.#..#.O.#....#..O...#..O#O..#.#.O..O...OO.....O...O....#.O.#OO.OOO#.O.....O..O#.O..#.OO##..
.....#..#.O.O##O.#O..O..OO.#O..O....O..O.....O..#.##..O.............#......O...###...O.....O....#...
O...O..O#O..#O.#O.O#.O...#O#.......O.......#...O.#.#.O#.......O.#.OO.O.#O..O..........O#...O.OO..#..
..O.O#.O.....O.OO..O#O...#.#....#..O.OO.....O.........O.O#...#....##......OO............O.O......#..
...O#OO.....OOO.........#OOO.O.......O.....O....O....#OO##.O##...#.................O....#O...#......
#.O#OO.#..#.......O.O.###O....O..O.....#.O.OOO.#.O#.#..O....#..#...#.O#.O#..##..#...##OO..O#...#...#
O...#.......O##.OO...#O..#......O.......O..#.....#..#.#..OO....#.#.#O..OO..#.OO.#..##OO#.O#OO..#...#
...O...#...O.#O....#.......O..OO.O..O.......O.#....#O..#O#.#...#....#.#....#....O.O......O.O........
#..O...OOOO.......#.....O..O...##O...#OO...OO..OO#O.O..#.....#O...#..#.....O.#O.#...#....#...O#....#
.O.#O.....#...#...O.OOO#...O#..##O#OO##.#.#...O#.....#O..O#..#OO....#O...O.#..O..#.....#.#.OO...O.#.
..#..OO......#..O..#.O.O..O.#...#.#....#.......O.#O#....O.#.O#.O.O.O..O.###O.OOO...#O....O#..O.#...#
....O.#.....#.....OO.......O#O.O..OO#..O.O...OOOO....OO..O.#..O.....OOO.O#O.O......#..#O.O....O.#..O
..#......#O##....O.O.OO..#.#.....O.#.#O#.....OO#......###..#.O.O#....#O...O...O...OO...OO..O.#...O.O
#...#.....O.#..##.......#.OO......#O..O.........O#.#O#.#..#O.O......O.O..O.OO#...O.#..O...O..O.O....
O.#..##...O.O..O....O.O.OO......O...O.O....#..#.O.....OO...O.OO.O....O....O.O.O........O..O#...OO.O.
....O..O.O...OO#O..O..........OO#...O..O.O#...#O......#.#.#...#..O.....O..O.#O#.....O.#O.O..O#.....O
#O...O...O..O.#.O#OO.......O#.....O.#..O.OO.#.........O..OO.....OO.OO#OO...O.#.O....#...#..#.##.O...
##O...#.##...OO.......##..#.#.#..OO.....OO#.....OOO.OO.......O.#.O...#..O.OO.#.....##.#.#........#.#
...OO..#....#..O...O........OO.O...##.......O...#O#..O...#....#O....#....#O..##.#O.#..O.....##OO#.O.
.O.#.O#O.#...#..O.##.#....OO..........O.#.O.O#O...#......O.O....O#.O.#.....O...##...O.#.O.O.#O##.O..
...#.....O....#.OO.#.#..#..#..###.....#.......#.O...O#...O..#.O#O##O.O....#.O.#....O............O...
#.##..##....#...O.....#.O.....OO....OOO......#...O.#....OO.#....O...##..#OO.#....O..O.O#O.O..#...O#.
O##....OO...#.#..O...#......#.##..O......#........O.O.O.#O..#.#..O..#.....#O.O.....##OO.O..O..#.....
.....O#..O.#.#..#O#.O.......OO.O.....#.....##...O..#...OO.O.OO....OOO........O#OO...O..#.OO##OO#.O#.
...#.O#O......O....O#................#O.....#..O.OO#.......#.#.O##......OO.#..#..O#...OO..#.#.O#O.#.
O......#.#....O.O...O.#.O......#.#.O..OOO.O..........#..O....#..O.O.#.O.##OO..#O#.O....#O..###...#..
..##..#...#..#.#O.#.#..#O#.#.....O.#..#OO....OOO#O...O##O..#.##.O..O..O#O..........O.#..#....O#.....
O.....#OO#...OO#O..O......#O.O..........O####.#.....#..#.....#...#..O#...#...#O#...#..O..#.O#......O
....O###.OOO...O.O...O#...O##....OO..O..O#..O.#.OO.O#.#...#O.#......##.O.......O...#.O.O.#.......O.#
O..O.....##....#OO#.OO....##.O#.O..#.#.......O......#....#O#.O.......OO..O.#..#.O..#..........OO..OO
.......O.O......O.....#O..#.....#....#O#...#...O..#...O#....#O.O.....O...#OO...#......#...##...OOO..
#..O.....#O.O..O.O...#.O...O.O.#.#OO.....OOO....O.........OO..O.OO#...O#..O...#.#..#.##.#..OO...#...
...O..OO.#O...OOOOOO..O#....#.OO...O#...O.....#.#.O.....O..O.O#O.#.#.O..#....O..#.#...OO#.......#O..
..............O.OO#.........##..............O..#........##....O...#.#.##O#.O....O#.OO....O.#O.OO.O.O
....O....#..O..O....O##...OO...O....O#..#O#.....OO...#.OOO#....#.O..O..O...#.....#..OO#.#O....O.O#..
#O#...O..O#.OO..OO....#.......O#.........##.....OOO......O..#....O.#.#.O.O..O.O..O...#.#..O.#......O
O....O...#..##...OO.....#.#.#.........OO..#..#...O...O..#..#.OOOO....#O..O##.O.O#....#....O.##.O..O.
....O..#.OO.##...O.OOO.O.O#.....#..O..O....#.#O...#O#O.....O..##...##.O.#O...#.....OO.#..#O#.O.#.O..
O..O#.....O..OO........OO...##....O#OO#.O..OO.....O.....O...O.##..#..#O..O.#.OO....#.........O...#.O
#...O.O..O..O........#OOOO......#..O...#..O.....OO#.....O.......O#...O..O.....##O.....OOO#...#..##O.
#.##...##O.#....#.OO#.OO.#..OO#...#...O..O.O#..##..O.....O.OO.#OO#....#.#.O.#...OO..#.##O...#.....#O
..#.O.....O##.....O.O#O.#O#..#...O...O#.......#O..OO.O..O..O.##O...O...........#..#....O............
..O.#O....O...#O....#....O...OO.OO...#O.O..O.....O.........##.....#..#O.O.OO..O..#....#.............
..#...O.OO.......#....OO.O...#.O...#O.O#......O..#..............#O......#.O#..#...O.......#......O..
..O..O....O.....O#O....OO.....O.O##O...O....#..OO...O.OO....O.O...O.O.......#..#O.O..O.........#.#..
...O..O#.......#..#...OOOO...O.###..#O.OO..O#.O.O.OOO.#.O..O.#......#OO....O....#..#O..O......O....O
....##OO###.......OO.##O.....##.#.OOOO.#OO#...#.#.O..#.....O.#.##O..#OOO.#OO.OO#OO.......O.#...#O...
....O.O.OO.O.#O...#.O.O.....OO#.OO....O.......O.O.O...##OO....O....#.#..#O###...OO..O...#.#.O.O.O.#.
....O....O....#...O..O#.OO..O.OOO...#.......###O#....##O.O#..O...O.#.O.O.......O.###..O#.#.O.O.....#
..#..O...O..O.......O..#.#O.O..O...O.........#.O......#....O..O..O#.O..O..#.O.......#...#...O#.OO..#
O....OO#O..#....O.#OOO..O.....O...O.O....O.O#O..#........OOO..#O..O.O.....O#O...#.#.....#.O.O......#
##OO.O#..#O#.....O.........O..O.#.......OO.....O...#........###.OO...O..#..OO.O#.O#O....#..##....O.O
.#OOO........O#...O.#..O.O.O..#.....#..OOO...#.....O........#.........O.O.....#.#.O.O#O.#.#...#...#.
.O.....#.#....O.O...#O..O###...O..#...#O#.O.O...........#.O....#O.OO......O....O..OO....#....#..##O#
....#..O#...#....#.O..O.#......OOO.........O....#....O...O#......#.#...OO..O..#.#O.#......#.O.#.#...
O.O.......O.O#OOO#.O......O#.O......OO.O..O.......##O....O...#.O...O..#....###.####...OO...OO..O..OO
#..#.....O.....O..O.#....#O....O.OO..#...O.O.....O.O..#O...O#....O##...O.....#OO#..OO#..#.O..O.###..
O##..#...O..#...........OO...#..#....#...#.O.....OOO...O.OO...O.#.#..O...OO.....OO#O...#O...#..#...O
....O....#.....O.#.#.OO..#.........#..........O#..O#O.#....###..O.....#.......O..#.#.....OOOO.......
O.O.O.#........#.#.O.#..##O.OO..O...#O#OO.....O...O.#.#.O#O......#O.....O...O..O##.##.#......O..#...
....O.......O..#.OO###..#.O#....#O.OO#..OO...#.#...###...#OO#..#...........#OO#..O.#..#....O#....O.O
....#O..#...#O..O..O...#....O....#....OOO.OO.O..#...OO.O#.....OO.O..#..#...O.#.#.O..#.#O.O#O.O......
.OO....#.OO.............#..###.#...O...#O...OO#...OO#O..#...##.#..........O..#...#.....O.....O...O..
#.O....#O....O#..#O......#####O....O..O....O#.......#..O.#..#....O...##..#.........O..O.....#.O#.O..
...O.O.....#....O.#O....#....O..........###....O.#.O.O#O.#.#.O..#.O.O...#.###...OOO...##..O.#.O..O#.
.##...#..O..O.#....O..OO..O.....#..O.##O#O.........OO#...#.......O#....O............#O.##.OO#..O.#..
.O.........O.#O.......#O..O....#.##.O.O...##....#.##...#O#.#.....O#O.#...#..O.#.O...O..##..O.#.O.O..
...O.#.O.O#.OO..OO#.O...O.O.O...O.O.###O.O..O.O.O........#.O....#....O.#.#OO.....OO.#.....##..#OO...
....OO##...#O.#..#.....#.#..##O.O##..O...OO#O...........O..O.#.OO..O...OO.##.O.#..#.##OO...#..O.....
...#....O....#..#.O....O..#....O...#....#O...O.O..#O#O..O.......#..O......#OO..OO.OO......#.##O...O.
...O......O.....OO#.O.#..O..O..O.O.....O...........O....#.###...O.#......O....O.##..#....O...#.#.#..
..O.O.#.O......OO..##.O......O#..#O.....##O.O#....#.#....O.....OO.#.....O.O#.#..O...O.#....O#...O..#
O...OO#O.......#..#O#........#.#.##.O........#O...#..#O..#.....#....O#O.#......##O....O.OO.O........
.#..O.....OO..O.#..O.O........O.......#.#.#.....O.OO.....#......O.......O##.O...#....#O....O........
.O.OO....O.O.##..........O.....O.##.#.......OO....O.O....#........##O..O.O##.O......#.#.#.....OO.O#.
O.....OOO.O###...OO.##.............#..O.#.....#O.....O..OO..O#....##....OO.....#.O.#OO......OO...O##
O........###.##..O......O#.....O.O#O......#....O#..O........O..O.#.......OO......O..##..OOO.O..O.O#.
OO.#O......OO...O..O..#..##O..#...O......#..............O..............O..#......#.#O....O...#...O..
.O..O#.O#..O.#..O..OO#..#.##O.....O.O.#......##O#..O....O......#.#.O..#.....O#.O..O.#.##.......#....
....OO.#....O..O...OO..#O..#...###..OO.......O##.##.#OO..OOO...#.O..O......O........##..#...OO...#..
....#O........#.O......O##...##O......#.O..O.O........##...#.O.......O..#OOOOO..#......O###..O..O.O.
#.#..O.#.........O#...#O.##..O.....#..O...........##......O..#...O......#.................OO...#O.#.
..O..#OO...OOO...O.#...#.......#OOOO#...#...O....O..#........OO....O..O..##.OO.O#....OO#O#..#....O..
.O.O..........OOOO.#.#......O##...............#.O.OO.###.#....O..O.O#O...OO..O#O........#.O.....O#.O
...#......#.#.....#........#O#.........#O....#O.O..#.###.....O#OOO#O#.#.O..#.#OOO..O.#....#O......#.
O..O...#O.O.....O..O.#.#......O.#O#...#.....#..OO#.....OO.O#.O.....#....O.#...O...O.OO.....O....O#..".to_string()
}
