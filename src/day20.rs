enum Pulse {
    Low,
    High,
}

enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction(Vec<(String, Option<bool>)>),
}

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
    todo!();
}

fn part2(input: &str) -> usize {
    todo!();
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
