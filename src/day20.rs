use std::{collections::HashMap, fmt::Display, hash::Hash};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

impl Hash for ModuleType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ModuleType::Broadcast => "broadcast".hash(state),
            ModuleType::FlipFlop(_) => "flipflop".hash(state),
            ModuleType::Conjunction(_) => "conjunction".hash(state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs_to: Vec<String>,
}

impl Module {
    fn process_input(&mut self, signal: &Signal) -> Vec<Signal> {
        match &mut self.module_type {
            ModuleType::Broadcast => self
                .outputs_to
                .iter()
                .map(|name| Signal {
                    from: self.name.to_string(),
                    to: name.to_string(),
                    pulse: signal.pulse,
                })
                .collect(),
            ModuleType::FlipFlop(ref mut is_on) => {
                if signal.pulse == Pulse::High {
                    return vec![];
                }

                *is_on = !*is_on;

                self.outputs_to
                    .iter()
                    .map(|name| Signal {
                        from: self.name.to_string(),
                        to: name.to_string(),
                        pulse: match *is_on {
                            true => Pulse::High,
                            false => Pulse::Low,
                        },
                    })
                    .collect()
            }
            ModuleType::Conjunction(ref mut memory) => {
                memory
                    .get_mut(&signal.from)
                    .and_then(|pulse| Some(*pulse = signal.pulse));

                let pulse = match memory.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };

                self.outputs_to
                    .iter()
                    .map(|name| Signal {
                        from: self.name.to_string(),
                        to: name.to_string(),
                        pulse,
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{:?}-> {}", self.from, self.pulse, self.to)
    }
}

fn main() {
    assert_eq!(32_000_000, part1(&test_input()));
    assert_eq!(11_687_500, part1(&test_input_2()));
    assert_eq!(898_557_000, part1(&input()));
    assert_eq!(0, part2(&test_input()));
    assert_eq!(0, part2(&input()));
}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);

    let (mut low_pulse_count, mut high_pulse_count) = (0, 0);

    for _ in 0..1000 {
        let mut signals: Vec<Signal> = vec![Signal {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        }];

        while !signals.is_empty() {
            // for signal in &signals {
            //     println!("{}", signal);
            // }

            low_pulse_count += signals
                .iter()
                .filter(|signal| signal.pulse == Pulse::Low)
                .collect::<Vec<_>>()
                .len();
            high_pulse_count += signals
                .iter()
                .filter(|signal| signal.pulse == Pulse::High)
                .collect::<Vec<_>>()
                .len();

            let mut next_signals: Vec<Signal> = vec![];

            for signal in &signals {
                let module = modules.iter_mut().find(|module| module.name == *signal.to);

                if let Some(module) = module {
                    let output = module.process_input(signal);
                    next_signals.extend(output);
                };
            }

            signals = next_signals;
        }
    }

    low_pulse_count * high_pulse_count
}

fn part2(input: &str) -> usize {
    let mut modules: HashMap<String, Module> = parse(input)
        .iter()
        .map(|module| (module.name.to_owned(), module.to_owned()))
        .collect();

    let mut cache: HashMap<(Signal, Module), Vec<Signal>> = HashMap::new();

    let mut number_of_button_presses = 0;

    loop {
        // This is a single button press
        let mut signals: Vec<Signal> = vec![Signal {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        }];

        number_of_button_presses += 1;

        if number_of_button_presses % 1_000_000 == 0 {
            println!("We have performed {number_of_button_presses} button presses");
        }

        while !signals.is_empty() {
            // for signal in &signals {
            //     println!("{}", signal);
            // }

            let mut next_signals: Vec<Signal> = vec![];

            for signal in &signals {
                let module = modules.get_mut(&signal.to);

                if let Some(module) = module {
                    let output: Vec<Signal> = cache
                        .entry((signal.clone(), module.clone()))
                        .or_insert_with(|| module.process_input(signal))
                        .to_vec();
                    next_signals.extend(output);
                };
            }

            signals = next_signals;
        }

        if signals
            .iter()
            .any(|signal| signal.to == "rx" && signal.pulse == Pulse::Low)
        {
            return number_of_button_presses;
        }
    }
}

fn parse(input: &str) -> Vec<Module> {
    let re = Regex::new(r"(?:(&|%)(\w+)) -> (.*)").unwrap();
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
