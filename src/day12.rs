use std::collections::HashMap;

#[derive(Debug)]
struct ConditionRecord {
    springs: String,
    damaged_spring_groups: Vec<usize>,
    arrangements: Option<Vec<String>>,
}

fn main() {
    assert_eq!(21, part1(&test_input()));
    assert_eq!(7622, part1(&input()));
    // assert_eq!(0, part2(&test_input()));
    // assert_eq!(0, part2(&input()));
}

fn part1(input: &String) -> usize {
    let mut records = parse_records(input);

    records
        .iter_mut()
        .map(|record| {
            println!(
                "Going to find arrangements for ({}\t{:?})",
                record.springs, record.damaged_spring_groups
            );
            set_arrangements(record);
            println!(
                "\t-> we found {} possible arrangements for ({}\t{:?})",
                record.arrangements.clone().expect("We just set them").len(),
                record.springs,
                record.damaged_spring_groups,
            );

            record.arrangements.clone().unwrap().len()
        })
        .sum()
}

fn part2(input: &String) -> usize {
    todo!();
}

fn parse_records(input: &str) -> Vec<ConditionRecord> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            ConditionRecord {
                springs: split.next().unwrap().to_string(),
                damaged_spring_groups: split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect(),
                arrangements: None,
            }
        })
        .collect()
}

fn set_arrangements(record: &mut ConditionRecord) -> &mut ConditionRecord {
    let swappables: Vec<usize> = record
        .springs
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '?')
        .map(|(i, _)| i)
        .collect();

    let string = fill_out_potential_spring_arrangement(
        &record.springs,
        record.damaged_spring_groups.iter().sum::<usize>()
            - record.springs.chars().filter(|ch| *ch == '#').count(),
    );

    let mut memo: HashMap<(String, usize), Vec<String>> = HashMap::new();
    record.arrangements = Some(
        recursively_find_arrangements(&string, &swappables, 0, &mut memo)
            .into_iter()
            .filter(|a| is_arrangement_possible(record, a))
            .collect(),
    );

    record
}

fn fill_out_potential_spring_arrangement(springs: &str, broken_count: usize) -> String {
    let mut parts = springs.chars().collect::<Vec<_>>();
    let mut todo_broken = broken_count;

    for i in 0..springs.len() {
        if parts[i] == '?' {
            if todo_broken > 0 {
                parts[i] = '#';
                todo_broken -= 1;
            } else {
                parts[i] = '.';
            }
        }
    }

    parts.iter().collect()
}

fn recursively_find_arrangements(
    springs: &str,
    swappables: &Vec<usize>,
    index: usize,
    memo: &mut HashMap<(String, usize), Vec<String>>,
) -> Vec<String> {
    let mut chars: Vec<_> = springs.chars().collect();

    let mut permutations = vec![];

    if index >= swappables.len() {
        permutations.push(chars.iter().collect());
    } else {
        for i in index..swappables.len() {
            if i != index && chars[swappables[index]] == chars[swappables[i]] {
                // Skip swapping the same chars as that results in an identical string.
                continue;
            }

            let mut cloned_chars = chars.clone();

            cloned_chars.swap(swappables[index], swappables[i]);

            if let Some(memoized_result) =
                memo.get(&(cloned_chars.iter().collect::<String>(), index + 1))
            {
                permutations.extend(memoized_result.to_vec());
                continue;
            }

            let result = recursively_find_arrangements(
                &cloned_chars.iter().collect::<String>(),
                swappables,
                index + 1,
                memo,
            );

            memo.insert(
                (cloned_chars.iter().collect::<String>(), index + 1),
                result.clone(),
            );
            permutations.extend(result);

            cloned_chars.swap(swappables[index], swappables[i]);
        }
    }

    permutations.sort();
    permutations.dedup();
    permutations
}

fn is_arrangement_possible(record: &ConditionRecord, arrangement: &str) -> bool {
    arrangement
        .split(".")
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect::<Vec<_>>()
        == record.damaged_spring_groups
}

#[allow(dead_code)]
fn test_input() -> String {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "????#?#???.??.. 9,2
?.#?????????###.?# 1,1,2,1,5,1
.???#????#?????#?#? 1,9,4
?#?.??.#?.??? 2,1,1,1
?????????#?###???.?. 1,9
????#?.?.?? 1,2,1
.???????#???..? 1,5
#????????#?#??#??. 2,14
?.?????.?? 2,2
???##??###???????#?? 9,4
.#.?.??..?# 1,1,2
.?????#??? 2,1,2
??.?..?#?##?????# 1,1,11
?.???.??.??#?. 1,2,2,4
...#??..????##??. 1,6
?????#.?##?????.??? 5,2,1,1,1
?##?#?.???#?.?? 5,4,1
.?.##???????#????.? 3,5
.?#???#?#..???.?? 6,1,1,1
.???##.???# 5,2,1
?#?#?#?.?#?.?? 5,3
.?#?.??.?# 1,1,1
?#???#?.???????.#. 2,2,6,1
##???.??.?.?? 4,1
.???#??.?#.. 4,2
??#??..#?..## 5,1,2
??##??#?????.??? 2,3,1,1
.????.?##????#?? 2,6,2
.?##????.?##???#?. 3,3,4,2
?#??????????#????## 2,1,1,3,2,4
?????.???????????? 4,1,1,6
???##?##????? 6,1
?.?..??#??#???.. 1,7
?#?????##??##????#.? 3,1,7,1,1,1
#???#???##????#??... 1,14
?????##??.?????. 1,6,2,1
???#????????#? 4,8
##???#..??##??#??.#? 3,2,1,6,1
???????????.#?? 4,3,2
?#???#?????? 1,6,1
?.?.??###...???#?.?? 5,5
..??????#???? 1,1
?????????????? 2,4,1,1
?#.?.?#?#?. 1,4
#?.?#????????#???? 2,3,8
?????#?#.????? 6,1,1,1
????#.??..? 4,2,1
?????????#?.? 2,2,3
???#???..?# 3,2,2
.#??.?.?.?##??##?#. 1,1,1,9
??#?#???#???##? 8,5
#??????###??????.??? 1,2,10,3
#?#?#??????#??? 1,1,5,4
#???.???#?????#?? 1,1,1,5,1
#?.?##??.?? 1,3,1
????????##??? 1,1,4
??..??.??##???????. 1,10
??.?????#?#??##???? 1,1,1,4,6
??#.?????#?#?? 1,1,3,5
?????##????# 1,7
??.?.??#????? 1,1,1,2
..?.???.??????? 1,1,1,2,1
????????#?#???#??? 2,1,8
?????#?##????????.?. 2,5,4,1
#?????..?.??##?# 1,1,1,5
.#??.????? 2,1,1
??#??##??.?#?. 8,1
.?????.#?. 4,1
.?#??#.????. 3,1,1,1
????.???..?#??. 1,1,1,4
?##??.??..#?#? 3,2,3
????#??.????.. 1,3,1
?.?##???##?.???.??? 1,8,3,3
??#??.#?#??? 3,3,1
.?????.?#???????? 1,3,6,2
??#??????????????#? 1,1,1,4,4,1
?.?.??#..???#?. 1,1,1,1,3
??#??....###??? 2,3
?????#?.###???.#?. 3,3,3,1
#..???.#???????? 1,1,2,1,2
???#???.#??# 4,1,2
?.??????????. 4,2
.#??..????? 1,5
?#?#?#???..??#??# 5,1,6
???#?????? 4,1
??#??##??.????? 7,2
..#????..#?#?.? 5,3
?..?.??#???#?? 1,1,2,5
#???##.??.?#?#? 2,3,1,5
????????.?#?? 1,3,1,2
#???..?#??? 1,1,3
#?#???.#..????#.#?? 1,4,1,3,1,3
#??#.???.#?#???. 1,1,1,1,6
???##????????#???.#? 9,1,5,1
?.#????#??.?#?.? 1,1,3,1,1
##?????.?.. 2,1,1
?#???#.#?#.??.??. 3,1,3,1,1
.??#???.?.????.#??#? 6,4,4
?.???####??.#???.??? 6,4,1
#.???#???#??? 1,4,1
???#???#?#??? 1,4,5
???#??.####???##?? 1,2,6,2,1
#?#.?????.??# 3,5,2
.#??.??#?? 1,3
??#?##???#??#..???? 12,1
???.??.???????#???# 1,2,12
??????#??.?#?##?? 4,6
.????.????## 4,2,2
.#????????????.##? 1,2,2,3
???.???#???..????#? 1,5,1,1,1,1
???????#?. 3,4
.???###?#????..? 1,9
.#??#????#?.??#?... 6,1,4
#????#??????.?#?? 1,1,8,1,1
?.????.??#.? 3,1
.????.??.??? 1,1,2,1
???????.?.?.??.?? 3,1,1
#????#????#.??? 1,8,1,1
.???????.?#?#??# 2,4,2,4
#??#???.#??.????##? 2,1,1,2,1,5
?#?#?.????????? 4,1,1,3
..?.?####??.?? 1,7,1
??###????#??? 4,5
...?#?????.???.??.. 2,1
.???????.????##.??# 4,1,1,2,2
.???.?.?#?? 1,1,2
#???...#?#??#?. 2,1,7
??###?#????????. 8,1,2
.??.?#?????#??? 1,9
??.?????#?#?????? 1,7
.????.#??? 4,1,2
?#?.?#?.??# 3,2,1
?.?#.???#?# 2,3
????#.????. 4,1
???????????#?????.? 1,1,8,1
.???##???#?? 1,3,1,1
????.????.#??? 1,3,4
????????.#??#..#?## 4,1,2,1,1,2
..??.?.#???##?.??.. 1,6
??.#????????#?## 4,3,4
??#??????#.##?# 5,2,4
???.#???.? 1,1,1
?#?????.?.?? 1,2,1,1
#????.?.#?? 1,1,2
?#??#????#??#?? 1,2,9
?#????#????.????.?? 10,1,1
????.????.?? 2,1,1,1
?#??#???.???.#?##? 7,3,1,2
??.??#???...????.? 5,1
???????.?? 1,2,1
.#.?####.. 1,4
?.#??.??#?##???. 1,2,5
??.?#??????? 3,2
??#?????????.?? 3,1,1,1,1
?????????? 2,1,1
???????????..? 3,2
????##???? 6,1
?#????##?# 3,4
????????.?#? 4,2,1
####..#?..?#?.? 4,2,1
?#?..?#?.?#.??? 2,1,2,1
???????..?#? 3,1,1,1
?#???????. 1,1,1
#.????????#???.? 1,1,1,7
?#?#??#??? 1,4,2
..??.?..?.??. 1,1
..?#???.???..? 4,3
?#?##.?.#? 4,1
.??#?#.?.??.???? 5,1,1,1
?#??#???????. 5,3
#?#?#?#??????. 1,7,1,1
????????.?. 2,2
???.??.????????#. 2,7
?.#..??????#????.? 1,10
??#?##??.???#?? 3,3,5
???????.????#?.. 2,3,5
..???????????? 1,3,1
?..??#??.?#????? 1,1,1,1,5
????#?..?? 5,1
???#.??#??.????? 4,3,3,1
?#?#??.??.?#?? 3,1,1,3
??.???##??#? 1,1,5
?#.?.???.????? 1,1,3,2
#???..??#???#?????? 1,1,1,3,1,3
??????##????.?..# 8,1,1,1
#????##??#? 2,5,1
#??.???#?..? 2,1,3
???.?.????????#.???? 1,1,8,3
??.#.#?.#???##????? 1,1,2,1,6
.??????????.???# 3,3,3
???#?#????? 4,3
?.????????#????.? 1,2,7
???##?#??.???? 6,1
??#???.?.#??##? 2,1,5
??#?.??###? 1,4
??###????.??#? 7,1,2
????.#????##??? 1,2,6
?#???#???# 7,1
??????#??#..?? 10,1
..##????.?? 2,2
.???.????#??.??????? 1,2,2,1,3
????#?#??#??.. 5,2
?#??.?.?..#??? 4,1,2
??#???.#?????# 3,2,4,1
???##??##.??#?..?#.? 1,6,2,1,1
#?#?#???????#???? 6,3
??????.?#?? 2,1,3
???#???#????#?..?? 2,8,1
????#?#???.??.?. 1,7,1,1
?????.?##?.? 3,3
?????.#?????? 5,1,2,1
?#...???#??#?????.?. 1,4,7
???#????#?? 2,2
.#?#???.???# 1,1,1,1
?#??#??.#?.?.?#??? 7,1,1,2,1
??.#.?????? 1,1
#?#?????#?#?? 5,1,1
??#?##????.???????? 1,4,1,1,2,1
?#????.?.#? 1,2,1
##??????#???#??#? 13,2
.???###?.????##? 5,4
????#?.?..???????? 3,2,1,1,1,1
?.?#??????????.? 3,1,3,1,1
.#?????#.#????? 1,1,2,5
???????#.. 1,1,2
??##??#??#?.#??#... 5,1,1,1,2
..#??##???##????##?# 16,1
??????????? 1,3
??#?#?.????####??.? 3,8
?????.???#?..#?## 2,1,4,4
??.???????##?#??#??? 1,1,3,10
????????#????#??.?# 2,10,2
?#?#??????????????#? 11,4
.#?.#???##. 2,6
??#..#??.?..# 3,3,1,1
???????##?.??????.?? 10,1,1,1,1
????#????????????#.? 3,12
???.?#.?#?. 2,1
.#?.???#??? 1,1,5
????#.#???#? 1,2,2,1
.?.?.???#????#? 1,5,2
???.?#???? 1,3
??.??#?#?.??# 1,5,1,1
.#.????#??????? 1,1,2,1,3
#????#??..?..??????. 8,1,3,1
?#???.??###?.????.. 5,6,1,1
???#???.??? 4,1
????#?#????????#?? 10,4
?.?#???#.?.?###?.?. 1,2,1,1,4,1
?##??????????????. 5,3,5,1
.??????????.???.#. 4,1,1,3,1
.?????#..??.??#..? 4,3
...??#?.???.?##??#?. 2,7
?????###?#???#. 2,6,2
????.??#.? 1,1,3
.?.???#??. 1,4
??.????.?????.?#. 1,2,4,2
??.#???#?? 1,7
.????#??#??#?# 1,11
?...#??.?##?#??????? 1,5
###???????????.#.?? 13,1,1
.????##?????????.?#? 9,1,2,2
.#?##?..??????## 4,1,2
?##?#?????.#??.???? 6,2,3,2
?.?#??##.? 1,6
#????.???.???# 2,1,3,4
...????##???.?### 6,4
.?.?##??????#????? 6,3
.??.#???#??#..?#?..? 1,1,5,3,1
???##?##????.#???? 11,1,1
?????#????????? 1,1,2,1,2
#???..?????? 1,2,1,1
.??????#??#??#??. 7,6
.???????.???#?? 3,1,1,1,1
??#??#..????? 4,2
.????#????.??.? 4,1
#??#??.??.##?.#.?? 1,2,1,2,1,2
??????#?#?#??#? 1,1,7,1
?????????..??? 3,3,1
?.#??????##?##??? 1,1,1,6,1
#???.#?????#.????? 1,1,1,1,1,3
#?#.?#????#?##??? 3,12
#.????#..????#?#? 1,1,2,7
?#??#????.#???? 7,2,1
?.#..??#??##?#.#? 1,1,6,1,2
?#??###?#?.?????? 3,5,1,4
.?.#???#??????##.??? 1,5,7,1,1
?..???#.?. 1,1,1
?#???#??###?#.? 5,5
??????????#???#..?? 6,1,2
???#????.???## 5,5
????#??.?? 1,3,1
?????#?#.? 1,1,3
???#??#??????? 1,1,2,4
???..??#.???? 3,2
????????.#??.????.?. 8,1,1,3,1
?????.?.###?#? 3,5
?.?#?#????#??#????? 1,8,4,1
#..??#??.#.??. 1,3,1,1
??.?#?????#?.# 8,1
.??#???.?#.?#???#? 1,1,2,2,6
.?#.?##?.. 1,3
?#????..##??#??? 6,2,5
??#?#??..#??#? 7,4
#?#???#.#??.??????#? 3,1,1,1,1,6
?##.???????#???? 2,11
#?#.#?#?????###?##?. 1,1,14
?#????#.?? 5,1
??????????.??.?? 3,1,2
??????#??????????. 1,1,8,1,2
??.#?##???? 2,4,2
#.?#?##??????.#???? 1,8,1,2,1
??#???????#.#?#?# 5,1,2,3,1
?????????##???#? 2,1,4,1
?#?..?..?????.?? 3,1,1,1,1
.#?????#??..???##??? 4,3,8
?????..?.?? 2,1,1
#?##?.???#.???#? 4,1,1,3
?#?#?#?###..?.#?# 1,7,1,1,1
??#?..?.##??# 2,1,2,2
??.??#????# 2,4,2
??#?????#??#???# 13,1
??.???????#?#? 1,8,2
.???#??#?#????#???#? 1,15
?????..???. 2,2
.?.###???? 1,5,1
?##?.????#????? 3,9
.???????????##?.? 1,10
??.??#??#????.? 1,7,1,1
?#??.???#?#??.#?#? 1,1,5,3
????.###?? 2,5
??#???##??? 2,2
?.???.#?..?. 1,1
?.??##?##??##?#????? 1,12,1,1
???????????#?###? 2,2,3,3
#?.????##?##????.? 1,9
???##?????.##???.?.? 6,1,3,1,1,1
???.###???##??#..#? 1,1,11,1
#?..#???????#?# 1,3,6
??#???#?.???#?. 8,3
#??#.????#?##??#.? 1,1,3,7,1
.?.?.????? 1,1
?.?.?#?.??#? 1,1,1,2
?#????.???????# 1,2,3,4
??????#.???#?.. 1,1,5
???#?#?#??#??#?? 3,1,7
??#????.???????? 4,1,2,1,2
#???#?.??? 2,2,2
?????#????? 2,3,2
???.?#????. 1,3
##?..???##??? 3,6
.??.#???#?? 2,2,3
.?????#?????##?? 3,1,1,4
???#?????##??? 2,4
????.#???#??#???.? 1,11
???.?.???? 1,1,2
#??###???#???.#? 1,8,1,2
.????.??## 1,1,2
??????????#??????# 1,5,2,1,2,1
.????????#?#??#.? 5,4,2,1
?????.???????#?.?? 4,4
??#?.??###..? 2,5
.?????.??. 1,1,1
??????.??. 1,1,1
.?#?.?#??##?##?? 2,2,6
##?.?#???? 2,3,2
?..?#.##???????#?? 1,1,4,4
??????????##?#? 1,5,1,4
???#?????.? 2,3
?#.?#????.???#??? 1,3,1,4
.?##???.??? 3,1,1
###.????.#????#?##?? 3,2,2,7
.?#??#????#? 3,1,2,2
.#???#?????#????? 5,1,1,3
??#??#?????.???????. 9,4
..????.?.??????.??? 1,1,1,1,3,2
??.????.??.. 1,1,1,1
##???#??????.##??# 8,1,3,1
?????#??.# 7,1
.??.#?..??? 1,2,1
??#?.#?????. 2,1,1,1
.??#.?###?????.#???# 1,1,4,4,5
.??.???#?.#???# 1,2,2,2,2
##???##?.??.?## 8,1,3
?##??#??#?. 2,5
???#.???#???????? 4,10
?.?#?#????? 1,2,1
?#??.??.?..??.? 3,1,1,1
.???#.#??. 1,1,1
?#?.??.????.. 2,1,1,1
???????#?.. 4,2
????.?.??????. 1,3
???#??#???? 6,2
????#??###.??#??. 1,1,1,4,2
??????????# 2,3,1
??#?.??????##??.? 2,3,5
????...????#?# 1,1,7
.?#???.????????.? 4,5
.#?#??.?.????...?.## 1,1,1,4,1,2
?????#?.?..#. 6,1
???..?????.?#?# 3,1,3,3
????#??????#??##???. 8,7,1
#??????##?##.???? 1,7,2,2
#?.#??#????.??. 1,2,1,1,2
?#????#??.?? 3,3,1
?#????????????#?#? 5,4,5
??????????. 2,3,2
???????##?#????#?? 1,1,1,6,2,1
??#?#?#??.??. 7,1,1
??.#???#??????#? 2,1,1,5,1
?.?#.????? 1,1,1
?##???#?#???????.? 8,2
.#?##??????. 4,2
#?.###????#?.?.? 2,5,2,1,1
...??????.??.?##?. 3,4
.#?.????????#? 1,1,1,4
?.?##.?#??###?#?##.? 3,12,1
??#.???#?#?#???#? 1,1,8,2
.??#.?.????????.?? 1,7
???????????. 1,1,1,1
.??????.??. 1,1,2
.##???.?##?.??#.?? 4,3,3
.#??.?..?#???? 2,1,3
????#????? 2,2,2
.#??????####??.. 1,8
??..??#???.# 2,3,1,1
?#????#??????.? 2,4,1,1
?#?.?.?.????????## 2,1,1,1,4
.??#?#?.?#?#?##?##?? 5,10
?#?.?.?????#.?#??? 1,1,1,1,2,2
.???##.?#???#??? 5,3,5
????????.???#?.?.?? 1,4,1,1,1,1
?#?????.?? 6,1
.#?.?????##??### 2,10
?#?????????.? 1,6
?..???????##?#?#?? 1,13,1
.????????? 1,2,1
???????#?.??? 1,6,1,1
?.##??...?. 4,1
??###????##?? 5,3
#??#.??#.???#?##?#?? 2,1,3,1,8
.????????#????.??? 2,1,5,3
???#??????##?????#?? 1,1,1,10
.??#????.???##?????? 1,1,10
###??.?????.?? 3,1,4,1
????.###????#? 1,4,2
..?????????. 1,7
???#.#.?##???? 4,1,3,1
#??#??.#?.?. 2,2,1,1
????#?????#???#? 9,2
.??.##?#?.???? 2,5,2
??#?#.#?##?????? 3,1,5,2
.??#?.?#?????..??#. 2,2,3,2
#?.?#??#?#?#.? 1,8,1
??.?#?.?.?#??.#? 2,2,1,3,1
#.#?????????.????? 1,2,4,1,1
????..??????#.?# 2,3,1,1
..#???#.??.?? 5,1,1
#?.???#??? 2,6
.##.?.#.??#???#?## 2,1,1,10
..????..#??.. 3,1
#??????#??#. 1,1,1,4
??##????..????.?? 3,1,4
????.####?. 2,5
??..#???#?##?? 1,10
???.#?????.? 1,1,1,1
??????.##? 2,3
#?#?###???#??? 1,7,1,1
.?.?#.????????.? 1,4
??##.??##..#?#?#??? 2,4,1,1,4
???###?.????? 4,1
?..?.#???#####? 1,1,1,7
?.###???.??#??#???? 3,1,1,8
?###???.???? 4,3
?.?.##.#???. 1,2,2,1
???.?????#??#??# 1,6,1,2
??????#??.???##?? 9,1,4
??.????#??? 1,3
???..?????? 2,1,1
????.?.##??????. 1,1,2,2,1
.?#??##???.??#???.? 7,4,1
#?????????#??#.?? 6,7,1
???..##???? 1,2,1
###?????.?????? 3,2,1,2,2
???????##?? 5,3
???#???.??#???? 1,1,1,4
??#..??.?#?###????? 1,1,6,4
?.??.??#?? 2,4
?.????#?.?? 2,1,1
????...???????.? 3,1,1
..##?????#??. 4,3
.??..##???? 1,2,1
??.??##?#????. 1,8,1
?.#.???.##..#????# 1,1,2,2,1,3
?????##???#...?? 9,1
??#?.?.??#. 2,1,1
??.??.#???#..?#????. 1,1,1,2,5
????.???#?????###? 1,1,4,4
???????#?#??.???. 2,8,3
?#?#.?.#.?.????????? 2,1,1,1,3,3
?????.##???##?#???? 1,13
????.??#?? 2,1,1
????#??..# 1,3,1
?#??#???????????? 8,1,1,1
????#.???.#???? 1,1,2,5
????????##?#??##??#? 1,2,13
?.??##?#?#??##? 1,12
????????????#?.?# 2,1,2,4,1
#?#????#?. 1,3,2
????#??.?.#?????#? 6,1,1,3
???#??#???#??????# 1,6,1,1,1,1
??.#???###?.#??????. 2,1,1,4,1,5
??.??????###??# 1,11
?.#?#?????.????.?## 7,3,3
.#.?#.??##?#?#?.?? 1,2,8,2
?#??###?.?# 1,5,2
..#??.??????#?#?? 2,9
??#?.?.???##?????#?? 4,1,9,1
???.???#???#?? 2,1,6,1
..???????? 2,3
??????..#.? 1,1,1
????????????## 1,1,4,3
????##???#.?##?#??# 6,1,4,1
#????.#.?#?##?.#? 2,1,1,4,2
#??????...#??#????? 3,2,6
?#?##?.?????.? 5,2,1
??##.?????#?. 3,2,2
???#?.??????##. 5,1,1,4
??#???#??#???##.#.? 14,1
???????..?#??? 5,1,5
??????#?#?.#?? 2,4,1
????#?????. 1,5,1
#.?..##????#.?#???# 1,1,3,3,5
??#?#???#????#.???.? 14,2
.????#?.?? 3,1
???#???????.?????? 1,3,4,1,1,1
???.???????.. 1,2,2
?????.??#.? 1,1,2
?#..?.?#?##????#?.. 1,11
#?.?##??????#.?. 2,2,5
.?#???##???.? 6,2
??#?#????#.???# 7,1,1,1
?.????#??????#?????# 1,3,2,3,1,3
???##?#???##..?#??.? 1,3,1,3,4,1
.?.?.#?#?#?##?##?.?? 1,10
.?????#??#???#?.. 2,5
???..????.?. 2,1,1
..##??????.?? 4,2
????#??.???#??????? 3,1,9
??????..??#???? 3,3,1
...#.#??????##?. 1,1,2,4
???#.??.?#??? 4,1,2,1
???.?#??.#??###? 1,2,6
???#?????????##?.? 1,2,1,6
???????????.???. 9,1,1
?.?###?##???. 1,3,3,1
.#??.?.????????. 1,1,3,1,2
.?.#???..??? 1,1,1,1
#?.??#.?.. 1,1,1
#.#?.???#??###???#.? 1,1,1,11
.???..?###???#?##?? 2,6,5
?.??#???#?##??.?. 4,5,1
??????????#????.?. 2,8,1
.#?....????????. 2,1
??????.?##?. 1,2,4
??????.?#?#??? 2,1,4,1
?.????#?#?????.? 1,7,1,1
#???#???..#.??#. 1,1,4,1,3
#???#?#?#.# 2,5,1
.????.#?????.# 1,1,6,1
??#?????.#???##??? 3,1,1,2,3,1
##???????#?##.??. 13,1
.??.#?#?????? 1,1,6
?????????##?? 1,2,5
??.??.?#.??#?#???#?? 2,1,1,3,6
??.??.#???.??.? 2,1,1,1,1
.??#?????? 2,3
????.??#?##?.?# 1,1,6,1
???#???#?#?? 1,3,1,1
#..?#????????.#??#?# 1,9,1,4
???#????#.?.???#? 3,2,1,4
???##??.??.?#?##.#? 5,5,2
??.?#??#.??.? 2,1
#.???#????.?#?.?? 1,3,3,3,1
.?????#?.??.?? 1,2,1,2
???.??##????? 7,1
???#????.?#??#????? 3,8
????#???.???##? 5,1,1,3
?????????????. 5,1,4
????.#??#?# 1,6
????...?.?#??.????? 1,1,1,4,4
???.??????#?????.#. 1,3,2,3,1
?##????#?#?#?????.? 4,6,1,1
..?#???....?##?##?? 3,7
?.?.?#?.?.?#???.?. 2,3
???#?????? 4,1
..?.??#??.?#??? 4,3
???..????#??.???#?. 2,5,1
?.#?.??.?? 1,1,1
????#?###.???#?#?#?? 7,8
????#?#????#?..???.? 4,7,2
???#?.?.?.?. 1,1
?.##.??#?. 1,2,2
?.?????????????? 4,4
.?#.?.??.????#??? 2,1,7
?.???.????. 1,1,1
?#??#???.?##???#??? 5,9
??#?????##..?# 4,3,2
#????????.?#??#.#.# 9,1,2,1,1
.????????#?? 1,2,2
???????.?? 1,2,1
???????.?##.# 2,1,3,1
???.?#?####??#??#?#? 2,7,7
?.?...??###?????? 1,5,1,1
?.???#?????#????? 10,3
??##?.??##????#????? 4,5,3
#?.???###????. 1,2,6
?#.?????.?. 1,4,1
?????####?.????##? 1,1,6,2,3
..?.??????#????###?? 1,16
???..?#??#?#?????? 1,9,1
?#??#??.?#???#?? 1,4,8
?????????.?##???.?? 1,1,3,3,1,1
?.?#??##??##????.?. 10,1
##.?.?#??. 2,1,4
#.?.#????????#???#?? 1,1,1,1,2,8
????##?#??? 1,4
??????##?? 2,2
?##???.#?#??? 5,3,1
??..????????? 1,2,2,1
????????????? 1,1,5
??..#???##?.?.?# 1,3,2,1,1
?#??#??##?????.. 4,2,1
?#??#??.?#??#??. 5,5
##??????#???? 10,1
#??#??????.?#?? 1,4,1,2,1
??#?#??.??.?.???.??? 7,1,1,1,1
?##?#????. 6,2
????????###???????? 2,1,11
??????#??????? 3,1,2
?.?#.?????????#???.? 1,1,1,8,1,1
#???#?.???#??#?????# 1,2,5,3,1
???.??#.????? 1,1,2,1
.????..#?##??#?#. 2,9
#?#??????? 5,1
##???????#??.?# 3,8,1
????#???#???? 1,2,1,3
?.#.?????###?#???? 1,10
?????...#??.#??.. 4,2,1
..??.??????.? 1,2
.?.?.?.??#.?. 1,1
..??#.??????? 2,4
#??#???#???.?#??.?? 6,1,2,1,1,1
..???????.??#??.?? 5,5
???.?#??..?????. 1,4
?.???#?#???????#. 5,1,1
????##???# 1,5,1
#???##???????#? 2,4,2,3
?#?##??????? 6,1,2
??#??###?????# 11,1
?#??#?????..#??###?? 8,7
#.?????.?# 1,4,1
#?##?????????#????? 5,10,1
#???#??#?.??.?#??#? 9,1,1,1
?###?????#? 5,1
???.????..#?###??. 1,2,7
??.?.?.??#?#?. 2,1,5
??#??##?.???#?#???. 4,3,1,1,2,1
?????#??????##.#.#?? 2,9,1,1
#?.?.#?.??????? 1,1,1,1,2
.?#??????????.??.? 6,2,1,1
?.??..?????#?.? 1,6
.?????.???#?????# 1,2,3,1,2
.#??#??#?#??#..?##? 1,2,1,2,1,3
?.?###?????#?###??. 5,8
??#?#?.???.#?##???? 1,1,1,1,5,2
????.???#???????.??# 1,1,5,2,1,1
??#.????##???#?..#. 3,4,1,1,1
????#?..?#.????? 1,1,2,2,1
??#?####????####???? 8,8
???.#?#??#.?? 1,3,2
???#???##?.#.??????? 1,7,1,1,1,1
?????#?.#?.??##.??.? 5,2,1,2,1
?#??????#??????## 9,2
?#??#?#??#?#???? 1,1,10
???????#?#.?# 1,5,1,2
??#?????????. 1,1,8
??????.??? 4,1
?????#?.??##?##.# 2,3,6,1
?#?#???????#??# 6,4,1
??##?????????#??#??? 1,15,1
??#?#?###????#..?? 8,2,2
??#?#?#??.????????. 6,1,1,1,1,1
???.?.??????#?.??? 2,1,8,1
????#???.?.??? 6,2
?#?.????.?????###### 2,4,1,1,7
#????????#?#??#?? 2,2,9
#????##???.?#.. 8,2
??#?????#????#??#? 3,2,10
???#??#???? 4,2,2
..?.???#.#??#?? 1,3,1,4
???#?.????#? 3,6
.??????##?#?.? 1,2,4,1
??.???#?.?#?##? 1,5,5
??##?#??????##??##.? 4,1,9
.#??#??.?##.? 4,3
.??#??#???#????? 3,7
???????#??#?##??. 5,8
.????????????#?????? 2,11
.##????#??#??.????#? 11,4
?##??..?.? 3,1
???#.?.?#?? 1,1,2
..#??????.??#??? 7,5
??.#?#.????????#?# 1,1,1,2,7
.#????.????#???.? 5,7
#??#???.??? 4,2
##???.?.#.?.#??.# 4,1,1,1,1
?##?.???#?? 2,2
?.??.?#???????###?? 1,1,6
???.?#???.?? 2,1,1,1
.?..?.##???#???# 1,6,2
???#???.#..? 7,1,1
.?.?..#???????? 1,8
??.#?##??????? 1,8,1
?.?#??#?#.??#.#? 1,2,3,3,1
????##?????#?.. 1,2,1,1
???#.??.?????? 1,1,2,2
??###?#?#.?..?.? 9,1,1
..?????#?##? 3,5
??#????.??? 6,2
????##?.??????? 6,2,1,1
?#?#??#??.??? 8,2
?#?????#??. 3,3
??????#?## 2,5
????.#???? 2,1,1
.#?#??..???. 5,3
?????#?##???? 1,2,5
??.#.??.#..#.????? 1,1,1,1,1,3
?????????.? 6,1
?#?????#.??.# 8,1,1
?.?#?#..???? 4,2
???#.????#? 4,5
#??????.???? 1,1,1,3
??#??##????.????? 7,4
#..???????????.??? 1,5,1,1,1
.??#?.??????? 3,6
.???..?#?##? 2,5
?#???#??????#?????? 4,2,1,3,3
??.???????##?# 1,7
??.??.?#.? 2,2
?????????.#?? 2,1,2,3
#.????????#?????.? 1,1,5,1,1
???????.??#? 3,1,2
?.???#??????#.?. 5,1,1,1
#?..?##?????#?#????# 2,5,6,2
?????#.??.?###.#??# 4,1,3,4
?.????.?.? 1,1
???##???.?.?????? 3,1,1
##?.#??????.??? 3,3,1,1
.?????#?#?#.# 1,7,1
#?.???#?????. 1,4,3
?#????.#?#???. 3,1,6
?#????????#????? 4,7
?#??..??.?#??#??#.? 4,2,2,5,1
??.???????.??? 3,1,1,1
???.????.?.??#???.? 1,5
?#???.????..?????.?? 3,1,3,4,1
???#??.#???##???? 1,2,1,8
.#??#?#???#??? 7,2
????????.??#.?? 1,1,1,3,1
??##???#??#. 3,4
???#?.?.?. 1,1,1
??.???????.? 2,4
?#???#?#????. 6,5
?.??##?.?#?????? 5,8
##?????..?.? 3,1,1
..??#??.#???##???? 4,7,1
???.?.##?.. 2,2
?.??????.?.?#??.??#? 4,1,4
.??#?###?#??#????.?. 14,1
????.???.?##??.???# 2,1,5,1,1
.?????.???????#.??#? 4,1,1,4,1,2
???...?????? 2,1,2
??##?#.??????#.? 1,2,1,1,3
?.?##???#? 4,1
.#????.?#?? 3,3
??#??#?????#?.#?#? 7,2,4
???#???#?????#???# 14,2
.?..????????#????##? 1,1,13
????#?##.?????.??#? 8,1,2
?#???#?#???? 1,1,1,1
?#...??..? 1,1
#.???#??.? 1,3
???????????????. 2,4
.??#.???.#?#?# 3,5
..#???????.????#??? 1,2,1,5,1
?#?????..??#?? 5,3
.#?##??.??#??? 4,1,1,1
???.??.????# 1,1,1,4
????#?#??#?? 1,2,1,1
?#?#?.??##??#??. 3,6,1
???##???????#???.#? 1,4,1,4,1,1
#??.#..#?#?? 2,1,1,1
?..??#????. 2,1
????????.?. 1,1
????????????? 2,1,1,1
??#..??#???#?##??# 1,1,1,11
???????????? 1,3,3
..???.????#?#??#?? 1,5,3
?????????.##??? 3,1,1,4
???????#?#?#???. 1,9,1
#?????#?#?.??#????. 5,4,1,1,1,1
????.#.?.??#?.??.. 1,2
??.#??.?..????###?# 2,1,1,1,1,6
????#.?.????#??. 1,2,1,4
?????.??#????.???? 3,3,1,1
?..#?#??#?#?? 1,4,4
????.?#??? 1,3
?.#?#?######?. 1,10
#???#???#.?.?#???? 3,2,2,1,5
#????.???????.#??? 2,1,4,1,1
???###??????? 6,2,1
????.#??#? 1,1,1
???#?#?#?#????.#???? 14,1,2
..?????.??? 4,2
?????.??#?#????? 3,7
?.##??#??##?#????? 3,5,2,3
..??#???.?#? 1,2,2
?#??#????##??#?. 2,4,3,2
##???#?.?#???.?? 3,1,1,1,1
??#????#??.#?#??#.#. 7,3,1,1
????#?#.??.#? 4,1
???#?.???? 2,2
#?????????????.? 1,9,1,1
????#?#?#.??? 8,2
.?????????#?? 1,3,2
?.???###?.??#?#?##?? 1,6,8,1
???.????#?.#???##. 2,1,2,1,3
??##???#?? 2,1,1
..?.??.?##?# 1,5
??????#??.?? 8,1
???#?#?.?#?.?#?? 2,4,1,2,1
?.#.?????#?##?#???.# 1,3,6,1,1
??.????????? 1,2
.????????? 2,3
?????#?#?? 1,4
??????#??#?.????? 1,4,1
?#????#.??##? 1,3,2
???###?#???#?#??#?? 2,11,2
??#???.?????#???? 4,5
??.#????#?..#? 1,1,5,1
???#?????.#.??.?? 5,1,1,1,1
??#????#?.#??. 4,2,1
?.?????#????##????? 1,1,2,1,7,1
?#?.#?.???.#?## 1,1,1,4
???#???.?#????##? 1,4,4,3
??????.?##??##????. 1,1,1,6,1
..?????.##??? 3,2,1
?#?#.?????#?# 3,2,4
?.?##??#???.? 1,7,1
????#.???#??#?## 4,1,1,2,2
?#??###?.??.?#? 7,1,1
???????.#?#?#?????? 4,6,1,1
??#????.#??? 1,1,1,1
.#?#??.?.? 1,1,1
##?#??.#???#? 4,1,2
..????#???#. 2,1,2
?????????.? 1,4
?#####??##?.????#. 11,1,1
####?##.?.?. 7,1
#.?#??.???.###? 1,3,3,4
.#????#???? 1,3
??????#????.??? 9,1,1
????#?????#? 3,1,2
?????##?????#?.? 7,2,1,1
????.##??????#??##?# 1,3,2,6,1
?#????????????. 7,4
???#.?.?#?????##?? 2,9
???..??.?..#?# 1,2,1,3
?#..???.?.#?.????? 1,3,1,1,1,1
??.??#????.?##.? 2,3,2,3
???#??#????? 1,1,2,1
????##??.?????? 2,5,1,2
#?????#.#?#??#??# 1,1,1,7,1
???#??#??.??. 1,4,1
??????#.?#????#? 5,2,3
?.??##??????##??#?# 15,1
??.???????##??#??? 2,11,2
???#??#??..#???.? 5,2
?.#????????#...#? 3,5,1
???#?????#??.?#?##?? 1,1,2,3,5,1
#?#.#?????. 3,3,1
?????.##.??#???? 1,1,2,3,1
??.?#?#????##??????. 1,3,5,1,1
.?????##??.#??. 7,3
#.?????????#?#???. 1,3,6,1
?????#.?.??#??#.? 2,1,5
??#?..?.???? 2,3
?.#.?.?????##?.#?? 1,1,1,1,2,2
.??????.?. 2,3,1
?.??....???? 1,2
?###..??.#????#??#? 4,2,1,1,2,1
?#?.?????#??.#?.?? 1,8,2,1
.????..?#???????? 2,10
.#?????##?? 4,3,1
????#?#?????#??.? 8,2,1
?#????????##??? 3,6
?.??????.???????#.? 1,3,1,2,1,1
???????????????#?..? 2,10,1
#??##??.????#? 2,4,1,3
#?????.??. 3,1
?????.?#??? 1,2
.#??#??#?#??##.?.?# 13,2
?#???#.??? 1,2,3
?.?####?#??.?.?#? 7,1
.??.?.???? 2,1,1
#.??#???.??????????? 1,4,2,2,1,3
?.?.##??.?. 1,3,1
????#????????? 6,5
.??.????##.? 1,5
???#?...#?#??? 3,4
????.?????#?? 1,6
??..?..???#?? 1,1,1,2
?#??#?.?###.# 5,3,1
.?##?#?.#??#???? 6,1,6
.##???#????#.???? 8,2,1,1
.????#.??#..??.##?. 5,3,1,2
?????.??#? 2,3
???####??????# 10,1
.?#??..#.?? 3,1,1
?#??..#???.?? 3,4
#?..???#??. 2,1,2
???????#???###????#. 1,13
????##???#...#??#.? 2,2,1,1,2,1
???#???..? 3,1,1
???.?.???? 1,4
?###????????.#? 8,1
.???#????.?.???? 4,3,4
?#????.???. 1,1,3
??????..?..?#??? 4,1,1,1,1
?????#.???#?. 4,2
?#?#????..????? 1,1,2,1,1
?#?.##??#?. 2,3,1
????.?#?#? 2,2,1
??###?##???#? 5,2,2
?#??#####??#.?.#?#? 2,8,4
??#?????????#? 3,2,3
#???.?#??? 2,1,1
??###?##???#???.??.? 8,3
??...#?#?.#??? 1,3,1,1
####????????... 9,1
.?#?...??#???#??.? 2,6
?#???????#???#???.? 1,1,1,4,5,1
?###???#?.#?.?.?#? 3,1,2,3
?????.?????.?#??? 4,4,3
??#?.?#???? 1,4
#.??#?##????? 1,5,1
#.???????#???#? 1,1,1,2,2
???.?#???????#???##? 2,6,1,6
?#.??#..???#?. 1,1,5
?.?##?.#??..????#? 1,4,3,5
.?#?#?..??? 4,1
?????#??#????????? 2,3,3,3
??.??????#?##?#??#?? 2,10,1
?#?##???????.??. 7,1,1
?#????#?????##?????? 7,1,5,3
???#?#.??? 4,1
?###??##??.?#?.??#? 10,1,2
.?#?#?.#????..????? 4,1,3,4
#???????.???#?????.? 8,1,3,1,1
?.#??#.?#??#?#??#? 1,1,7,2
#?#?????#???.?.. 6,3,1
?.#?#????????? 1,6,1,1
.#????#?..? 2,3
??#????.???#.?????? 6,1,1,1,1,1
??..?????.????????? 5,3,1,2
????????#?? 1,4
?.?#?#.###?.#??.?? 1,4,4,1,1
????.?????? 1,4
.??#?#????# 7,1
.#.????.?#? 1,1,1
###???????????.#.#? 11,1,1,2
.#?#???????#?? 4,2,3
###??#?.###?#??. 4,1,5
##??#..????? 3,1,5
?.?.????#?.??????? 1,1,3,6
.#?#???.??##.????.?. 5,4,1
.?#????#?#?.?#.?? 10,1,1
???###?##?????#??? 12,1
#.#????.?#????#?.# 1,5,2,4,1
.?.?#???#??#?.#.? 1,2,2,1,1
?#.?.????#?. 1,4
..??##?.?????? 5,1,1
???.?##??????#?##?? 1,3,9
??????#??? 3,3
???#????????????.??. 7,1
??.??#?.#??#?.? 3,1,3,1
?.???????#???#?.#?. 1,12,2
#?#?#??.?.?????? 3,2,1,6
????#.??##?? 3,1,3,1"
        .to_string()
}
