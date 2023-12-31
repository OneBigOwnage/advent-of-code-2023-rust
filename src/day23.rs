use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Path {
    from: Point2D,
    to: Point2D,
    length: usize,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "From {} to {}. Distance: {}",
            self.from, self.to, self.length
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone)]
struct HikingTrail {
    start: Point2D,
    end: Point2D,
    terrain: Vec<Vec<Terrain>>,
    paths: Vec<Path>,
    width: usize,
    height: usize,
}

impl HikingTrail {
    fn valid_neighbors_while_taking_slopes_into_consideration(
        &self,
        path: &Vec<Point2D>,
        current_position: &Point2D,
    ) -> Vec<Point2D> {
        // If we're currently on a slope, the slope dictates our only valid next step.
        if let Terrain::Slope(direction) = &self.terrain[current_position.y][current_position.x] {
            let next_position = match direction {
                Direction::Up => Point2D {
                    x: current_position.x,
                    y: current_position.y - 1,
                },
                Direction::Right => Point2D {
                    x: current_position.x + 1,
                    y: current_position.y,
                },
                Direction::Down => Point2D {
                    x: current_position.x,
                    y: current_position.y + 1,
                },
                Direction::Left => Point2D {
                    x: current_position.x - 1,
                    y: current_position.y,
                },
            };

            // The steps is only valid if we haven't been there before.
            if path.contains(&next_position) {
                return vec![];
            } else {
                return vec![next_position];
            }
        };

        // If we're not on a slope, we can move in any direction where there isn't a forest and we
        // haven't been there before.
        self.valid_neighbors(path, current_position)
    }

    fn valid_neighbors(&self, path: &Vec<Point2D>, current_position: &Point2D) -> Vec<Point2D> {
        let mut neighbor_paths = self.neighbor_paths(*current_position);
        neighbor_paths.retain(|neighbor| !path.contains(neighbor));
        neighbor_paths
    }

    fn neighbor_paths(&self, point: Point2D) -> Vec<Point2D> {
        let mut valid_neighbors = vec![];

        if point.y > 0 && self.terrain[point.y - 1][point.x] != Terrain::Forest {
            valid_neighbors.push(Point2D {
                x: point.x,
                y: point.y - 1,
            });
        };

        if point.x < self.width - 1 && self.terrain[point.y][point.x + 1] != Terrain::Forest {
            valid_neighbors.push(Point2D {
                x: point.x + 1,
                y: point.y,
            });
        };

        if point.y < self.height - 1 && self.terrain[point.y + 1][point.x] != Terrain::Forest {
            valid_neighbors.push(Point2D {
                x: point.x,
                y: point.y + 1,
            });
        };

        if point.x > 0 && self.terrain[point.y][point.x - 1] != Terrain::Forest {
            valid_neighbors.push(Point2D {
                x: point.x - 1,
                y: point.y,
            });
        };

        valid_neighbors
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PathInfo {
    current_position: Point2D,
    path: Vec<Path>,
    length: usize,
}

impl PathInfo {
    fn have_we_visited_this_location(&self, location: Point2D) -> bool {
        if self.path.is_empty() {
            return false;
        }

        let mut legs = vec![self.path.first().unwrap().from];

        for i in 0..self.path.len() {
            legs.push(self.path[i].to);
        }

        legs.contains(&location)
    }
}

fn main() {
    assert_eq!(94, part1(&test_input()));
    assert_eq!(2310, part1(&input()));
    assert_eq!(154, part2(&test_input()));
    assert_eq!(6738, part2(&input()));
}

fn part1(input: &str) -> usize {
    let trail = parse(input);

    let mut paths_to_end: Vec<Vec<Point2D>> = vec![];

    let mut stack: Vec<Vec<Point2D>> = vec![];
    stack.push(vec![trail.start]);

    while let Some(path) = stack.pop() {
        for neighbor in trail
            .valid_neighbors_while_taking_slopes_into_consideration(&path, path.last().unwrap())
        {
            let mut new_path = path.clone();
            new_path.push(neighbor);
            if neighbor == trail.end {
                paths_to_end.push(new_path);
            } else {
                stack.push(new_path);
            }
        }
    }

    paths_to_end.iter().map(|path| path.len()).max().unwrap() - 1
}

fn part2(input: &str) -> usize {
    let trail = parse(input);

    let mut longest_hiking_route: Option<PathInfo> = None;

    let mut stack: Vec<PathInfo> = vec![];
    stack.push(PathInfo {
        current_position: trail.start,
        path: vec![],
        length: 0,
    });

    while let Some(path_info) = stack.pop() {
        for path in trail
            .paths
            .iter()
            .filter(|path| path.from == path_info.current_position)
            .filter(|path| !path_info.have_we_visited_this_location(path.to))
        {
            let mut new_path_info = path_info.clone();
            new_path_info.length += path.length;
            new_path_info.path.push(*path);
            new_path_info.current_position = path.to;
            if path.to == trail.end {
                if let Some(ref route) = longest_hiking_route {
                    if new_path_info.length > route.length {
                        longest_hiking_route = Some(new_path_info);
                    }
                } else {
                    longest_hiking_route = Some(new_path_info);
                }
            } else {
                stack.push(new_path_info);
            }
        }
    }

    longest_hiking_route.unwrap().length
}

fn parse(input: &str) -> HikingTrail {
    let terrain = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Terrain::Path,
                    '#' => Terrain::Forest,
                    '^' => Terrain::Slope(Direction::Up),
                    '>' => Terrain::Slope(Direction::Right),
                    'v' => Terrain::Slope(Direction::Down),
                    '<' => Terrain::Slope(Direction::Left),
                    other => panic!("Cannot parse {other} as a valid terrain"),
                })
                .collect::<Vec<Terrain>>()
        })
        .collect::<Vec<Vec<Terrain>>>();

    let width = terrain[0].len();
    let height = terrain.len();

    let mut trail = HikingTrail {
        start: Point2D { x: 1, y: 0 },
        end: Point2D {
            x: terrain[0].len() - 2,
            y: terrain.len() - 1,
        },
        terrain,
        paths: vec![],
        width,
        height,
    };

    let mut crossroads = vec![trail.start, trail.end];

    for y in 0..height {
        for x in 0..width {
            if trail.terrain[y][x] == Terrain::Forest {
                continue;
            }

            let point = Point2D { x, y };
            if trail.neighbor_paths(point).len() > 2 {
                crossroads.push(point);
            }
        }
    }

    let mut paths = vec![];

    for i in 0..crossroads.len() {
        let new_paths = find_accesible_neighboring_crossroads(&trail, &crossroads, &crossroads[i]);

        paths.extend(new_paths);
    }

    trail.paths = paths;

    trail
}

fn find_accesible_neighboring_crossroads(
    trail: &HikingTrail,
    all_crossroads: &Vec<Point2D>,
    current: &Point2D,
) -> Vec<Path> {
    let mut paths_to_neighboring_crossroads: Vec<Path> = vec![];

    let mut stack: Vec<Vec<Point2D>> = vec![];
    stack.push(vec![*current]);

    while let Some(path) = stack.pop() {
        for neighbor in trail.valid_neighbors(&path, path.last().unwrap()) {
            let mut new_path = path.clone();
            new_path.push(neighbor);
            if all_crossroads.contains(&neighbor) {
                paths_to_neighboring_crossroads.push(Path {
                    from: *current,
                    to: neighbor,
                    length: new_path.len() - 1,
                });
            } else {
                stack.push(new_path);
            }
        }
    }

    paths_to_neighboring_crossroads.retain(|path| path.from != path.to);
    paths_to_neighboring_crossroads
}

#[allow(dead_code)]
fn test_input() -> String {
    "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "#.###########################################################################################################################################
#.......#...#...###...#.....#...#.....#...#####...#...............#...#...#...#.......#.......#...###...#####.......###.........###.......###
#######.#.#.#.#.###.#.#.###.#.#.#.###.#.#.#####.#.#.#############.#.#.#.#.#.#.#.#####.#.#####.#.#.###.#.#####.#####.###.#######.###.#####.###
#.......#.#.#.#.#...#.#...#.#.#.#...#.#.#.....#.#.#...........#...#.#.#.#.#.#.#.#.....#.....#...#.....#...#...#...#.....#.......#...#...#...#
#.#######.#.#.#.#.###.###.#.#.#.###.#.#.#####.#.#.###########.#.###.#.#.#.#.#.#.#.#########.#############.#.###.#.#######.#######.###.#.###.#
#...#...#.#.#.#.#...#.....#...#...#.#...#.....#.#.#...###...#.#...#.#...#...#.#.#...#.>.>.#...#.........#.#...#.#.........###.....#...#.....#
###.#.#.#.#.#.#.###.#############.#.#####.#####.#.#.#.###.#.#.###.#.#########.#.###.#.#v#.###.#.#######.#.###.#.#############.#####.#########
###...#.#.#.#.#.###.......#...#...#.....#.....#.#.#.#.#...#...#...#...###.....#...#.#.#.#...#.#.#...###...#...#.......###...#...#...#.......#
#######.#.#.#.#.#########.#.#.#.#######.#####.#.#.#.#.#.#######.#####.###.#######.#.#.#.###.#.#.#.#.#######.#########.###.#.###.#.###.#####.#
#.......#.#.#.#.#...###...#.#...#...#...#.....#.#.#.#.#.......#.....#...#.....#...#...#...#.#.#.#.#...#.>.>.#...#.....#...#...#.#...#.#.....#
#.#######.#.#.#.#.#.###.###.#####.#.#.###.#####.#.#.#.#######.#####.###.#####.#.#########.#.#.#.#.###.#.#v###.#.#.#####.#####.#.###.#.#.#####
#.#.....#.#.#.#.#.#...#...#.>.>.#.#.#.#...#...#.#.#.#...###...#.....#...#.....#.#...###...#.#.#.#.#...#.#.#...#...#...#.#.....#.#...#.#...###
#.#.###.#.#.#.#.#.###.###.###v#.#.#.#.#.###.#.#.#.#.###.###.###.#####.###.#####.#.#.###.###.#.#.#.#.###.#.#.#######.#.#.#.#####.#.###.###.###
#.#.#...#.#.#.#.#...#.#...#...#.#.#.#.#.###.#.#.#.#.#...#...#...#...#.###...#...#.#.#...###...#...#.....#.#.#.....#.#.#.#.###...#...#...#.###
#.#.#.###.#.#.#v###.#.#.###.###.#.#.#.#.###.#.#.#.#.#.###.###.###.#.#.#####.#.###.#.#.###################.#.#.###.#.#.#.#.###.#####.###.#.###
#.#.#.#...#...#.>.#.#.#.###...#.#.#.#.#.#...#...#...#.>.>.###.#...#.#...#...#.#...#...#...#.......#.......#.#.#...#.#...#.###.#.....#...#...#
#.#.#.#.#######v#.#.#.#.#####.#.#.#.#.#.#.#############v#####.#.###.###.#.###.#.#######.#.#.#####.#.#######.#.#.###.#####.###.#.#####.#####.#
#.#.#...#.......#...#.#.#####.#.#.#.#.#.#.......###...#.....#.#...#.#...#...#.#.........#.#.#...#.#.......#.#.#...#.....#.###.#.#...#...#...#
#.#.#####.###########.#.#####.#.#.#.#.#.#######.###.#.#####.#.###.#.#.#####.#.###########.#.#.#.#.#######.#.#.###.#####.#.###.#.#.#.###.#.###
#...#...#...........#.#.#.....#...#...#.#.....#.#...#.......#...#.#.#...###...#.........#.#...#.#.#...#...#.#...#.#...#.#.#...#...#...#.#.###
#####.#.###########.#.#.#.#############.#.###.#.#.#############.#.#.###.#######.#######.#.#####.#.#.#.#.###.###.#.#.#.#.#.#.#########.#.#.###
#.....#.............#...#.............#.#...#...#...#.......###...#.....#.....#.......#.#.#.....#...#...###.#...#.#.#...#...#...#...#...#...#
#.###################################.#.###.#######.#.#####.#############.###.#######.#.#.#.###############.#.###.#.#########.#.#.#.#######.#
#.#...............#...#...............#.....###...#...#.....#...........#...#.....#...#...#...............#...###...###...#...#.#.#.......#.#
#.#.#############.#.#.#.#######################.#.#####.#####.#########.###.#####.#.#####################.#############.#.#.###.#.#######.#.#
#.#.#...#...#.....#.#.#.......###...#.....#...#.#.#...#.....#...#.......#...#...#...#...#...#.............#...#.....#...#.#...#.#.......#.#.#
#.#.#.#.#.#.#.#####.#.#######.###.#.#.###.#.#.#.#.#.#.#####.###.#.#######.###.#.#####.#.#.#.#.#############.#.#.###.#.###.###.#.#######.#.#.#
#.#.#.#...#...#...#.#.#...###...#.#.#.#...#.#.#.#.#.#.###...###.#...#...#.....#.......#...#.#.............#.#.#.#...#.#...#...#.........#.#.#
#.#.#.#########.#.#.#.#.#.#####.#.#.#.#.###.#.#.#.#.#.###v#####.###.#.#.###################.#############.#.#.#.#.###.#.###v#############.#.#
#.#.#.#...#...#.#.#.#.#.#.#...#...#...#...#.#.#.#...#.#.>.>.###.#...#.#.#...###...#.........#...#.........#.#...#...#.#.#.>.#...#...#####...#
#.#.#.#.#v#.#.#.#.#.#.#.#.#.#.###########v#.#.#.#####.#.#v#.###.#.###.#.#.#.###.#.#.#########.#.#.#########.#######.#.#.#.#v#.#.#.#.#########
#.#.#.#.#.>.#.#.#.#.#.#.#.#.#.#...#.....>.>.#.#.#...#...#.#.#...#...#.#.#.#...#.#.#...#.....#.#.#.........#.......#.#.#.#.#.#.#.#.#.........#
#.#.#.#.#v###.#.#.#.#.#.#.#.#.#.#.#.#####v###.#.#.#.#####.#.#.#####.#.#.#.###.#.#.###.#.###.#.#.#########.#######.#.#.#.#.#.#.#.#.#########.#
#...#...#...#.#.#.#.#.#.#.#.#.#.#.#.#.....#...#.#.#...#...#...###...#.#.#.#...#.#.#...#.#...#.#.#...###...#...#...#.#.#.#.#.#.#.#.#...#.....#
###########.#.#.#.#.#.#.#.#.#.#.#.#.#.#####.###.#.###.#.#########.###.#.#.#.###.#.#v###.#.###.#.#.#.###v###.#.#.###.#.#.#.#.#.#.#.#.#.#.#####
#...........#...#...#.#.#.#.#.#.#...#.....#.....#...#.#.....#.....#...#.#.#...#.#.>.>...#.#...#.#.#...>.>.#.#.#...#...#.#.#.#.#.#.#.#...#####
#.###################.#.#.#.#.#.#########.#########.#.#####.#.#####.###.#.###.#.###v#####.#.###.#.#####v#.#.#.###.#####.#.#.#.#.#.#.#########
#.#.............#...#...#.#.#...#...#...#.......#...#.......#...#...#...#...#...###...###.#...#.#.#.....#...#...#.....#...#...#.#.#...#...###
#.#.###########.#.#.#####.#.#####.#.#.#.#######.#.#############.#.###.#####.#########.###.###.#.#.#.###########.#####.#########.#.###.#.#.###
#.#.#...........#.#.#...#.#.#.....#...#...#...#.#...........###.#...#...#...#.......#...#.#...#.#.#...........#.#.....###.......#...#...#...#
#.#.#.###########.#.#.#.#.#.#.###########.#.#.#.###########.###.###.###.#.###.#####.###.#.#.###.#.###########.#.#.#######.#########.#######.#
#...#.....#...#...#...#.#...#.....#...###.#.#...#...###...#.#...#...#...#.###.#...#.....#...###.#.#...........#.#.#.....#.......#...#.......#
#########.#.#.#.#######.#########.#.#.###.#.#####.#.###.#.#.#.###.###.###.###.#.#.#############.#.#.###########.#.#.###.#######.#.###.#######
#...#...#...#...#.....#.#...#...#...#...#.#...#...#.....#...#...#...#...#.#...#.#.#...#.......#.#.#.#.......#...#...#...#.......#.#...#######
#.#.#.#.#########.###.#.#.#.#.#.#######.#.###.#.###############.###.###.#.#.###.#.#.#.#.#####.#.#.#.#.#####.#.#######.###.#######.#.#########
#.#...#...........###...#.#.#.#.###.....#...#.#.............###.....###...#.....#...#.#.#.....#...#...#.....#.....#...###...#.....#.........#
#.#######################.#.#.#.###.#######.#.#############.#########################.#.#.#############.#########.#.#######.#.#############.#
#.................#.......#...#...#...#...#...#.............#...#.............#...#...#.#.###...#.......###...###...#...###...###...#.......#
#################.#.#############.###.#.#.#####.#############.#.#.###########.#.#.#.###.#.###.#.#.#########.#.#######.#.#########.#.#.#######
#.....#...........#.............#.....#.#...###...............#.#.........#...#.#.#.....#.....#.#.#.........#.....###.#.#...#...#.#.#.......#
#.###.#.#######################.#######.###.###################.#########.#.###.#.#############.#.#.#############.###.#.#.#.#.#.#.#.#######.#
#...#.#.......#...###...#...###.....###.#...#...#...............###...#...#...#.#.#.............#...###.....#...#...#.#...#.#.#...#.........#
###.#.#######v#.#.###.#.#.#.#######.###.#.###.#.#.#################.#.#.#####.#.#.#.###################.###.#.#.###.#.#####.#.###############
#...#...#...#.>.#...#.#.#.#.###...#.#...#.....#.#.#.....#...#...#...#.#.....#.#.#.#.............#.......###...#.#...#.....#.#...............#
#.#####.#.#.#v#####.#.#.#.#.###.#.#v#.#########.#.#.###.#.#.#.#.#.###.#####.#.#.#.#############.#.#############.#.#######.#.###############.#
#.#...#...#...###...#.#.#.#.#...#.>.>.#.....#...#...#...#.#.#.#.#.#...#...#.#.#.#.###...........#.............#...#...#...#...#.........#...#
#.#.#.###########.###.#.#.#.#.#####v###.###.#.#######.###.#.#.#.#.#.###.#.#.#.#.#.###v#######################.#####.#.#.#####.#.#######.#.###
#.#.#...#...#...#...#.#.#.#.#.#.....###...#...###...#...#.#.#.#...#...#.#.#.#...#...>.>.###...###.........#...###...#.#.....#.#.......#...###
#.#.###.#.#.#.#.###.#.#.#.#.#.#.#########.#######.#.###.#.#.#.#######.#.#.#.#########v#.###.#.###.#######.#.#####.###.#####.#.#######v#######
#.#.###...#.#.#...#...#...#...#.......###...###...#.....#.#...#...#...#.#...###...#...#...#.#...#.......#...#...#...#.#...#.#.#...#.>.#...###
#.#.#######.#.###.###################.#####.###.#########.#####.#.#.###.#######.#.#.#####.#.###.#######.#####.#.###.#.#.#.#.#.#.#.#.#v#.#.###
#...#.......#.###...#...#...#...#...#.....#...#.......#...#...#.#.#.....#.......#...#...#.#.#...#.....#...#...#.#...#...#...#...#...#...#...#
#####.#######.#####.#.#.#.#.#.#.#.#.#####.###.#######v#.###.#.#.#.#######.###########.#.#.#.#.###.###.###.#.###.#.#########################.#
#...#.........#...#...#.#.#...#...#.....#.###...#...>.>.###.#...#.......#...........#.#.#.#.#...#.#...#...#...#.#.....#.....................#
#.#.###########.#.#####.#.#############.#.#####.#.###v#####.###########.###########.#.#.#.#.###.#.#.###v#####.#.#####.#.#####################
#.#.#...###.....#.......#.............#...#.....#...#.#...#.#...........#.........#...#.#...#...#.#.#.>.>...#.#...#...#.....................#
#.#.#.#.###.#########################.#####.#######.#.#.#.#.#.###########.#######.#####.#####.###.#.#.#v###.#.###.#.#######################.#
#.#...#...#...#.......#...#...........#...#.......#.#.#.#.#.#.....#...###.......#.#...#...###.....#...#.#...#...#.#...#...#...###.....#.....#
#.#######.###.#.#####.#.#.#.###########.#.#######.#.#.#.#.#.#####.#.#.#########.#.#.#.###.#############.#.#####.#.###.#.#.#.#.###.###.#.#####
#...#...#...#...#.....#.#.#.........###.#...#...#.#.#.#.#...#.....#.#...#.......#...#.#...#.............#.......#.#...#.#...#...#...#.#.....#
###.#.#.###.#####.#####.#.#########.###.###.#.#.#.#.#.#.#####.#####.###.#.###########.#.###.#####################.#.###.#######.###.#.#####.#
###...#.#...#...#.....#.#.........#.#...#...#.#.#...#...#...#.......#...#...#...#...#.#...#.....#...#...###...###...###.......#.....#.#...#.#
#######.#.###.#.#####.#.#########.#.#.###.###.#.#########.#.#########.#####.#.#.#.#.#.###.#####.#.#.#.#.###.#.###############.#######.#.#.#.#
#.......#.....#.......#.........#...#...#.....#...#...#...#...........#####...#...#.#.#...###...#.#...#.....#.....###.........#...###.#.#...#
#.#############################.#######.#########.#.#.#.###########################.#.#.#####.###.###############.###.#########.#.###.#.#####
#.#.....#.................#.....#...#...#...#.....#.#.#.................#...#...#...#...#...#.#...#.............#...#.#.........#...#.#.....#
#.#.###.#.###############.#.#####.#.#.###.#.#.#####.#.#################.#.#.#.#.#.#######.#.#.#.###.###########.###.#.#.###########.#.#####.#
#.#.#...#.#...........#...#...###.#.#.....#.#.#...#.#.#...#...#.........#.#.#.#.#...#...#.#.#...###.......#...#...#.#.#.#...........#...#...#
#.#.#.###.#.#########.#.#####.###.#.#######.#.#.#.#.#.#.#.#.#.#v#########.#.#.#.###.#.#.#.#.#############.#.#.###.#.#.#.#.#############.#.###
#...#...#.#.....#...#...#...#.#...#.#...#...#.#.#.#.#.#.#...#.>.>.....#...#.#.#.#...#.#...#.#...#...#...#...#...#...#...#.............#.#...#
#######.#.#####.#.#v#####.#.#v#.###.#.#.#.###.#.#.#.#.#.#######v#####.#.###.#.#.#.###.#####.#.#.#.#.#.#.#######.#####################.#.###.#
#.....#...#...#...#.>...#.#.>.>.###...#.#.#...#.#.#.#.#...#.....#...#.#.#...#.#.#...#.#.....#.#.#.#.#.#.#.....#...#...#...###.....#...#.....#
#.###.#####.#.#####v###.#.###v#########.#.#.###.#.#.#.###.#.#####.#.#.#.#.###.#.###v#.#.#####.#.#.#.#.#.#v###.###.#.#.#.#.###.###.#.#########
#...#.#...#.#.......###...#...###...#...#.#.#...#.#.#...#.#.....#.#.#.#.#...#.#.#.>.>.#...#...#...#.#.#.>.>.#.....#.#.#.#.#...#...#.........#
###.#.#.#.#.###############.#####.#.#.###.#.#.###.#.###.#.#####.#.#.#.#.###.#.#.#.#v#####.#.#######.#.###v#.#######.#.#.#.#.###.###########.#
#...#.#.#.#...............#.....#.#.#.....#.#.#...#...#.#.#...#...#.#.#.###.#.#...#.....#.#.###.....#.###.#.......#.#.#.#.#...#.........#...#
#.###.#.#.###############.#####.#.#.#######.#.#.#####.#.#.#.#.#####.#.#.###.#.#########.#.#.###.#####.###.#######.#.#.#.#.###v#########.#.###
#...#...#.....#...#.......#.....#.#.#...###...#...#...#.#.#.#.......#.#...#...###.......#.#...#.#...#.#...#.....#...#...#...>.#.......#.#...#
###.#########.#.#.#.#######.#####.#.#.#.#########.#.###.#.#.#########.###.#######.#######.###.#.#.#.#.#.###.###.#############v#.#####.#.###.#
###.........#.#.#...#.....#.......#...#.......###...###...#.#...#...#.#...###...#.......#.....#.#.#.#.#...#.#...###...#.......#.#.....#.....#
###########.#.#.#####.###.###################.#############.#.#.#.#.#.#.#####.#.#######.#######.#.#.#.###.#.#.#####.#.#.#######.#.###########
#...........#...#...#.#...#.............#.....#.............#.#...#.#...#.....#.........#...###...#.#.###...#.......#.#.#.......#...#...#...#
#.###############.#.#.#.###.###########.#.#####.#############.#####.#####.###############.#.#######.#.###############.#.#.#########.#.#.#.#.#
#...........#.....#...#...#...........#...#...#.....#.........#...#.....#.#...#...........#.....###...#...#.....#.....#...###.....#...#...#.#
###########.#.###########.###########.#####.#.#####.#.#########.#.#####.#.#.#.#.###############.#######.#.#.###.#.###########.###.#########.#
#...........#.#...........###...#...#.....#.#.#...#...#.........#.#...#.#.#.#...#.........#.....#.......#.#...#...#...#...#...###...#...#...#
#.###########.#.#############.#.#.#.#####.#.#.#.#.#####.#########.#.#.#.#.#.#####.#######.#.#####.#######.###.#####.#.#.#.#.#######.#.#.#.###
#.............#.....###.......#...#.......#.#.#.#.#...#.........#...#...#...#...#.......#.#.#...#.....#...###.#...#.#.#.#.#.....###...#.#.###
###################.###.###################.#.#.#.#.#.#########.#############.#.#######.#.#.#.#.#####.#.#####.#.#.#.#.#.#.#####.#######.#.###
###...#...#...#...#...#.......#.....#...###.#.#.#.#.#.....#...#.........###...#...#.....#...#.#...###.#.#...#.#.#.#.#.#.#.#...#...#.....#...#
###.#.#.#.#.#.#.#.###.#######.#.###.#.#.###.#.#.#.#.#####.#.#.#########.###.#####.#.#########.###.###.#.#.#.#v#.#.#.#.#.#.#.#.###v#.#######.#
#...#...#...#...#.....#...###.#...#...#.....#.#.#.#.....#...#.........#.#...#...#...###.....#...#.#...#.#.#.>.>.#.#.#.#.#.#.#.#.>.#.......#.#
#.#####################.#.###v###.###########.#.#.#####.#############.#.#.###.#.#######.###.###.#.#.###.#.###v###.#.#.#.#.#.#.#.#v#######.#.#
#.#.....#...#.........#.#.#.>.>...#.......#...#.#.#...#...#.....#...#...#.....#.......#.#...#...#...###...###...#.#.#...#...#...#.#...###...#
#.#.###.#.#.#.#######.#.#.#.#v#####.#####.#.###.#.#.#.###.#.###.#.#.#################.#.#.###.#################.#.#.#############.#.#.#######
#.#.###.#.#.#.#.......#.#.#.#...#...#####...###.#.#.#.###...#...#.#.#.....#...........#.#.###.......#...........#.#.....###.....#...#.#.....#
#.#.###.#.#.#.#.#######.#.#.###.#.#############.#.#.#.#######.###.#.#.###.#.###########.#.#########.#.###########.#####.###.###.#####.#.###.#
#.#.#...#.#.#.#.#.....#.#...###.#.....#...#...#.#...#...#...#.....#...#...#.............#.###...#...#...........#.#...#.#...#...#...#...#...#
#.#.#.###.#.#.#.#.###.#.#######.#####.#.#.#.#.#.#######.#.#.###########.#################.###.#.#.#############.#.#.#.#.#.###.###.#.#####.###
#...#.....#...#...#...#.....#...#...#.#.#.#.#.#.#.......#.#.###.......#.#...#.....#.......#...#.#.#.........#...#.#.#.#.#...#.#...#...#...###
###################v#######.#.###.#.#.#.#.#.#.#.#.#######.#.###v#####.#.#.#.#.###.#.#######.###.#.#.#######.#.###.#.#.#.###.#.#.#####.#.#####
#...#...#...#.....#.>.#.....#.#...#.#...#...#.#.#.#.....#.#.#.>.>...#...#.#.#.#...#.###...#.###...#.#.....#...###.#.#.#.#...#.#...#...#.....#
#.#.#.#.#.#.#.###.#v#.#.#####.#.###.#########.#.#.#.###.#.#.#.#v###.#####.#.#.#.###v###.#.#.#######.#.###.#######.#.#.#.#.###.###.#.#######.#
#.#.#.#.#.#.#...#.#.#...#...#.#.#...#.......#.#.#.#.#...#.#...#...#...#...#.#.#...>.>.#.#.#.......#...###.......#...#...#...#.....#.......#.#
#.#.#.#.#.#.###.#.#.#####.#.#.#.#.###.#####.#.#.#.#.#.###.#######.###.#.###.#.#####v#.#.#.#######.#############.###########.#############.#.#
#.#...#...#.....#.#.....#.#...#.#...#.#.....#...#.#.#.....#.......###...###.#.###...#...#.#.......###...........###.......#...#.........#.#.#
#.###############.#####.#.#####.###.#.#.#########.#.#######.###############.#.###.#######.#.#########.#############.#####.###.#.#######.#.#.#
#.......#.......#.......#.....#.###...#.........#...#.....#...............#.#.#...#...###...###.....#.....#...#...#.....#.....#.#.......#...#
#######.#.#####.#############.#.###############.#####.###.###############.#.#.#.###.#.#########.###.#####.#.#.#.#.#####.#######.#.###########
#...###...#...#.#...........#...#...#...........#.....#...#.......#.......#...#.....#.....#...#...#.#.....#.#.#.#...###.....#...#.#...#.....#
#.#.#######.#.#.#.#########.#####.#.#.###########.#####.###.#####.#.#####################.#.#.###.#.#.#####.#.#.###.#######.#.###.#.#.#.###.#
#.#.#...#...#.#...#.....#...#.....#...#.........#.....#...#.....#...#.....###...#...#...#...#.#...#...#.....#.#...#.....#...#.#...#.#.#.#...#
#.#.#.#.#.###.#####.###.#.###.#########.#######.#####.###.#####.#####.###.###.#.#.#.#.#.#####.#.#######.#####.###.#####.#.###.#.###.#.#.#.###
#.#.#.#.#...#.#.....###...###.........#.#.......#...#...#.#...#.#...#.#...#...#...#...#...#...#.........#...#.#...#.....#.....#.....#...#...#
#.#.#.#.###.#.#.#####################.#.#.#######.#.###.#.#.#.#.#.#.#.#.###.#############.#.#############.#.#.#.###.#######################.#
#.#...#.....#...#.......#...#...#...#...#.......#.#.....#.#.#.#.#.#.#.#...#.........#...#...#.....#...###.#...#...#.#...###.................#
#.###############.#####.#.#.#.#.#.#.###########.#.#######.#.#.#v#.#.#.###.#########.#.#.#####.###.#.#.###v#######.#.#.#.###v#################
#.#...#...........#.....#.#.#.#.#.#.###.........#.......#.#.#.>.>.#.#.#...###...#...#.#.#...#...#.#.#.#.>.>.#...#.#.#.#.#.>.#...#...#...#...#
#.#.#.#.###########.#####.#.#.#.#.#.###.###############.#.#.#######.#.#.#####.#.#v###.#.#.#.###.#.#.#.#.###.#.#.#.#.#.#.#.#v#.#.#.#.#.#.#.#.#
#.#.#.#.#...........###...#.#.#.#.#...#...........#.....#.#.....#...#.#...#...#.>.>.#.#.#.#...#.#.#.#.#.#...#.#.#.#.#.#.#.#.#.#...#...#.#.#.#
#.#.#.#.#.#############.###.#.#.#.###.###########.#.#####.#####.#.###.###.#.#######.#.#.#.###.#.#.#.#.#.#.###.#.#.#.#.#.#.#.#.#########.#.#.#
#.#.#.#.#.#...#.........#...#.#...#...#...........#.....#.#...#.#.....#...#...###...#.#.#.###.#.#...#.#.#...#.#.#.#.#.#.#.#.#.#.........#.#.#
#.#.#.#.#.#.#.#.#########.###.#####.###.###############.#.#.#.#.#######.#####.###.###.#.#.###.#.#####.#.###.#.#.#.#.#.#.#.#.#.#.#########.#.#
#.#.#.#.#.#.#...#.......#...#.....#...#.....#...#####...#.#.#.#.#.......#...#...#...#.#.#...#.#...#...#.#...#.#.#.#...#.#.#.#.#.....#...#.#.#
#.#.#.#.#.#.#####.#####.###.#####.###.#####.#.#.#####.###.#.#.#.#.#######.#.###.###.#.#.###.#.###.#.###.#.###.#.#.#####.#.#.#.#####.#.#.#.#.#
#...#...#...#.....#...#.....#...#.#...###...#.#...#...###.#.#.#.#.#.....#.#.#...###.#.#...#.#...#.#.#...#.#...#...#...#.#.#.#.#.....#.#...#.#
#############.#####.#.#######.#.#.#.#####v###.###.#.#####.#.#.#.#.#.###.#.#.#.#####.#.###.#.###.#.#.#.###.#.#######.#.#.#.#.#.#.#####.#####.#
#.............#...#.#.#.....#.#.#.#...#.>.>.#...#.#.....#.#.#.#.#.#.#...#.#.#.....#.#.#...#...#.#.#.#...#.#...#.....#...#.#.#.#...#...#.....#
#.#############.#.#.#.#.###.#.#.#.###.#.###.###.#.#####.#.#.#.#.#.#.#.###.#.#####.#.#.#.#####.#.#.#.###.#.###.#.#########.#.#.###.#.###.#####
#...............#...#...###...#...###...###.....#.......#...#...#...#.....#.......#...#.......#...#.....#.....#...........#...###...###.....#
###########################################################################################################################################.#".to_string()
}
