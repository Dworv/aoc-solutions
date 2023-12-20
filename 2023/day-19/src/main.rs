use std::{collections::HashMap, ops::Range};

use utils::rts;

fn main() {
    let input = rts(19);
    let input = parse(&input);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &(Vec<Workflow>, Vec<Part>)) {
    let (workflows, parts) = input;

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow.clone());
    }
    let start_workflow = workflow_map["in"].clone();

    let mut sum = 0;
    for part in parts {
        let mut dests = vec![];
        let mut dest = Destination::Workflow(start_workflow.name.clone());
        while let Destination::Workflow(name) = dest {
            dest = apply_workflow(part, workflow_map.get(&name).unwrap());
            dests.push(dest.clone());
        }
        if dest == Destination::Accept {
            sum += part.a + part.s + part.m + part.x;
        }
    }

    println!("Result: {}", sum);
}

fn part_2(input: &(Vec<Workflow>, Vec<Part>)) {
    let (workflows, _) = input;
    // let workflows = vec![
    //     Workflow {
    //         name: "in".into(),
    //         rules: vec![
    //             // Rule { categ: 'x', check: '<', target: 2, dest: Destination::Reject },
    //         ],
    //         default: Destination::Accept
    //     }
    // ];

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow.clone());
    }

    println!("Result: {}", num_combos(HyperSelect {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001
        // x: 1..4,
        // m: 1..3,
        // a: 1..3,
        // s: 1..3
    }, Destination::Workflow("in".into()), &workflow_map));
}

fn num_combos(mut selection: HyperSelect, prev_dest: Destination, workflows: &HashMap<String, Workflow>) -> u64 {
    // dbg!(selection.clone());
    let workflow = match prev_dest {
        Destination::Workflow(name) => workflows.get(&name).unwrap(),
        Destination::Accept => return dbg!(dbg!(selection).volume()),
        Destination::Reject => return 0
    };
    if selection.volume() == 0 {
        return 0;
    }
    let mut sum = 0;
    for rule in &workflow.rules {
        // println!("judging {:?} with {}{}{}", selection, rule.categ, rule.check, rule.target);
        let pre_cut = selection.clone();
        let categ = match rule.categ {
            'x' => &mut selection.x,
            'm' => &mut selection.m,
            'a' => &mut selection.a,
            's' => &mut selection.s,
            _ => panic!()
        };
        let mut breakoff = pre_cut.clone();
        let breakoff_categ = match rule.categ {
            'x' => &mut breakoff.x,
            'm' => &mut breakoff.m,
            'a' => &mut breakoff.a,
            's' => &mut breakoff.s,
            _ => panic!()
        };
        println!("{}{}{}: {}..{}", rule.categ, rule.check, rule.target, categ.start, categ.end);
        match rule.check {
            '<' => {
                let cutin = rule.target.min(categ.start);
                let cutoff = rule.target.min(categ.end);
                *breakoff_categ = cutin..cutoff;
                categ.start = cutoff;
                println!("< {:?} and {:?}", selection, breakoff);
                sum += num_combos(breakoff.clone(), rule.dest.clone(), workflows);
            }
            '>' => {
                let cutin = (1+rule.target).max(categ.start);
                let cutoff = (1+rule.target).max(categ.end);
                *breakoff_categ = cutin..cutoff;
                categ.end = cutin;
                println!("> {:?} and {:?}", selection, breakoff);
                sum += num_combos(breakoff.clone(), rule.dest.clone(), workflows);
            }
            _ => { panic!() }
        };
        if selection.volume() == 0 {
            return sum;
        }
    }
    sum + num_combos(selection, workflow.default.clone(), workflows)
}

#[derive(Debug, Clone)]
struct HyperSelect {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>
}

impl HyperSelect {
    fn volume(&self) -> u64 {
        (self.x.end - self.x.start) * (self.m.end - self.m.start) * (self.a.end - self.a.start) * (self.s.end - self.s.start)
    }
}

fn apply_workflow(part: &Part, workflow: &Workflow) -> Destination {
    for rule in &workflow.rules {
        let component = match rule.categ {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("bad categ")
        };
        let caught = match rule.check {
            '<' => component < rule.target,
            '>' => component > rule.target,
            _ => panic!("bad check")
        };
        if caught {
            return rule.dest.clone();
        }
    }
    workflow.default.clone()
}

fn parse(input: &String) -> (Vec<Workflow>, Vec<Part>) {
    let mut workflows = vec![];
    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (name, the_rest) = line.split_once('{').unwrap();
        let raw_rules: Vec<_> = the_rest.split(',').collect();
        let mut rules = vec![];
        for rule in &raw_rules[0..raw_rules.len() - 1] {
            let (catch, dest) = rule.split_once(':').unwrap();
            let categ = catch.chars().nth(0).unwrap();
            let check = catch.chars().nth(1).unwrap();
            let target = &catch[2..].parse::<u64>().unwrap();
            rules.push(Rule {
                categ,
                check,
                target: *target,
                dest: match dest {
                    "A" => Destination::Accept,
                    "R" => Destination::Reject,
                    _ => Destination::Workflow(dest.to_string())
                }
            });
        }
        let default = raw_rules.last().unwrap();
        let default = &default[..&default.len() - 1];

        workflows.push(Workflow {
            name: name.to_string(),
            rules,
            default: match default {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                _ => Destination::Workflow(default.to_string())
            }
        });
    }

    let mut parts = vec![];
    for line in lines {
        let components: Vec<_> = (&line[1..line.len() - 1]).split(',').map(|x| (&x[2..]).parse::<u64>().unwrap()).collect();
        parts.push(Part {
            x: components[0],
            m: components[1],
            a: components[2],
            s: components[3]
        });
    }
    (workflows, parts)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: Destination
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    categ: char,
    check: char,
    target: u64,
    dest: Destination
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination {
    Workflow(String),
    Accept,
    Reject
}