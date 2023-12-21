use std::collections::{HashMap, VecDeque as Deque};

use utils::rts;

fn main() {
    let input = parse(&rts(20));

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &HashMap<String, Module>) {
    let mut modules = input.clone();
    let mut queue = Deque::new();

    let mut num_lows = 0;
    let mut num_highs = 0;
    for _ in 0..1000 {
        num_lows += 1; // button press
        for reciever in modules["broadcaster"].outgoing.iter() {
            num_lows += 1;
            queue.push_back((reciever.clone(), "broadcaster".to_string(), Pulse::Low));
        }

        while let Some((name, from, pulse)) = queue.pop_front() {
            let module = modules.get_mut(&name).unwrap();
            match &mut module.kind {
                ModuleKind::FlipFlop(on) => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        let msg = Pulse::from(*on);
                        for reciever in module.outgoing.iter() {
                            if msg == Pulse::Low {
                                num_lows += 1;
                            } else {
                                num_highs += 1;
                            }
                            queue.push_back((reciever.clone(), name.clone(), msg.clone()));
                        }
                    }
                },
                ModuleKind::Conjunction(memory) => {
                    *memory.get_mut(&from).unwrap() = pulse.clone();
                    let msg: Pulse = memory.values().any(|pulse| *pulse == Pulse::Low).into();
                    for reciever in module.outgoing.iter() {
                        if msg == Pulse::Low {
                            num_lows += 1;
                        } else {
                            num_highs += 1;
                        }
                        queue.push_back((reciever.clone(), name.clone(), msg.clone()));
                    }
                }
                ModuleKind::Untyped => {},
                _ => { panic!() }
            }
        }
    }

    println!("Results: {}", num_lows * num_highs);
}

fn part_2(input: &HashMap<String, Module>) {
    let mut modules = input.clone();
    let mut queue = Deque::new();

    // silly things
    let final_conj = modules["rx"].incoming[0].clone();
    let mut end_conjs = modules[&final_conj].incoming.clone();
    let mut end_nums = vec![];

    let mut press = 0;
    while end_conjs.len() > 0 {
        press += 1;
        for reciever in modules["broadcaster"].outgoing.iter() {
            queue.push_back((reciever.clone(), "broadcaster".to_string(), Pulse::Low));
        }

        while let Some((name, from, pulse)) = queue.pop_front() {
            let module = modules.get_mut(&name).unwrap();
            match &mut module.kind {
                ModuleKind::FlipFlop(on) => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        let msg = Pulse::from(*on);
                        for reciever in module.outgoing.iter() {
                            queue.push_back((reciever.clone(), name.clone(), msg.clone()));
                        }
                    }
                },
                ModuleKind::Conjunction(memory) => {
                    *memory.get_mut(&from).unwrap() = pulse.clone();
                    let msg: Pulse = memory.values().any(|pulse| *pulse == Pulse::Low).into();
                    for reciever in module.outgoing.iter() {
                        if end_conjs.contains(&name) && msg == Pulse::High {
                            end_nums.push(press);
                            end_conjs.swap_remove(end_conjs.iter().position(|x| x == &name).unwrap());
                        }
                        queue.push_back((reciever.clone(), name.clone(), msg.clone()));
                    }
                }
                _ => {}
            }
        }
    }

    println!("Results: {}", end_nums.iter().fold(1, |acc, x| lcm(acc, *x)));
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let mut name = parts.next().unwrap().to_string();
        let kind = if name == "broadcaster" {
            ModuleKind::Broadcast
        } else if name.starts_with('%') {
            name = name[1..].to_string();
            ModuleKind::FlipFlop(false)
        } else if name.starts_with('&') {
            name = name[1..].to_string();
            ModuleKind::Conjunction(HashMap::new())
        } else {
            panic!("Unknown module type: {}", name);
        };

        let module = modules.entry(name.clone()).or_insert(Module {
            incoming: vec![],
            outgoing: vec![],
            kind: ModuleKind::Untyped
        });

        module.kind = kind;

        parts.next(); // skip arrow
        let mut recievers = vec![];
        for mut reciever in parts {
            reciever = reciever.trim_end_matches(','); // remove trailing comma
            module.outgoing.push(reciever.to_string());
            recievers.push(reciever)
        }

        for reciever in recievers {
            let reciever = modules.entry(reciever.to_string()).or_insert(Module {
                incoming: vec![],
                outgoing: vec![],
                kind: ModuleKind::Untyped
            });
            reciever.incoming.push(name.clone());
        }
    }

    for (_, module) in modules.iter_mut() {
        if let ModuleKind::Conjunction(map) = &mut module.kind {
            for input in module.incoming.iter() {
                map.insert(input.clone(), Pulse::Low);
            }
        }
    }

    modules
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Module {
    incoming: Vec<String>,
    outgoing: Vec<String>,
    kind: ModuleKind
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Untyped
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low
}

impl From<Pulse> for bool {
    fn from(value: Pulse) -> Self {
        match value {
            Pulse::High => true,
            Pulse::Low => false
        }
    }
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        match value {
            true => Pulse::High,
            false => Pulse::Low
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcf(a, b)
}

fn gcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcf(b, a % b)
}
