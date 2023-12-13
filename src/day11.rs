use std::collections::{HashMap, HashSet};

type Point = (usize, usize);
type Route = Vec<Point>;

#[derive(Debug)]
struct GalaxyPair {
    origin: Point,
    destination: Point,
    route: Option<Route>,
    length: Option<usize>,
}

fn main() {
    // assert_eq!(9605127, part1());
    assert_eq!(8410, part2());
}

fn part1() -> usize {
    let universe = test_input();
    let galaxies = find_galaxies(&universe);
    let galaxy_pairs = make_galaxy_pairs(&galaxies);
    let expansion_rate = 2;

    println!(
        "We found {} galaxies, which makes for {} unique pairs.",
        galaxies.len(),
        galaxy_pairs.len()
    );

    let expansion_points = get_expansion_points(&universe);

    let pairs_with_shortest_paths: Vec<GalaxyPair> = galaxy_pairs
        .iter()
        .take(100)
        .map(|&(origin, destination)| {
            let (route, length) = find_shortest_path_dijkstras(
                &universe,
                (origin, destination),
                expansion_rate,
                &expansion_points,
            );

            GalaxyPair {
                origin: *origin,
                destination: *destination,
                route: Some(route),
                length: Some(length),
            }
        })
        .collect();

    pairs_with_shortest_paths.iter().fold(0, |acc, path| {
        acc + path.length.expect("We should have set all lengths")
    })
}

fn get_expansion_points(universe: &String) -> Vec<Point> {
    let mut expansion_points = vec![];

    for (y, line) in universe.lines().enumerate() {
        if !line.chars().any(|ch| ch == '#') {
            expansion_points.extend((0..line.len()).map(|x| (x, y)));
        }
    }

    for x in (0..universe.lines().nth(0).unwrap().len())
        .into_iter()
        .filter(|i| {
            universe
                .lines()
                .all(|line| line.chars().nth(*i).unwrap() != '#')
        })
    {
        expansion_points.extend((0..universe.lines().count()).map(|y| (x, y)));
    }

    expansion_points
}

fn find_galaxies(universe: &String) -> Vec<Point> {
    let mut galaxies = vec![];

    for (y, line) in universe.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

fn make_galaxy_pairs(galaxy_coordinates: &Vec<Point>) -> Vec<(&Point, &Point)> {
    let mut pairs = vec![];

    for i in 0..galaxy_coordinates.len() {
        for ii in (i + 1)..galaxy_coordinates.len() {
            pairs.push((&galaxy_coordinates[i], &galaxy_coordinates[ii]));
        }
    }

    pairs
}

fn find_shortest_path_dijkstras(
    universe: &String,
    (origin, destination): (&Point, &Point),
    expansion_rate: usize,
    expansion_points: &Vec<Point>,
) -> (Route, usize) {
    let mut nodes_discovered: HashMap<Point, (Vec<Point>, usize)> = HashMap::new();
    nodes_discovered.insert(*origin, (vec![*origin], 0));

    let max_x = universe
        .lines()
        .nth(0)
        .expect("The universe should have a first line")
        .chars()
        .count()
        - 1;
    let max_y = universe.lines().count() - 1;

    let mut edge_nodes: HashSet<Point> = HashSet::new();

    edge_nodes.insert(*origin);

    loop {
        let mut next_edge_nodes: HashSet<Point> = HashSet::new();

        for edge_node in &edge_nodes {
            for (x, y) in get_neighbors_bounded(&edge_node, &(0, 0), &(max_x, max_y)) {
                let new_dest = (x, y);

                let (mut new_path, mut new_length) =
                    nodes_discovered.get(&edge_node).unwrap().clone();

                new_path.push(new_dest);
                new_length += match expansion_points.contains(&new_dest) {
                    true => expansion_rate,
                    false => 1,
                };

                next_edge_nodes.insert(new_dest);

                if let Some(node) = nodes_discovered.get(&(x, y)) {
                    if new_length < node.1 {
                        nodes_discovered.insert(new_dest, (new_path, new_length));
                    }
                } else {
                    nodes_discovered.insert(new_dest, (new_path, new_length));
                }

                if new_dest == *destination {
                    return nodes_discovered.get(&new_dest).unwrap().clone();
                }
            }
        }

        edge_nodes = next_edge_nodes;
    }
}

fn get_neighbors_bounded(
    (x, y): &Point,
    (min_x, min_y): &Point,
    (max_x, max_y): &Point,
) -> Vec<Point> {
    let mut neighbors = vec![];

    if *x > *min_x {
        neighbors.push((*x - 1, *y));
    }

    if *x < *max_x {
        neighbors.push((*x + 1, *y));
    }

    if *y > *min_y {
        neighbors.push((*x, *y - 1));
    }

    if *y < *max_y {
        neighbors.push((*x, *y + 1));
    }

    neighbors
}

#[allow(dead_code)]
fn visualize_path_in_universe(universe: &String, path: &Route) {
    let mut output: String = "".to_owned();

    for (y, line) in universe.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if path.first() == Some(&(x, y)) {
                output += "1";
            } else if path.last() == Some(&(x, y)) {
                output += "2";
            } else if path.contains(&(x, y)) {
                output += "+";
            } else {
                output += &ch.to_string();
            }
        }
        output += "\n";
    }

    println!("{output}\n----------------------------");
}

fn part2() -> usize {
    let universe = input();
    let galaxies = find_galaxies(&universe);
    let galaxy_pairs = make_galaxy_pairs(&galaxies);
    let expansion_rate = 1_000_000;

    println!(
        "We found {} galaxies, which makes for {} unique pairs.",
        galaxies.len(),
        galaxy_pairs.len()
    );

    let expansion_points = get_expansion_points(&universe);

    let pairs_with_shortest_paths: Vec<GalaxyPair> = galaxy_pairs
        .iter()
        .take(1000)
        .map(|&(origin, destination)| {
            let (route, length) = find_shortest_path_dijkstras(
                &universe,
                (origin, destination),
                expansion_rate,
                &expansion_points,
            );

            GalaxyPair {
                origin: *origin,
                destination: *destination,
                route: Some(route),
                length: Some(length),
            }
        })
        .collect();

    for path in &pairs_with_shortest_paths {
        println!(
            "Path: from ({}, {}) to ({}, {}). Length: {}. Route: {:?}",
            path.origin.0,
            path.origin.1,
            path.destination.0,
            path.destination.1,
            path.length.unwrap_or(99),
            path.route.clone()
        );
    }

    pairs_with_shortest_paths.iter().fold(0, |acc, path| {
        acc + path.length.expect("We should have set all lengths")
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MinDistanceNode {
    point: Point,
    distance: usize,
}

#[allow(dead_code)]
fn test_input() -> String {
    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "...........................#.............#..........#.........#..................#.......#.............................................#....
..............#...........................................................................................#...............#.................
.....................#......................................................................................................................
...................................#................................#................................................#....................#.
.................#.......................................................#..........#..................#.............................#......
...............................................................................................#.................#..........................
......................................................#........................#........................................#...................
........#.....................................................#...........................#.................#...............................
.........................................#.....#...............................................................................#............
...........................#.........................................................#...................................................#..
.....#................................................................#............................................................#........
.............#..........................................#.......................................#.................#.........#...............
............................................#...............................................................................................
.#...............#.....................#....................................#.............................#................................#
............................#....................#..........................................................................................
...................................................................................................#..........#.............................
.......#............#.........................................#........#.........................................................#..........
..............#.................................................................#..........#............................#...................
................................#........................................................................................................#..
...............................................#.....................................................#...........#..........................
#...............................................................#...........................................................................
............#......#...................................#....................................................#.............#.........#.......
.......................................................................................#....................................................
....................................#........................#......#...........................#...........................................
.........#.....#.............#............#.....................................#.....................................#.....................
..........................................................................................................#.....#...............#..........#
........................................................#...................................................................................
..........................#.................................................................................................................
....#..................................................................#..................................................#.................
.....................#..........................................#...........................#.........#.....................................
...........#.........................#.......................................#......#.................................................#.....
#...............................................................................................#..................#............#...........
............................#.............#.........#.......................................................................................
........#...........................................................#.......................................................................
.........................................................................#..........................#.......................................
..#..........#............................................#.....#.....................#...............................#.....................
.....................................#...............................................................................................#......
....................#......................#....................................................................................#...........
.....#......................................................................................................................................
................................#....................#............................#.........................................................
........................#......................................................................#.............................#..............
.......................................................................#....................................................................
..............#.................................#......................................................#.......#..................#.......#.
............................................................#............................#..................................................
..........................................................................................................................#.................
..#.................#.........#............................................#.......#........................................................
.......#.............................................#........................................#.......................#.....................
...............................................................#.................................................................#..........
...........................#...............#.......................................................#.....................................#..
.................................#..............#...............................................................#...........................
...............#........................................................................#...................................................
.........................................................#......................................#..........#................................
..#......................................#................................#.................................................................
...............................#............................................................#..............................................#
..........#........#............................................#...............................................................#...........
................................................................................#..................#...........#..........#.................
..................................#........#................................................................................................
............................................................................................................................................
.....................#............................................#............................#.......#.....................#..............
............................#........................#.....................#..........#..........................#.....................#....
.........................................................................................................................#..................
.#.........#...........................#.......................#...................................#........................................
................#........#.....................................................................................................#..........#.
...........................................................................................#.................#..............................
...............................#......................................#............................................#........................
.........................................#.......#......................................................#...................................
..........#..................................................#.................#............................................................
#.......................#..............................................................#...........................................#.......#
.................................#......................#........................................................#........#.................
............................................................................................................................................
...............................................#...............................................#............................................
................#.....................#.......................................#.............................................................
......................................................#...............#.....................................................................
...................................................................................#.............................................#..........
..................................................................#........#....................................#...........................
.#.......................................................#.................................#............#...................................
............................................................................................................................................
........#.......................#................................................#.........................................#................
..................#......#......................................................................#...........................................
......................................#...............#..........#.....................................................................#....
...#....................................................................................#...................................................
...............#..............................#.............................#...............................................................
...................................#..............................................................#.....#........................#.........#
.......#............#...................................................................................................#...................
...........................#.................................................................#................#.............................
.....................................................#...............................................#......................................
...........#..............................................#.................................................................................
..............................................#.................#...............#.........#.................................................
................#.......#...........#.................................#...................................#.................................
.....................................................................................#...........#...................#......................
........................................................#......................................................................#.......#....
.....#......................#...............................................................................................................
.............#........#..........................#..........................................................#.............#................#
............................................................................................................................................
........#............................#............................................#..........#..............................................
............................................#...................................................................#...........................
............................................................................................................................................
..#..............#......#.........................................#...............................#.........................................
....................................................#......................#.......................................................#......#.
...................................#........................#................................................................#..............
.....................................................................#...........#..........................................................
...............#.............................#...........................................................#..................................
..........#..............................................................................#.....#.................#..........................
.....................................................................................................................................#......
.#................................#.........................................................................#...............................
........................#.........................................#......#.........#................#.......................................
.................#........................................#..................................................................#..............
........................................#...................................................................................................
....#.....#........................................#............................................#...........................................
............................................................................................................................................
..............................................................................#.....................................#................#......
.....................#................................................................#.........................................#...........
.............................#.........#........#.........#.............#...........................#.......................................
................................................................#.........................#..................#............#.............#...
........#.......................................................................#...........................................................
........................#..................#............................................................#...................................
.....................................#.................................................#.........#................#.........................
...................................................#.....................#....................................................#............#
............................................................................................................................................
................#..............................................#.................#............................#.............................
....#........................#..........................#............#......................................................................
....................................#........................................................#..............................................
.......................#..........................................................................#.....#...........#............#..........
........#..................................#................#.............#...........................................................#.....
................................................#.......................................................................#..................#
..................#.................................................................#.......................................................
............#........................#...........................................................................#.............#............
...#........................................................................................................................................
..........................................#.................................#................#..............................................
.......................#............................#.............#...................................................#.............#.......
.................................#....................................................#........................#............................
.................................................................................#........................................................#.
..........................................................#.............................................#...................................
...#............#...........................................................................................................................
..............................................#...................................................#.........................................
.........................................................................#.....#.......#.........................................#..........
............#.........#......................................................................#........#.....................................
...........................#........................................#........................................#.......................#......
.....#.................................#..............#........#.....................................................#......................
...............................#............................................#........#....................................#.................".to_string()
}