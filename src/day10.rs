#[derive(Debug, PartialEq)]
enum RelativeDirection {
    Top,
    Right,
    Bottom,
    Left,
}

fn main() {
    assert_eq!(6956, part1());

    assert_eq!(455, part2());
}

fn part1() -> i32 {
    let input = input();
    let (x, y) = find_starting_position(input).unwrap();

    let connections = find_connections_to_start(input, x, y);

    let mut one_x = connections[0].0;
    let mut one_y = connections[0].1;

    let mut one_last_x = x;
    let mut one_last_y = y;

    let mut two_x = connections[1].0;
    let mut two_y = connections[1].1;

    let mut two_last_x = x;
    let mut two_last_y = y;

    let mut steps = 1;

    loop {
        if one_x == two_x && one_y == two_y {
            break;
        }

        match traverse_pipes_get_next_coord(input, (one_x, one_y), (one_last_x, one_last_y)) {
            (Some((new_x, new_y)), (prev_x, prev_y)) => {
                one_x = new_x;
                one_y = new_y;
                one_last_x = prev_x;
                one_last_y = prev_y;
            }
            _ => (),
        };

        match traverse_pipes_get_next_coord(input, (two_x, two_y), (two_last_x, two_last_y)) {
            (Some((new_x, new_y)), (prev_x, prev_y)) => {
                two_x = new_x;
                two_y = new_y;
                two_last_x = prev_x;
                two_last_y = prev_y;
            }
            _ => (),
        };

        steps += 1;
    }

    println!("The point farthest from the starting position is {steps} steps away");

    steps
}

fn part2() -> i32 {
    let input = input();

    let coords = find_coords_of_all_pipes_in_loop(input);

    let mut enclosed_tiles: Vec<(i32, i32)> = vec![];
    // TODO: Figure out what we should use as the "inside" of the loop.
    // Idea: Raycast both ways while traversing the loop, at some point we must
    //       hit the canvas edge with either of the two rays. This is then the
    //       outside. We either move the starting position to our current
    //       position or we traverse back keeping track of the inside along the
    //       way.
    //
    //       For some reason, though, we always get the right answer already,
    //       without this being implemented, no matter which starting inside
    //       direction we choose. Magical...
    let mut inside_direction = RelativeDirection::Bottom;

    for (mut index, (x, y)) in coords.iter().enumerate() {
        if index + 1 >= coords.len() {
            index = 0;
        }

        inside_direction = determine_inside(&coords, (*x, *y), &inside_direction);

        let tiles_inside = find_tiles_raycast(&coords, (*x, *y), &inside_direction);

        enclosed_tiles.extend(tiles_inside);

        // If the next pipe changes direction, we need to do two directional ray cast.
        // This solves the literal corner cases.
        if determine_inside(&coords, coords[index + 1], &inside_direction) != inside_direction {
            let tiles_inside = find_tiles_raycast(
                &coords,
                (*x, *y),
                &determine_inside(&coords, coords[index + 1], &inside_direction),
            );

            enclosed_tiles.extend(tiles_inside);
        }
    }

    // Dedup only works for consecutive items, so the list must be sorted.
    enclosed_tiles.sort();
    enclosed_tiles.dedup();

    // Uncomment the next two lines to show the output in the terminal.
    // let output = mark_coords_on_input(input, &coords, &enclosed_tiles);
    // println!("{}", output);

    println!(
        "There are exactly {} tiles enclosed by the loop of pipes",
        enclosed_tiles.len()
    );

    enclosed_tiles.len() as i32
}

#[allow(dead_code)]
fn mark_coords_on_input<'a>(
    input: &'static str,
    pipes: &Vec<(i32, i32)>,
    enclosed_tiles: &Vec<(i32, i32)>,
) -> String {
    let mut output = "".to_owned();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if pipes.contains(&(x as i32, y as i32)) {
                output += match ch {
                    'L' => "└",
                    'J' => "┘",
                    '7' => "┐",
                    'F' => "┌",
                    '|' => "│",
                    '-' => "─",
                    _ => "S",
                };
            } else if enclosed_tiles.contains(&(x as i32, y as i32)) {
                output += &"I";
            } else {
                output += " ";
            }
        }

        output += "\n";
    }

    output
}

fn determine_inside(
    all_pipes: &Vec<(i32, i32)>,
    (currentx, currenty): (i32, i32),
    last_inside: &RelativeDirection,
) -> RelativeDirection {
    let (i, _) = all_pipes
        .iter()
        .enumerate()
        .find(|(_, coord)| **coord == (currentx, currenty))
        .unwrap();
    let prev_i = match i {
        0 => all_pipes.len() - 1,
        index => index - 1,
    };

    let relative_direction = get_relative_direction(all_pipes[prev_i], all_pipes[i]).unwrap();

    let inside = match (last_inside, &relative_direction) {
        (RelativeDirection::Left, RelativeDirection::Bottom) => RelativeDirection::Left,
        (RelativeDirection::Right, RelativeDirection::Bottom) => RelativeDirection::Right,
        (RelativeDirection::Left, RelativeDirection::Top) => RelativeDirection::Left,
        (RelativeDirection::Right, RelativeDirection::Top) => RelativeDirection::Right,

        (RelativeDirection::Bottom, RelativeDirection::Right) => RelativeDirection::Bottom,
        (RelativeDirection::Top, RelativeDirection::Right) => RelativeDirection::Top,
        (RelativeDirection::Bottom, RelativeDirection::Left) => RelativeDirection::Bottom,
        (RelativeDirection::Top, RelativeDirection::Left) => RelativeDirection::Top,

        (RelativeDirection::Bottom, RelativeDirection::Bottom) => RelativeDirection::Right,
        (RelativeDirection::Top, RelativeDirection::Bottom) => RelativeDirection::Right,
        (RelativeDirection::Bottom, RelativeDirection::Top) => RelativeDirection::Left,
        (RelativeDirection::Top, RelativeDirection::Top) => RelativeDirection::Left,

        (RelativeDirection::Right, RelativeDirection::Right) => RelativeDirection::Top,
        (RelativeDirection::Left, RelativeDirection::Right) => RelativeDirection::Top,
        (RelativeDirection::Right, RelativeDirection::Left) => RelativeDirection::Bottom,
        (RelativeDirection::Left, RelativeDirection::Left) => RelativeDirection::Bottom,
    };

    inside
}

fn find_tiles_raycast(
    all_pipes: &Vec<(i32, i32)>,
    (origin_x, origin_y): (i32, i32),
    direction: &RelativeDirection,
) -> Vec<(i32, i32)> {
    let max_x = all_pipes
        .iter()
        .reduce(|max, cur| match cur.0 > max.0 {
            true => cur,
            false => max,
        })
        .unwrap()
        .0;

    let max_y = all_pipes
        .iter()
        .reduce(|max, cur| match cur.1 > max.1 {
            true => cur,
            false => max,
        })
        .unwrap()
        .1;

    let mut enclosed_tiles = vec![];

    match direction {
        RelativeDirection::Top => {
            for y in (0..origin_y).rev() {
                let tile = (origin_x, y);

                if all_pipes.contains(&tile) {
                    break;
                } else {
                    enclosed_tiles.push(tile);
                }
            }
        }
        RelativeDirection::Bottom => {
            for y in origin_y + 1..max_y {
                let tile = (origin_x, y);

                if all_pipes.contains(&tile) {
                    break;
                } else {
                    enclosed_tiles.push(tile);
                }
            }
        }
        RelativeDirection::Right => {
            for x in origin_x + 1..max_x {
                let tile = (x, origin_y);

                if all_pipes.contains(&tile) {
                    break;
                } else {
                    enclosed_tiles.push(tile);
                }
            }
        }
        RelativeDirection::Left => {
            for x in (0..origin_x).rev() {
                let tile = (x, origin_y);

                if all_pipes.contains(&tile) {
                    break;
                } else {
                    enclosed_tiles.push(tile);
                }
            }
        }
    }

    enclosed_tiles
}

fn find_coords_of_all_pipes_in_loop(input: &'static str) -> Vec<(i32, i32)> {
    let (start_x, start_y) = find_starting_position(input).unwrap();

    let connections = find_connections_to_start(input, start_x, start_y);

    let mut one_x = connections[0].0;
    let mut one_y = connections[0].1;

    let mut one_last_x = start_x;
    let mut one_last_y = start_y;

    let mut pipes = vec![(start_x, start_y)];

    loop {
        pipes.push((one_x, one_y));

        match traverse_pipes_get_next_coord(input, (one_x, one_y), (one_last_x, one_last_y)) {
            (Some((new_x, new_y)), (prev_x, prev_y)) => {
                one_x = new_x;
                one_y = new_y;
                one_last_x = prev_x;
                one_last_y = prev_y;
            }
            _ => break,
        };
    }

    pipes.dedup();

    pipes
}

fn find_starting_position(input: &'static str) -> Option<(i32, i32)> {
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                return Some((x as i32, y as i32));
            }
        }
    }

    None
}

fn find_connections_to_start(input: &'static str, start_x: i32, start_y: i32) -> Vec<(i32, i32)> {
    let mut connections = vec![];

    for (x, y, char) in get_surrounding_coords((start_x, start_y))
        .iter()
        .filter_map(|(x, y)| {
            get_char_by_coordinate(input, *x, *y).map(|character| (x, y, character))
        })
    {
        if is_pipe(char) && is_pipe_connected_to_start(input, (start_x, start_y), (*x, *y)) {
            connections.push((*x, *y))
        }
    }

    connections
}

fn find_connections(input: &'static str, (origin_x, origin_y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut connections = vec![];

    for (x, y, char) in get_surrounding_coords((origin_x, origin_y))
        .iter()
        .filter_map(|(x, y)| {
            get_char_by_coordinate(input, *x, *y).map(|character| (x, y, character))
        })
    {
        if is_pipe(char) && are_pipes_connected(input, (origin_x, origin_y), (*x, *y)) {
            connections.push((*x, *y))
        }
    }

    connections
}

fn get_char_by_coordinate(input: &'static str, x: i32, y: i32) -> Option<char> {
    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if column as i32 == x && row as i32 == y {
                return Some(char);
            }
        }
    }

    None
}

fn is_pipe(char: char) -> bool {
    vec!['|', '-', 'J', 'L', 'F', '7'].contains(&char)
}

fn are_pipes_connected(
    input: &'static str,
    (a_x, a_y): (i32, i32),
    (b_x, b_y): (i32, i32),
) -> bool {
    let a_char = get_char_by_coordinate(input, a_x, a_y).unwrap();
    let b_char = get_char_by_coordinate(input, b_x, b_y).unwrap();

    if let Some(rel_dir) = get_relative_direction((a_x, a_y), (b_x, b_y)) {
        return match rel_dir {
            RelativeDirection::Top => {
                (a_char == '|' || a_char == 'J' || a_char == 'L')
                    && (b_char == '|' || b_char == '7' || b_char == 'F')
            }
            RelativeDirection::Right => {
                (a_char == '-' || a_char == 'F' || a_char == 'L')
                    && (b_char == '-' || b_char == '7' || b_char == 'J')
            }
            RelativeDirection::Bottom => {
                (b_char == '|' || b_char == 'J' || b_char == 'L')
                    && (a_char == '|' || a_char == '7' || a_char == 'F')
            }
            RelativeDirection::Left => {
                (b_char == '-' || b_char == 'F' || b_char == 'L')
                    && (a_char == '-' || a_char == '7' || a_char == 'J')
            }
        };
    }

    false
}

fn is_pipe_connected_to_start(
    input: &'static str,
    (startx, starty): (i32, i32),
    (b_x, b_y): (i32, i32),
) -> bool {
    let b_char = get_char_by_coordinate(input, b_x, b_y).unwrap();

    if let Some(rel_dir) = get_relative_direction((startx, starty), (b_x, b_y)) {
        return match rel_dir {
            RelativeDirection::Top => b_char == '|' || b_char == '7' || b_char == 'F',
            RelativeDirection::Right => b_char == '-' || b_char == '7' || b_char == 'J',
            RelativeDirection::Bottom => b_char == '|' || b_char == 'J' || b_char == 'L',
            RelativeDirection::Left => b_char == '-' || b_char == 'F' || b_char == 'L',
        };
    }

    false
}

fn get_relative_direction(
    (a_x, a_y): (i32, i32),
    (b_x, b_y): (i32, i32),
) -> Option<RelativeDirection> {
    if b_x == a_x && b_y == a_y - 1 {
        return Some(RelativeDirection::Top);
    } else if b_x == a_x + 1 && b_y == a_y {
        return Some(RelativeDirection::Right);
    } else if b_x == a_x && b_y == a_y + 1 {
        return Some(RelativeDirection::Bottom);
    } else if b_x == a_x - 1 && b_y == a_y {
        return Some(RelativeDirection::Left);
    }

    None
}

fn traverse_pipes_get_next_coord(
    input: &'static str,
    (x, y): (i32, i32),
    (lastx, lasty): (i32, i32),
) -> (Option<(i32, i32)>, (i32, i32)) {
    (
        find_connections(input, (x, y))
            .iter()
            .filter(|coord| **coord != (lastx, lasty))
            .cloned()
            .collect::<Vec<(i32, i32)>>()
            .first()
            .cloned(),
        (x, y),
    )
}

fn get_surrounding_coords((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
}

#[allow(dead_code)]
fn test_input() -> &'static str {
    "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
}

#[allow(dead_code)]
fn input() -> &'static str {
    "7-LJ7.F-F77FF-77FJ-J-F7FF|777F7--..JJ.F.7.|.F-J7-J777F7FF77F|.|7L-7.F-|7F7FF-J7LF|.7--7J-F.F--7-L--77.|F-J77F7F|-F-F7-J-FFF---F--77|FF7--L7.
L7.F--J.L7J7|---FJ-|JLF77L7J7F|-J7|J.FLLJ.FFF7|F7L7-J7LF7.-|L-77-LL7J.|.L-7JFJ7-7J7L-LFJF-7-|7|-|FL--J|L77L-J|7|F|-7J.|LLJJL|.|JF7-|-|||L|-F
F|--7.L-7|.-L-|.|7-LL-J|L-|.FJJ7F-..F|.L|FLJL7F7|F.L-|-7LJL|JL77-.|7|7.J.LF-7FJ7JF-J||J|FL77LJJ-7-77||F--7JL-7-LJJF--77LLF7F7-|.JJ|||JFF-|JF
FJ|L7-FL-L7J|FL-|LJFL7-F|LJ--J.F-7F-7L7--7.|F7J|F|7.L||F7J-J.FFJ.LLF-7F..-|--JLJF7J.LF.LJ|L-JJ.|.LJ-LFF7|L7-L|-LF7|F|7--7LJF77|7|LF|7FJ|L|F|
|LF7L-7J||LF-FJ.|7FF-F--J-J-L-|JJLL.J.||L7-FF7|L-L-7F77L77FJFFLJ7|JL7F-77-L-JL--LJJ7L-77LJJJ|.FF-FJLL|L--L7F|JFLF7FLJ7FJJJFL7JJ77-7|LJL|.J-F
|7LJ-|L--L7|FF7F.F7F--7|J|LF.-L-F.|77FLJ.7|FJ|77|||F7L77JFF.F|FJ7|.L-L---7|.||L-JJF--L-|-||L|7LJFJ.F77.|7FFLJ.7JLLJ|L-J.LLJLJJF-7JF7JFJJ.|.|
L|JLLL-|.|--LJ-J7LLLJ.-7.FJL7-|||.F7-7J|FF7|FJFJL7F|L7L|FJ7|J|-|L7JJ|7-|.-F7L-7.LJ7.L|J..7|JL7LF7JF7LJFLF777FF7FFJ|F-JLF--|LLFJF-7|L.7||7..L
.|.F||L.FL-FL7F-JL|7...L7|7FL7|F-FJF-JLLFJLJ|J|-F--JFJ7FFJLJ-|L|7|.FJ-7..||L7|F7|7L-FJJFF|7-F7J||FJ|7LLLJLLLFJ|-7L777.--L7L-77.J|F-JLJ-F7|77
F7F7L-.F7FF..-|FJ7LLF|JF7JFJ7|FF7|F|||-LL--7|F77L--7L7F77-|.||.J---|.|L-7FJFJF|F7-7|.|.|L|-F||.||L7L7LL|J7||L7|.L7LFF7JJJ-77|L-L7.|7J|F-JL-J
L-J|7.F7-7.77.|JFFFF.--LJ.J-LL.F-LF-7-F7F--JLJ|F7JLL7||L7.777-|JLL-J-7LLLL7L-7F|||L-J-F--7.F||FJL7|FJ.||LF7F7||FFJLL||J|FF77F-J.7JF|F-J..7|J
|L-F7-J|L|7L-.|LLF77-|FJ|.L.|J.||-L7L-J|L----7||||F7|LJFJF--7FL7||.F7JFLL-L-7|FJL77FJFL-7L7FJ|L-7||L777J7|LJ||L-77|FJ||FFJ|7||.7JF|LJ|J.-L7|
J-FL--FF.LL-FLJ7..|LJL7J|7.L-.--7..L--7L---7FJ|||FJLJF-J|L7FJ-J|7JFLF-F7FFLFJ||F-JF7L|F7|FJ|FJF7|LJFJF7F7L-7LJF-JFFL7L7FJFJ||77J7FJFF|-||.-J
LFJ7|L-77||F|FLLF-|L-.|F77J|LL7F|-FFF7L---7|L7||||F--JF77.|L7JFF7F7|F-7LFFFJFJ||F-JL7FJLJ|FJL-J|L-7|J|LJ|F7L-7L7F7F7|FJL7L77LJ|F-J---L.LLJJJ
.LJFJ7LF7--J|L7|L--7|FFJL7.-7LJ7|F-7|L7F--J|F|LJ|||F-7|L-7|FJF-J|F-7L7|-F7L7L-J||F--JL--7||F--7L7FJ|FJF-J|L7LL7LJ|||||F7|FJ7F-FJ77|.||7.|J|7
FJ.7-JLJ-JFL|.|7LF|FF7L7FJ.||FJF7L7|L7|L--7|FJF7||||FJL-7|||LL7FJ|FJFJ|FJ|FL--7|||F77F7FJ||L-7L-JL7|L7|F7L7|F7L-7LJLJLJLJL-7J|FJLF7.F-7F7.77
FJLF7.L7JF7F-7777LJF|L7||F7-F-7|L-JL-JL7F-J|L7|LJ|LJL--7|LJL-7|L-JL7L7LJFJF7F-J||||L7||L7||F-JF7F7|L7|LJ|FJ|||F-JF-----7F7FJ7LJ--|F7|L--J-F7
L-LJL.|LF|L---FJ-LFJL7|||||JL7|L----7F-JL-7|||L7FJF----JL-7F-JL-7F7L7L-7|-||L7FJ|||FJ||FJ||L7FJLJ|L7|L-7LJFJ|||F7L--7F7LJLJF|F-.L|LFF-|J-FLJ
.L||F-7.LL-L7F|7.F-F-JLJ||L7|||FF-7FJL7LF-JL-JFJL7L-7F7F--JL7F-7||L-JF-JL7||FJL-J||L-J|L7|L7|L--7|FJ|F7L-7L7||LJ|F--J|L---7JJ|.FJF7|7-77L7JJ
7F|LJFJ.|JF77LF7J7F|F7F7LJFJFJL7L7|L-7L7L7F7F7L-7|F7LJ||F7F-JL7||L7F7|F7FJ||L-7F-JL--7L7||J|L7F-J|L7|||F7L7||L7FJL---JF7F-J..|..FJFLJF|--J.|
|L7--F-7-F7--7L|LFLLJLJ|F7L7L7FJFJL-7L7|.LJLJL7FJ||L--JLJ|L7F-JLJFJ||LJ||FJL7FJL-7F-7|FJ|L7|FJL-7|FJ|||||7||L7LJF-7F7FJLJ7-|-|-F7JFL7J.F-J-7
|LFJ7|.F-7LFJ-J|.|||-F7LJL7L-J||L-7FJFJ|F7F7F7|L-JL7LF7|FJFJL--7FJ-|L77||L7FJ|F--JL7||||L7||L7-FJ||FJ||||FJ|||F-JFLJLJJ.F-77-|J||LF7J.F-7--|
|.|.7-LL7|JLJ7FJ.FF--J|7-|L--7L7F7|L-JFJ|||LJ||F--7|FJL7|FJF7F7||F7L7L7||FJL7||F--7|||L7FJLJFJFJFJ|L7||||L7|FJL--7F---7FJFJJJF-JL-7JJ|..||.J
FF77.FL.L7.FF|J7-LL7F7L7F---7|FJ||L7F7L7||L-7||L7|LJL7FJ||.|||||LJL7L7LJ||F7||||F-J|||FJL--7L7L7|FL7LJ||L-JLJF--7|L7F-J|FJ|F7|F7F7|7FL--J7F.
|.L77|7L-77J7J|JFLLLJL7|L--7|||FJ|FLJL7||L7-|LJFJF-7FJL-JL7|LJLJF--J|L-7|LJ||||||JFJ||||-F7|FJFJL-7L-7||F-7F7L-7LJL|L7FJL--JLJ|LJLJ7J.L7L|-L
L-7L7--F|L|.F7L--7FF--JL7F-JLJ|L7|F--7|||FJFJF-J.L7|L--7F-JL-7F-JF-7F7FJ|F7|||||L7L7LJL7FJLJLSL-7FJ|FJLJL7LJ|F-JJF7|FJL7F---7FJ7-||JF7L|.|LJ
.FFJJ.FF.-L7.F-|-LFL---7|L---7L-J||F7LJ||L7L7|F7FFJL-7FJ|F7F-J|F7L7|||L7||LJLJ|L7|FJF--J|F--7|F-JL--JF---JF7LJF--JLJ|F-JL--7LJ.|7FF-|7FLJL7.
FF|.FF77-L-L-|7L-JLLF7FJ|F--7L--7|LJL7FJ|FJFJLJL7L-7FJL7|||L-7LJ|FJ|||L|||F--7|FJ|L7|F7FJ|F7LJL-----7|F-7FJ|F7|F---7LJF-7F7L--7-77L|J.JJ|FLJ
7LL7J|JJ7L-7.-L-|LF-JLJFJL-7|7F-JL--7|L7|L7L7F--JF7||F-JLJ|F7L7FJL7LJL7||||F7LJL7|-|LJ|L7||L-7|F7F--J||FJ|FJ|||||F7|F7L7|||F7FJ7||F-..J--7-J
|7J|JLF7-7.|7LJJ||L-7F7|-F7|L7L----7|L7||FJFJL-7FJ|||L7F--J||FJL-7L-7FJ||||||F7FJL-JF7L-J||F-JFJ|L--7LJL7|L7|||L7||LJL-J||||LJ-7-JJ|..|7.|7|
|-7.|.F|-7-LJ.L|F7F7LJ|L-JLJFJFF7F7||-||||FJF--J|FJ|L-J|F-7|||F--JF7|L-J|||||||L7F--J|F--J|L7FJFJF7FL-7FJ|FJ|LJFJ|L----7LJLJ.F777|--.|LL-7-7
L7|F-JJ.||FL-J77||||F-JF--7FJF7|LJLJ|FJ||||FJF--J|JL--7||FJ||||F7FJ||F--J||||||-LJFF7|L-77L7LJFJFJ|F7FJL-JL7|F-JFJF----JJ-LF-JL7F77.L|7FL7-F
J7-F77.-77--FLFFJLJ|L--JFFJ|FJ|L--7FJL7LJ||L7L--7L7F7FJLJ|FJ||LJ|L7|||F7F||||||F-7FJLJF-JF7L-7|-L7|||L-7F-7LJ|F7|FJF7F--7F-JF--J77L7LF-77|F|
|7-7FJ--J-7FJJ.L--7L7F7F7L7|L7|F7FJL7L|F-J|FJ.F-JFJ||L-7FJ|FJ|F-JJ|LJLJL7|||||||FJL--7L--J|FFJL7FJLJL7FJ|FL--J|LJL-JLJF-JL-7|F7LL-.|J|7.LL--
L|--JJJF-L7.|7-|JLL7LJLJL-JL-J|||L-7L7|L-7|L7FJF-JFJ|F7|L7||FJL-7FJF----J|||||LJ|F7F-JF---JFJF-JL-7F-JL7L-----JF7F7F--J7F7FJLJL-7F|JFL|.|7F.
||.FJFF|77|7JF7JLF7L---------7LJ|F-JFJL7FJ|FJ|FJF7|FJ|||FJ|||F--JL7|.F7F7||||L-7|||L7FJF7F7|FJF7F7||F7||F---7F7||||L-7F-JLJF----J--7|F-7|F|.
-J.-FF-F-LF7-|L7F|L---------7L-7LJF7L-7||FJ|FJL7|LJL7||||FJ|||F--7||FJ||||||L-7|||L7||FJ|||||FJ|||||||FJL7F7LJ||LJL--J|F7F7L--7F-7.L-|-L|-77
.L7LLJ|.FL||LL7L7L-----7F7F7L--JF7|L7F||LJFJ|FFJL-7FJ|||||FJ|||F-J|LJ|||||||F7|||L7|||L7||LJ||FJ|||||||F7LJL-7|L--7F-7LJ||L--7LJFJ77.--7L7F7
|.|F|F||77|L--JFJF7JF-7LJLJL----JLJFJFJL-7L7|FJF-7|L7|||||L7||||F7L7F-J|||LJ|LJ||FJ||L-J|L7FJ|L7|||||LJ|L----J|F--JL7|.FJL7F7L--JJ-LF|FFL-FL
F-F77-LJF-JF--7|FJ|FJFJF-----------J-L7F-JJ|||FJ||L7|||||L7||||||L7|L7FJLJF7L7FJ|L7|L--7|FJL7|7LJ|||L-7|F7F-7FJL7F--JL-JF-J|L----7.L-F-L.L7|
|F|JJ|7-L7FJ|FJLJFJL7L7L-7F7F----7F--7|L-7FJ||L-7|FJ||LJ|FJ||||||FJL7|L---J|-||FJFJ|F7FJ||F-J|F-7LJ|F-JLJLJFJL--J|F----7L7||F----J-|.|FFJ7L7
L-L7-J7.LLJ7FJF-7L-7L7|F7LJLJF--7LJF7|L7FJ|FJL7FJLJ.|L7FJL7LJLJLJL7FJ|F7F--JFJ|L7|FJ||L7|||F7|L7L--JL7F----JF--77|L---7L7|FJL---7J-7--.|.--J
F|LJ.7||J7JFJFJFJF-JFJLJ||F-7L-7|F7|LJL|L7|L7FJL---7L-J|F-JF--7F7F|L7LJ|L7F7L7|FJ||FJL7|LJ||||FJF7F--JL--7F7|F-JFJF7F7L7|LJF7F--J-J.FFL-7.LJ
L7J.--777|7L-JLL7|F7L--7L7|FJF-JLJLJ7F7|FJL7|L7F7F7|F--JL--JF-J||FJFJF-JFJ|L-J|L-J||FFJL-7LJ||L7|LJJF7F77LJLJL--JFJLJL-JL--JLJJ.|7-F-|J.L|.L
LFF|-|LLL-F-----JLJL7F7L7|||FJF7F---7||LJJ-LJLLJLJLJL-7F-7F7L7FJLJFJ7|F7L7|F--J.F-JL7|F-7L-7||FJ|F--JLJL--7F7F--7L-----------7L-.LJF-|F7FJ-.
7.LLFJ.|.|L--------7LJ|FJLJ|L-JLJF--J|L--7F--7LF7F7F--JL7||L-JL--7L-7|||FJ||F---JF-7|||FJF-JLJL7|L--7F7F-7LJ|L-7L--7F7F7F7F7FJLLJLFL.LJ|7L7F
FJ.F|.7|-LJF7F7F7F7L-7LJF-7L----7L-7FJF--J|F7L-JLJ|L7F-7|||F-----JF-JLJ||FJ|L-7F7L7||||L7L--7F7LJF7|||LJ7L7.L-7L--7||LJLJLJLJF7J|.LJFJFJJ-|7
LJ|-|-F7.LFJLJLJLJL--JF-JLL----7L--J|FJF77||L7F---JJLJL||LJ|F--7F7|JF7JLJL7|F-J||FJ||||FJF7FJ|L--JL-J|F---JF7FL--7||L----7F--J|.-7J77.7FF7LJ
J||.7-7JL.L7F--7F----7L7F--7F-7L----JL-JL-JL7LJF-7JF7LFJL7FJ|F-J||L-JL---7||L-7|||FJ|||L7||L7L7F7F-7FJL7F--JL7F7FLJ|F7F--J|F--J7L|JFL..FJ||.
F7-.JFJJL7JLJF7LJF7|FJFJL-7|L7L7F---7F--7F--JF7L7L7|L7L7FJ|FJ|F7|L7F7F-7FJLJF-J|LJL-J|L7LJ|FJ7LJLJ|LJF7LJF--7LJ|F-7LJ|L---JL------7L|-7-F7-|
|LFFJL7F7F---JL--JL-JFJ.F-J|.|FJL--7|L7FJL---JL7|FJL7|LLJFJL7LJLJFJ||L7LJ-F-JF7L7F7F-JFJF-JL--7F-----JL--JF7L-7LJFJF7L-------7F---J|F-|-J.||
F7|J.LLJ-L---7F7F-7F-JF7L-7L-JL----JL-JL-7F7F-7LJL--JL7F-JF7L-7F-JFJL7|F-7L-7|L7LJ||F7L7L7F7F7||F------7F-JL-7L-7L-JL--7F---7|L7F7FF77L-J.||
LF|-L-J7FF7LFJ|LJFJ|F-J|F-JF---7F7F----7FJ|||FJF7F--7FJL7FJ|F7||F7|F-JLJFJF-JL7|F-J|||FJ.LJ||||LJF----7LJF---JF7L----7FJ|F--JL-J|L-J|77J--FJ
-7J-J.LF7|L-JFJF7L-JL-7LJF7L-7FJ|||F---JL7|LJL7||L7JLJF-J|FJ|LJLJLJL--7FJFJF7FJ|L-7|||L7F--J|LJ7FJF-7FJF7L----J|F---7|L-JL7JF--7|F--JFJ-FJFJ
FJF-.FL|LJF-7|FJL-----JF-JL7FJL-JLJL---7-|L-7FJ|L7L--7L7FJL7L7F-------JL7|FJ|L7|F-JLJL7|L-7FJLF7L-JFJL-JL-----7LJF-7||F7F7|FJF7LJL-7L|L7J.J7
J-L7L77L7FJ.||L7F------J.F7||F--------7L7|F-JL7L7L7F7L7LJF-JFJL--7F7F7F7|LJFJFJ||F7.F7LJFFJ||FJL7F7L---------7L7FJ.|||||||LJFJL-7F7|-|7LF7-F
|.F7J--FJ|F-J|7LJ-F-7F7F-JLJ|L-------7|FJ||F7JL-JJLJ|FJF7L7FJ|F--J||||||L-7L7|FJLJL7|L---JFJFJF7LJL----7FF---J7|L-7||LJLJ|F-JF-7LJLJ..FF-J7J
|FFJ|L-L-JL7FJ-JF7L7|||L--7FJF7F-----JLJ7LJ|L77F7F-7|L-JL7|L-7L7F7|LJ|||F-J7||L7F--JL----7|7L-JL----7F7L7L--7F7L-7||L---7||F7|FJ|||..FF.FFFF
-FF7F--JJLFJL7FFJL-JLJL---JL7||L----------7L7L7|LJFJL-7F7||F-J|LJ|L-7|||L--7|L7|L-7.FF--7LJF7F7F----J|L7|F7FJ|L-7|||F7F-JLJ|LJL-7JF--JF-LJJJ
LLJ-L-77|-L--JFJF---7F-7F---J||F----------J.|FJ|F-JF7FJ|LJ|L7JF7FJF7||||F7FJL7|L7FJ7-L-7|F7||||L-7F-7|||LJ|L-JF-JLJLJLJF-7FJF---JJJ7.||F|J-7
F|.F7|LFL7LL|7L-JF7FJ|JLJF---J|L------7F7F7FJ||||F7||L7L7FJFJFJLJFJ|||||||L7FJ|-LJJL|F-JLJLJLJL-7LJFJ|FJF7L---JF7-F---7L7||FJFF777FF7JJ-F7F|
7LL.|LL|F7.77F---JLJFJF--JF7F7L-------J|LJ|L7L-JLJLJL7L-JL7L7L7F7L7||||LJ|FJL7|7LF7F7L------7F-7L-7L-JL-JL-7F--JL7L7F-JFJLJL--J|F--J|7|F|L.|
L-|J.FFFFF-77L------J|L---JLJL---------JF7L7|F7F7F7F7L7F7JL7|FJ|L7|LJLJJL||JJLJF-JLJ|F7F7F7FJL7L-7L7F-----7LJF7F-JFJL7|L7F-7F--J|F--J-J7-J-F
LJFJ|-7.||FJF7.F--7F7F--------------7F--JL-J||LJ|||||FJ||F7|||FJFJL-7JJL|LJJJFFL---7LJLJLJ|L--JF7|FJL----7L--JLJF-JF7L--JL7|L---JL-7L|LJ||LJ
|FJF|J|F-JL7|L7L-7|||L-------------7LJ|F----JL77|||||L7|LJ|||LJL|F-7L77L-7L|.F---7LL--7F-7|F7FFJLJL-7F---JF7F---JF-JL7F-7FJL-7F7F7FJ.-7LJ7.|
-F--L.FL--7|L7|F-JLJL7F7F---7F-7F--JF--JF7F7F7L7LJLJL7LJF-JLJJJF||JL7L7-.JFF-L--7|F--7||L|LJL7|F---7|L----JLJF7F7L--7LJF|L--7LJLJ||.|7L77F.7
F77|L|JF--JL7||L----7|||L--7|L7|L---JF7FJ|||||FJF-7F7L-7L7J.LLFFJ|.F|FJ7FFFJFF77||L7FJLJFJF-7|LJ7F-J|F7F-----JLJ|F--JF-7L--7|F7F7LJF-7|.||F7
LL-77F-L---7||||F-7FJLJL---JL7||F-7F-JLJFJ|||LJFJFJ||F7L-J7FF-7L-J.FJ||FJF|FFJL-JL-J|F7|L-JFJL--7L-7|||L-------7|L--7|FJF-7LJ|LJ|F-JFJ77L7-J
|-LF7.FF---JLJL7L7||F7F-7F7F7LJLJFJL-7F7L-J|L-7|FJF|LJL-----JFJ7L|F|FJ7J-FL7L7F----7LJL----JF7F7L--JLJL--------JL-7FJ|L-JFJF7|F-JL-7|F-77.|.
L7.JJFFJF---7F7L-JLJ|||7LJ|||F--7L--7LJ|F7.L--J||F7|F--7F-7F-J|L-7-LJFJ|J|LJLLJLF--JF7F7F7F-J||L7F7F7F7F---------7LJFJF-7L-JLJL7F7J|LJFJ7F|7
F77-FJL-JLF7LJL----7|LJF--J|||F-JF-7|F7LJL-7F7FJLJLJL-7LJ7LJ7L|L|JJ.FJFFF77|JLJ7L7F7|||LJLJFFJL7||LJLJ|L7F-7F7F7FJF7|FJF|F7F7F7LJ|FJF7L--777
|JFF|.F7-FJL-------JL-7L7F7|||L7FJFJ|||F---J|||F-----7L-7F--7.L|||-FFJLJF-J.FL7|-||||||F----JF7|LJF---JFJ|JLJLJLJ-|LJL-7LJLJLJ|F7LJFJL---J.|
LFJFL7|L-JF-7F7F7F-7F-J-LJ|||L7||.L7LJLJF7-FJLJL--7F7L-7LJF-J..|LF-L.F..|.|7LLJJ7LJ||||L7F7F7|LJF7L----JFL--7F-7F-JF---JF7F77FJ|L7FJJF----77
||-LJ-L7F7L7LJLJLJFJL7F7F7LJL-JLJF7L7F7FJL7|F-----J||F7L--JJ7|7LF-7|7|.F-||L-7|FLF-J|LJFJ|LJ|L7FJL7F7F-7F---JL7LJF-JF7F7|LJL7L7|FJL--JF7F-J7
L77.|-LLJL-JF7F--7L--J||||F---7F-JL-J||L7FJ|||F7FF-JLJL7.F7-JLJLL-7FJ-7|.LLJ-FL|-L--JF-JFJF7L-JL-7LJ||FJL-7F-7|F-JF-JLJLJF-7|FJ|L7F--7||L7J|
FLF.||LLL||FJLJF7|F-7.||||L--7|L-----JL-JL-JL-JL-JF7F-7L7|||L..||.L7JLFFJ7FFF-7|L|L.|L-7|FJ|F7F-7L-7LJL77FJL7LJL-7L-7F---JF||L-JFJL-7|||FJ7|
||J||7.L||FJF7FJ||L7L-JLJ|F7.||F7F7F7F-7F--------7||L7L7LJ|7-LF77FF|LFF7.J-J|JLJ.7FFFF7LJL7LJ||FJF-JF-7L-JF7|F---JF7|L----7LJF--JF7FJLJ|L7L-
--.LFF--FFL-JLJFJL7|F---7||L-J||LJLJLJ||L-------7|||FJFJF7L77.JJLJ7J|FJ|-J||.FJ.FF7FFJL7F-JF7LJL7L--J|L---JLJL----JLJF-7F-JF7L--7|LJJ-L|FJ.|
|F--J|.LF------JF7LJ|F7FJ|L--7|L7F-7F-7|F-------JLJLJ-L-J|FJ7.F|LLL77L7|.||J7|F7FJL7|F7LJF-JL7F7L-----------7F-----7FJ.LJF7|L7F7LJF7.FFLJ|--
7F|F||.FL------7|L--J||L7L-7FJL7LJ-LJFJ|L-----7F7F7F77F7||L7J7|77FLF-FJL7-L---|||F7LJ|L7FJF--J||F----------7|L----7LJF7F7||L7LJL--JL7||FL-.|
LLLLF7J7F----7FJ|F---J|FL7FJL7FJF----J|L7F----J|LJ|||FJL7L-J7LJFJ|LF-JF-J.|7JF||LJL7FJJLJFJF-7|||F---7F7F--JL--7F7|F7|||LJL7L7F-7F-7L-7JLL-|
|JFF-JFFL---7|L-JL---7|F-J|F7||JL-----7FJL-----JF7LJLJF-J-J-|77F7J7L7FJJLLJ7.FJ|F77LJF--7L7L7LJLJL--7|||L----7FLJ|LJLJLJF--JFJL7|L7|F-JFF7F7
7-L|..FF----JL-7F----JLJF7LJLJ|F7F7F7FJL7F----7FJL7F--JF7|||F7FJ|JFFJL-77|JLFL7|||F7|L-7L7L-J7F---7FJLJL--7F7L--7L7F-7F7|F77|F-J|FJ|L-7F77.|
|FLF.F-L----7F-JL7F-7F7FJL7F-7LJLJLJLJF7LJF7F7LJFFJL---JL7F7||L7L7FJF--J-F7JF7|LJLJ|F--JFJ-F--JF-7|L7F---7LJL---J7LJ-LJLJ||FJL-7LJL|F7LJL7.L
|J-|-7J.F---J||F7LJFJ|LJF7LJFJF7F--7F7||F7|LJL7F7L7F7F---J|||L7L7|L7|F77.|L-J|L-7F-JL--7|F7L---JFJL-J||F7L------7F--7LF7FJLJF--JF-7|||F7FJ7|
--7-.LFLL7F-7L-J|F7L-JF-JL--JFJ||F-J|||||||F-7LJL7||||F-7FJLJFJFJL-JLJ|F7|F--J7FJL-7F7FJ||L7F7F7L----JFJL-------JL7FJFJLJF--J|F-JFJ||||||-J-
LFF7F-7LLLJ.L7F7LJL---JF7F7F7L7LJL--JLJLJLJL7L-7FJLJ||L7LJF--J.L----7FJ||||F7F7L7F-J|||FJL7||||L------JF------7F7FJL-JF--JF7F7L7FJ|LJLJLJF|.
F7LLF7F7.LF--J||F------JLJ||L7|F-------7F7F7L-7||F7JLJ7L-7L-7F----7FJL7|||||||L-JL--JLJ|LFJLJ|L-7F--7F7|F-----J|LJF7F7L--7|||L-JL-7J.||.L7.7
LL-LFJL--JL---JLJF7F------JL7LJL-----7JLJ||L7FJLJ|L7F7F--JF-JL---7|L-7LJLJLJ|L-7F-----7L7|F7FJF7LJF7LJLJL7F7F--JF-JLJL---J||L7F---JJF|-L.LFJ
FL7..J-|7|-LF----JLJF-------JF7.F7JF-JF-7|L7LJLF-JFJ||L7F-JF7-F--JL--JF-----JF7LJF-7F7|FJ|||L7|L7FJL--7F7LJLJF--JLF7F7F--7||.|L-7F777LJ.|JLJ
|FJJ7|7|-7-FJF-7F7F7|F7F7F7|FJ|FJL-JF-JFJL7L--7|F-JFJL-JL-7|L7L------7L---7F-J|F7L7||LJL7LJ|FJL7LJF7F7LJL----JF7F7|||||F-J|L-JF7LJL77LF77.F|
LJ|JFJJJF7|L-J|LJLJLJ|||LJL7|FJL---7|F-JF7L---J|L7|L7F----JL7|F7F7F-7L7F--JL-7LJL7||L--7L--JL7JL--JLJL--------JLJLJLJLJL-7|F--JL---JF7-J.FF|
7L--L.LL7LF--7F7FF---J||F-7LJL----7LJL--JL---7.|FJF-JL--7LF7||||||L7L-JL7F-7FL-7FJ||7F-JF--7FJF7|F7F---7F---7F7F-7F----7FJ|||F7LF7F7||77-FJ|
..L|-F7.|FL-7|||FJF7F7LJL7L7F7F--7L-7F7F7F---JFJL7|F7F--JFJ||LJ||L7L-7F-J|FJF7FJL7|L7L-7|F7LJJ|L-J|L--7|L--7||||FJL---7LJFJL-JL-JLJLJL-77LF|
FFJ|FLJ-L-F7||||L-JLJL7F7|FLJLJF7L-7|||||L---7L7FJLJ||F7|L7|L7FJ|FJF-JL7F||FJ|L-7LJFJF7|||||F7L-7FJF7FJL7F-JLJLJL-7F--JF7L7F7F7F7F7F-7FJ7-||
L|.L|JLF-L|LJLJL---7F-J|LJF----JL--JLJLJL-7F7|J||F7FJ||L7L|L-J|FJL7L--7L-J||FJF7|F-JFJ||LJL7|L7FJL7||L-7LJ|F-7F7F7LJF--JL-J|LJLJLJ|L7LJL-7J7
FFJL77-J|LL7F-7F--7LJF-JF7|F-7F----7|F7LF7LJLJFJLJ|L7LJFJFJF-7LJF-JF7|L7F-J|L-J|||F7L7LJF7FJL7LJF7LJ|F7L--7|FJ||||F7L7F----JF----7L-J-L..L7|
JJ7.||.LL.L|L7LJF7L--JF-J|LJ|LJF7F7L-JL7||F-7FJF7FJFJF-J.L-JFJF7|F7||F-JL-7L--7||LJL7L-7|LJF7L--JL7FJ||F7FJ||FJLJLJL-JL-7F7FJF7F7L---7.F7.|.
FLL7-J7J7|FL-J7FJL7F7FJF7L7F77FJLJL-7F7LJ||FJL7|LJJL7|F77|F7L7|LJ|||||F7F-JF--JLJF--JF-JL-7|L7LF7-|L7|LJLJ-|LJF7F------7LJLJFJLJL-7F7|7JFJ77
F7L|F-7LFF-7F7FJF7LJ|L-JL7|||FJF---7LJL--J|L--JL7.F7|||L7FJL-J|F7|LJ|||LJF7|F---7|F7JL---7LJFJFJL7L7LJF----JF-J|L-----7|F7F-JF--7LLJLJ.FJFL-
||F-7-.FJL7LJ|L-JL7FJF7F7||||L-JLF7L-----7L7F--7|FJ||LJFJL7F--J||L-7||L7FJ|LJF7FJLJL7F7F7|F-JFJF-JFJF7|F----JF7L-7F---JLJ|L--JF7L-7.JF-|FL77
L7F...FJF-JF7L----JL-JLJLJLJ|F7F-JL------JFJL-7LJL7||F-JF7|L--7|L7FJLJFJ|FJF7||L7F--J|LJLJL7.L7|F7L7|LJL---7FJL7FJL-----7|F-7FJ|F-JJ7|JL|FJ-
L||F----L7FJL----7F---7F-7F7LJLJF--7F7F-7|L7F-JF-7|LJ|F-J||F--JL7|L7F-J-|L7|LJL7|L--7|F7F--JF7||||7||F----7LJF-JL7F7F7F7||L7||F|L7JF-L-7-LJ.
-J||-F|-FJ|F-7.F7||F--J||LJL7F7FJF7LJ||FJF-J|F7|FJL7FJL-7LJL-7FFJ|FJ|F-7|FJL--7|L7F-JLJ|||F7|LJ||L7LJL---7L--JF7|LJ||LJ||L-J|L7L-JFJ||LLJF77
.-F7L-F7L7|L7L-JLJ|L7F7L7F--J|LJ.||F-J|L7L-7||LJ|F7|L7F-JF---JFJFJL7||FJ|L7F7FJ|FJL7F7FJ|FJ||F-JL7|F-7F--JF---JL--7|L-7LJF7LL-JF7FJFF-777|J7
.|J.|.|||LJ-L7F7F-JJLJL7||F--J|F-JLJF7|FJF7|||F-J|||FJL-7|F7F7L7|F7||||FL7|||L7||F-J||L7||FJ||JF7|||FJL---JF7F7F--JL-7L--JL----JL7LF-7LJ7JLF
FF7F--|L7F7F7LJLJF7F7F7LJ|L7F--JF7F-J|||FJLJ||L7FJ||L7F7|||||L7|||||||L7FJLJL7|LJL7FJL7|LJL7||FJ||LJL-7F-7FJLJ|L----7L-7F-7F-----JLJ|F7JJ-|J
FJ-J.FL7||LJL7F7FJLJLJL--JFJL7F7|LJF7||LJF-7|L7|L7LJ-||||||||FJ|||||LJ-|L7JF-JL7F-J|F7|L7F7|||L7||F7-FJL7|L--7|F7F-7L-7LJFJL7F--7J.L7|J|--7|
|.L-FF7||L--7LJLJF7F7F7F7FJF-J||L7FJLJL--JFJ|FJL7L-7FJ|||||||L7|LJ|L--7|FJFJF--JL-7LJ||FJ|LJ|L7||||L-JF-J|F-7|LJ|L7|F7L-7|F7LJF-J-7FFJ|L.|L-
|77JF|LJ|F-7L7F-7|||||LJLJ7|F7|L-JL--7F-7FJ-|L-7L7FJL7|||LJ|L7||F-JF-7|LJJL-J-F---JF-J|L7L-7L7LJ||L--7L-7LJ-|L-7L7|||L--JLJL7FJJJLLFL-7|.77|
||.FJL-7|L7||LJ.||LJ|L7F7F-J|LJF-----JL7LJF7L-7|FJ||FJ||L-7|FJ|||F7|FJL----7F-JF7F7|F7L-J7FJFJF-JL7F-JF-JF-7|F-JFJ||L7-F7F-7LJ-|JFFL|..FF77J
LLF-FF-JL-J|F7F-JL-7|FJ|LJF7L-7|F--7F-7L--JL-7LJL7|FJFJL7FJLJLLJ||||L-7F-7FJL-7||||LJL---7L7|F|F--JL-7L--JFJLJF7L7|L7L-JLJFJJ|FL--7FL7.L|JF7
.L|L-L----7|||L-7F-J|L7L7FJ|F-JLJJFJL7L-7F7F7L-7FJ|L7L7FJL-7F---J|||F-JL7LJF--J|LJL7F--7FJFJL7||F--7FJF7F-JF--JL7|L7|F----JJL-L-7|LLJL7-J-|F
F-|FLF----JLJL--J|F-JFJFJ|FJ|.F---JF-JF7LJ||L--JL7L7L-J|F7FJ|F--7|LJ|F7FJF-JF-7|F--JL-7|L7L7FJLJL-7LJFJ|L--JF---J|FJ|L---7F77||7.JFF|7.FJFJJ
FL||JL------7F-7FJ|F7|FJFJL7|FJF7F7|F-JL-7|L-7|F-JFJLF-J|||-LJF-J|F-J||L7L-7|FJ||F7F--JL7|J||LF---JF7|JL7F-7L---7||FJF7F7LJL7-LL.|F-||.L7J77
FJFJF-------JL7|L7LJLJL7|F7LJL7||||||F7F7LJF7L7|F7L-7L7FJ|L7F-JF7|L-7||FJF7||L-J|||L---7||FJL7L---7||L77||-|F-7FJLJL7|||L-7FJFJFF7J-|L7.LLJ7
..LFL-----7F7FJL7L--7F-JLJ|F--J|||||||LJL-7|L7|LJ|F7L7||J|FJL-7|||F-J||L7||LJF--J||F-7FJLJ|F7|F7LFJ|L7L7|L7|L7|L---7|||L-7|L777FL|JFL-FF-7.F
J7.F------J||||FJF7FJ|F7F7|L-7FJLJ||||F---JL7LJ7FJ||FJ|L7||F7FJ|||L7FJL7LJ|F7L--7||L7|L--7|||||L-JFJFJFJL7||FJ|F-7FJ||L7FJ|FJ7--|JF-7J|77L-7
|L-L-7F7F--J|L7|FJ||FJ|||LJF-J|F--J|||L7F-7FJ|F7L-J||FJFJ|||LJFJ|||LJ.|L--J||LF7||L7||F--J|||||F7FJLL7|-FJ|LJ-|L7|||LJJLJ-LJJJ7.|-F.|-7LJFLL
F7LF-J|||F--JFJ||FJ|L7|||FFJF7|L7F-J||FJ|FJL--JL--7||L-JFJ||F7|FJL-7F7F----JL-JLJL7LJ|L--7||LJ||LJF7FJL7L7L--7|FJ|L-7-7FJJLLJFLJ.|.F|.LFFJFJ
F--L-7||||F-7|||||FJFJ|LJFJFJ||.LJF7LJL7|L7F7F7F7FJLJFF-JFJLJLJ|F7FJ||L--7F----7F7L-7|F7FJ||F-JL--J||F7L7|F--J|L7|F7L-7JJF77JL7L-7-JFJ|L-LJ7
7|-J-LJLJLJFJL7LJLJ|L7L-7L7|FJ|F--JL---JL7|||||||L-7LFJF7L-77F-J|||F||JF-J|F--7|||F-J||||F||L7F7F-7||||FJ|L--7|FJ|||F-JLLJL|7-L-L7-7.L-J|-F|
LFJ|F|7F---JF7L--7F7L|F-JLLJL7|L-7F-7F7F7|||||LJ|F7L7|FJ|F7|FJF7||L-JL7L7FJL-7LJ||L-7LJ|L7||FJ||L7||LJ||FJF-7|||LLJLJJ..7J7.JLL7J.FL7.L7J7|J
.7.|FFFJF-7FJ|F7FJ||FLJJ.FL7|||F-J||LJ||||LJ|L7FJ|L7||||||LJ|FJ||L7F7FJJ||F--JF7||F7L-7|FJ|||FJ|FJ|L-7LJL7L7|||L-7JFJJF7J.|J.FL|---|.J-J7FJ7
FL.L--L-JFJ|||||L-JL77|F-JFFFLJL-7|F--J||L7L|FJ|FJFJ|LJFJL-7||J||FJ|||F-J||F7FJLJ||L7FJ|L7LJ||.||FJF7L7F7|FJ||L--JF|7.FF.F|J.LJJ.F|.-77F-LFJ
L--JL77F-JFJFJ||F7F7L7-F.FFF7F|F-J|L-7FJL-JFJL7||-L7|JFJF7FJLJFJ||FJ||L-7|LJLJF--JL7|L7L7L7FJL7LJL7|L7LJ||L7|L-7LL-J|--L-|-7FL7-7J-LJF|-JLJ|
|L7-F|FL-7|FJFJ||||L7L-7F7FJ|F7L-7|JFJL7JF-JF-J|L-7|L7L7||L7F7L7||L7||F-JL7F--JF7F7||FJ7L-JL-7|F77||FL7FJ|FJL-7L7F|LL7-FF7FL7JLJJJF||FJ..FFL
|-|FFL-LJLJ|FJL||LJFJF-J||L7LJL--J|FJF-JFJF7L7.|F7||FJ.|||FJ|L-J|L7|LJL--7|L--7|||LJ||-F7F77FJLJL7|L7FJ|FJL-7JL-J-|LLJ7LLJ|.||..LF7-J|FFF7LJ
J-L7J|L|J..||7-||7FL-JJFJL7L7F-7F-JL7|FFJFJ|FJFJ|LJLJF-J||L7L7F7|FJ|F7F--J|LF-J||L7FJL7|||L7L7F7FJ|FJL7|L7F-JF7L|F|J.|7-|-J-LL-|FLL--7-JFJ.7
..LJ|--7JFF|L7F||-J|7|.L-7L-JL7LJLF-J|FJFJFJ|FL7L--7JL7FJ|FJFLJ||L-J|LJF-7L7|F7|L7||F7LJLJFJ.||LJFJL7-LJFJL--J|.F7J|7L|.L7J.F7FF|L|.L7|FL|.|
F.|FF.-|-LL|FJ-LJJ--L7FLJL7F7FJ.J.L-7||FJLL7L7FJF7FJ7-||FJ|F---JL--7|F7L7L7|||LJFJ||||F-7FJF7|L-7L7FJ-LLL7F7F-J7L|LF--FJ--.FJ7.FF.||7.|J-|F7
.FJ|.F||7L.LJ7.LLJFLL-7-FFJ|LJJ-|7F7LJ|L-7-|FJ|FJ|L--7|||FJL-7F-7F-JLJL7|.LJ||F-JFJ||LJFJL-J|L-7|FJL777FL||||J-L-7.JLL||FJ-J.FF|J-J7.|7F-J-.
.LFF7JL-|F7|.L-FJ.|.L||FFL7L--7.-F|L--JF-JFJL7|L7L7F7|LJLJF--J|FJL-7JLFLJ-F7||L--JFJL-7L-7F7L7-LJL7FJLF7-LJLJ|7F|77F-..-JJ.L7L|||.FLL-77-|77
--7LF.||L|.J-|LL-JJ.F7LJJ.L-7FJ.|LL7F-7|F7L7FJ|FJ.||LJ.|.LL--7||F7FJ7LJ||FJLJL7F--JF-7L-7LJL7L--7FJL--J|FLFJ.L7|JLLJJ-FJ|FL-7||JJ--L7FLJJLL7
F7|J.LF|7.F|-F.L77.F--L|L-7LLJ-|-77LJFJLJL7|L7|L7-LJLJ--7L7LFLJLJLJFF7|FFJF7F7|L-7FJFJF7L7F7L--7|L7F7F-J7-J7F-|L7..|FLF--|.-J7|J||LJF-J--JF|
J77FFJ|.|FL7J.F7.FLJ||7F7LJFJFLL-J--FJF-7FJ|FJ|FJ7JFF7F-JL|-|F|J7JL-LJLFL-JLJLJF-JL7L7|L7LJL7F7LJFLJLJJ|LFL|.FJ.L|-L|-7JFF7|JF|.FJ7.7J.F.FLJ
FL7-7-LL-JF7.FF--|-F--LL-J|7.|.LF.|LL7L7||FLJL||JJ7FJ|F-JJ|7J7|L7.F-|-L|.L|7F--JF7FJFJ|FJF-7LJL--7||7-F-J|LLLLFJ.|.7-F-7LF-|77.|J7F|J.F|-7.F
7J.L|.L|FLLF-7|.L|-J7J.|F-FFJJ-F--7-L|FJ|L-7JJLJ.J7L-LFJJ-7JJF7LL|J7F-7F7-LLL---J|L7L7|L7L7|F--7FJ-F77FJ-FF7JF-J.|-.7|L|-|LJJ-L|77J-7FLJLJ.7
|7|.F7.FJFLLFJ7-||F-77F|J-F|J.FL|-FJ.LJ-L--J.FJ|.FJ77||.LF|F77F-LL-FF-JLL7..|F---JFJFJL7L7|||7L|L7J.|7..-|J|L|7.F|J-J7|||.LLLJ7|.-7LL-.L-J-J
77J|JJ7L.|JJ..J-7-F7|7F|-L||7F-7|.F-F|J.LJL|-77F-F-JF|7..F|7|7|FL|LLL7LJFJFF-|F7F7L7|F7L7|||L7.|FJJJLLL|.|7|.L7F-LJFF|F--7.||L||FJ||J|L-FJ7L
L--LJJ|L-L7.FLJ..|JL7-LL-L.FLJJ.--JLL7J.L|-JJJF-JL7JF|-..JJF|--7.|LL-JLL|J---LJLJL-JLJL-JLJL-J-LJJ.LJ.|J-L7--JL-|L7-LL|-J-|-JJL-J.L|LF|LLJL."
}
