use std::collections::BTreeMap;

fn parse_mappings(lines: &str) -> BTreeMap<&str, (&str, &str)> {
    lines
        .trim_start()
        .split_terminator('\n')
        .map(|line| line.split_once(" = ").unwrap())
        .map(|(current_node, left_right)| {
            (
                current_node,
                left_right
                    .trim_matches(&['(', ')'][..])
                    .split_once(", ")
                    .unwrap(),
            )
        })
        .collect()
}
enum Instruction {
    Left,
    Right,
}
impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(()),
        }
    }
}
pub fn part1(input: &str) -> String {
    let (pattern, mappings) = input.split_once("\n\n").unwrap();
    let mappings = parse_mappings(mappings);
    let instructions = pattern.chars().map(|c| c.try_into().unwrap());

    #[derive(Copy, Clone)]
    enum NodeState<NodeType> {
        SearchingForEndState(NodeType),
        Ended(usize),
    }
    format!(
        "{}",
        instructions
            .cycle()
            .enumerate()
            .scan(
                NodeState::SearchingForEndState("AAA"),
                |accum, (count, instruction)| {
                    if let NodeState::SearchingForEndState(node) = accum {
                        match mappings
                            .get(node)
                            .map(|(left, right)| *match instruction {
                                Instruction::Right => right,
                                Instruction::Left => left,
                            })
                            .unwrap()
                        {
                            "ZZZ" => *accum = NodeState::Ended(count),
                            next_node => *node = next_node,
                        }
                    }
                    // needed to derive Copy to do this, so not sure we're actually saving
                    // on memory use compared to an externally-allocated mutable accumulator...
                    // (at least it's still on the stack and not the heap)
                    Some(*accum)
                },
            )
            .find_map(|state| match state {
                NodeState::Ended(count) => Some(count),
                NodeState::SearchingForEndState(_) => None,
            })
            .unwrap()
    )
}

pub fn part2(input: &str) -> String {
    let (pattern, mappings) = input.split_once("\n\n").unwrap();
    let mappings = parse_mappings(mappings);
    let instructions = pattern.chars().map(|c| c.try_into().unwrap());
    #[derive(Copy, Clone)]
    enum NodeState {
        EndsWithZAfter(usize),
        // nodes traversed until now
        SearchingForEndState(usize),
    }
    let mut current_nodes = mappings
        .iter()
        .filter_map(|(node, (_, _))| {
            node.ends_with('A')
                .then_some((*node, NodeState::SearchingForEndState(0)))
        })
        .collect::<Vec<_>>();
    for instruction in instructions.cycle() {
        if current_nodes
            .iter()
            .all(|(_, state)| matches!(state, NodeState::EndsWithZAfter(_)))
        {
            break;
        }
        for (node_def, state) in current_nodes.iter_mut() {
            if let NodeState::SearchingForEndState(traversed) = state {
                let (left, right) = mappings.get(node_def).unwrap();
                *node_def = match instruction {
                    Instruction::Right => right,
                    Instruction::Left => left,
                };
                *traversed += 1;
                if node_def.ends_with('Z') {
                    *state = NodeState::EndsWithZAfter(*traversed)
                }
            }
        }
    }
    format!(
        "{}",
        current_nodes
            .iter()
            .filter_map(|(_, state)| {
                if let NodeState::EndsWithZAfter(traversed) = state {
                    Some(traversed)
                } else {
                    None
                }
            })
            .fold(1, |accum, next| lcm(accum, *next))
    )
}
/// thank you <https://rustp.org/number-theory/lcm/>
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

/// thank you <https://rustp.org/number-theory/lcm/>
fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[test]
fn part2_on_sample() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    assert_eq!(part2(input), "6");
}
