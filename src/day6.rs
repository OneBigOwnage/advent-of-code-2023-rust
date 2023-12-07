use regex::Regex;

#[derive(Debug)]
struct RaceRecord {
    record_time: i64,
    record_distance: i64,
}

impl RaceRecord {
    fn margin_of_error(&self) -> i64 {
        let mut ways: i64 = 0;

        for ms_hold_down in 0..self.record_time {
            if (self.record_time - ms_hold_down) * ms_hold_down > self.record_distance {
                ways += 1;
            }
        }

        ways
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let race_records = parse_records(input());

    let margins: Vec<i64> = race_records
        .iter()
        .map(|race| race.margin_of_error())
        .collect();

    println!("Ways to win each race: {margins:?}");
    println!(
        "Multiplied together, yields: {}",
        margins.iter().fold(1, |acc, margin| acc * margin)
    );
}

fn part2() {
    let race_record = parse_single_record(input());

    let margin = race_record.margin_of_error();

    println!("Ways to beat this record: {margin}");
}

fn parse_records(input: &'static str) -> Vec<RaceRecord> {
    let times: Vec<i64> = Regex::new(r"Time:\s+(\d+\s*)+")
        .unwrap()
        .captures_iter(input)
        .flat_map(|capt| {
            let (m, [_]) = capt.extract::<1>();

            m.split_whitespace()
                .skip(1)
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let distances: Vec<i64> = Regex::new(r"Distance:\s+(\d+\s*)+")
        .unwrap()
        .captures_iter(input)
        .flat_map(|capt| {
            let (m, [_]) = capt.extract::<1>();

            m.split_whitespace()
                .skip(1)
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(
            |(&record_time, &record_distance): (&i64, &i64)| RaceRecord {
                record_time,
                record_distance,
            },
        )
        .collect()
}

fn parse_single_record(input: &'static str) -> RaceRecord {
    let (_, [time]) = Regex::new(r"Time:\s+([\d+\s*]+)")
        .unwrap()
        .captures(input)
        .unwrap()
        .extract();

    let mut time = String::from(time);
    time.retain(|c| !c.is_whitespace());

    let (_, [distance]) = Regex::new(r"Distance:\s+([\d+\s*]+)")
        .unwrap()
        .captures(input)
        .unwrap()
        .extract();

    let mut distance = String::from(distance);
    distance.retain(|c| !c.is_whitespace());

    println!("Time: {time}, distance: {distance}");

    let record_time = time.parse().unwrap();
    let record_distance = distance.parse().unwrap();

    RaceRecord {
        record_time,
        record_distance,
    }
}

#[allow(dead_code)]
fn test_input() -> &'static str {
    "Time:      7  15   30
    Distance:  9  40  200"
}

fn input() -> &'static str {
    "Time:        49     78     79     80
    Distance:   298   1185   1066   1181"
}
