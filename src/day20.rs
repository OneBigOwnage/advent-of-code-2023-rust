use regex::Regex;

#[derive(Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(String, Pulse)>),
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs_to: Vec<String>,
}

fn main() {
    assert_eq!(32_000_000, part1(&test_input()));
    assert_eq!(11_687_500, part1(&test_input_2()));
    assert_eq!(0, part1(&input()));
    assert_eq!(0, part2(&test_input()));
    assert_eq!(0, part2(&input()));
}

fn part1(input: &str) -> usize {
    dbg!(parse(input));

    todo!();
}

fn part2(input: &str) -> usize {
    todo!();
}

fn parse(input: &str) -> Vec<Module> {
    let re = Regex::new(r"(?:(&|%)(\w+)|broadcaster) -> (.*)").unwrap();
    let broadcaster_re = Regex::new(r"broadcaster -> (.*)").unwrap();

    let conjunction_mapping = input
        .split("\n")
        .map(|line| -> (String, Vec<String>) {
            if line.contains("broadcaster") {
                let (_, [raw_outputs_to]) = broadcaster_re.captures(line).unwrap().extract();

                return (
                    "broadcaster".to_string(),
                    raw_outputs_to
                        .split(", ")
                        .map(|name| name.to_string())
                        .collect(),
                );
            } else {
                let (_, [_, name, raw_outputs_to]) = re.captures(line).unwrap().extract();
                return (
                    name.to_string(),
                    raw_outputs_to
                        .split(", ")
                        .map(|name| name.to_string())
                        .collect(),
                );
            }
        })
        .collect::<Vec<(String, Vec<String>)>>();

    input
        .split("\n")
        .map(|line| {
            if line.contains("broadcaster") {
                let (_, [raw_outputs_to]) = broadcaster_re.captures(line).unwrap().extract();

                Module {
                    name: "broadcaster".to_string(),
                    module_type: ModuleType::Broadcast,
                    outputs_to: raw_outputs_to
                        .split(", ")
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>(),
                }
            } else {
                let (_, [raw_type, name, raw_outputs_to]) = re.captures(line).unwrap().extract();

                Module {
                    name: name.to_string(),
                    module_type: match raw_type {
                        "%" => ModuleType::FlipFlop(false),
                        "&" => ModuleType::Conjunction(
                            conjunction_mapping
                                .iter()
                                .filter(|(_, to)| to.contains(&name.to_string()))
                                .map(|(from, _)| (from.to_owned(), Pulse::Low))
                                .collect(),
                        ),
                        other => panic!("Cannot parse {other} into ModuleType"),
                    },
                    outputs_to: raw_outputs_to
                        .split(", ")
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>(),
                }
            }
        })
        .collect::<Vec<Module>>()
}

#[allow(dead_code)]
fn test_input() -> String {
    "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
        .to_string()
}

#[allow(dead_code)]
fn test_input_2() -> String {
    "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "&kv -> qb
%px -> qz, tk
%xk -> sv, zv
%rj -> lx, qz
%ks -> fc
%dx -> gt, dr
%lz -> qz, df
%dz -> fr
broadcaster -> cn, xk, rj, gf
%ct -> ks
%hq -> bz
%qv -> cx
&qz -> vk, qm, rj, kv, hq, tk
&jg -> qb
%cf -> sv, tz
&dr -> cn, jz, tq, ks, mr, ct
%mx -> bn
%bv -> sk, kf
%cn -> dr, mq
%vk -> lz
%jd -> qz
&qb -> rx
%tp -> sv, lm
%jz -> ct
%tq -> tj
%bn -> sv, cf
%br -> sk, hc
%gt -> dr, nd
%nd -> dr, nk
&rz -> qb
%lx -> qm, qz
&sk -> qv, kf, rd, qh, jg, gf
%mq -> jz, dr
%rl -> bv, sk
%tz -> sv, ng
%df -> qz, jd
%tk -> hq
&mr -> qb
%gf -> rl, sk
%qm -> nt
&sv -> xk, rz, zv, dz, mx
%hc -> sk, nf
%xp -> br, sk
%bc -> sv, tp
%fc -> dr, tq
%nf -> sk
%cx -> sk, qh
%bz -> vk, qz
%zv -> dz
%kf -> rd
%tj -> dr, dx
%fr -> mx, sv
%ng -> bc, sv
%lm -> sv
%nk -> dr
%nt -> qz, px
%qh -> xp
%rd -> qv"
        .to_string()
}
