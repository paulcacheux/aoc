use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day18;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

#[derive(Debug, Clone)]
pub struct NodeList {
    nodes: Vec<Node>,
    main_index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PairVisitState {
    Begin,
    Middle,
    End,
}

struct VisitItem {
    depth: usize,
    index: usize,
    pair_state: PairVisitState,
}

impl NodeList {
    fn push_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn depth_first_visit<F: FnMut(&VisitItem) -> bool, const MID: bool, const POST: bool>(
        &self,
        mut visitor: F,
    ) {
        let mut open_list = vec![VisitItem {
            depth: 0,
            index: self.main_index,
            pair_state: PairVisitState::Begin,
        }];

        while let Some(item) = open_list.pop() {
            if !visitor(&item) {
                return;
            }

            match self.nodes[item.index] {
                Node::Lit(_) => {}
                Node::Pair(lhs, rhs) => {
                    if item.pair_state == PairVisitState::Begin {
                        if POST {
                            open_list.push(VisitItem {
                                depth: item.depth,
                                index: item.index,
                                pair_state: PairVisitState::End,
                            });
                        }
                        open_list.push(VisitItem {
                            depth: item.depth + 1,
                            index: rhs,
                            pair_state: PairVisitState::Begin,
                        });
                        if MID {
                            open_list.push(VisitItem {
                                depth: item.depth,
                                index: item.index,
                                pair_state: PairVisitState::Middle,
                            });
                        }
                        open_list.push(VisitItem {
                            depth: item.depth + 1,
                            index: lhs,
                            pair_state: PairVisitState::Begin,
                        });
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.depth_first_visit::<_, true, false>(|item| {
            match (&self.nodes[item.index], item.pair_state) {
                (Node::Lit(lit), _) => {
                    print!("{}", lit);
                }
                (Node::Pair(_, _), PairVisitState::Begin) => {
                    print!("[");
                }
                (Node::Pair(_, _), PairVisitState::Middle) => {
                    print!(",");
                }
                (Node::Pair(_, _), PairVisitState::End) => {
                    print!("]");
                }
            };
            true
        });
        println!()
    }

    fn explode(&mut self) -> bool {
        let mut left_lit = None;
        let mut exploding_pair = None;
        let mut right_lit = None;
        let mut skip_right = Vec::new();

        self.depth_first_visit::<_, false, false>(|item| match self.nodes[item.index] {
            Node::Lit(_) => {
                if exploding_pair.is_some() {
                    if !skip_right.contains(&item.index) {
                        right_lit = Some(item.index);
                        false
                    } else {
                        true
                    }
                } else {
                    left_lit = Some(item.index);
                    true
                }
            }
            Node::Pair(lhs, rhs)
                if item.pair_state == PairVisitState::Begin
                    && item.depth >= 4
                    && exploding_pair.is_none() =>
            {
                if let (Node::Lit(l), Node::Lit(r)) = (&self.nodes[lhs], &self.nodes[rhs]) {
                    exploding_pair = Some((item.index, *l, *r));
                    skip_right = vec![lhs, rhs];
                }
                true
            }
            _ => true,
        });

        if let Some((explode_index, left_value, right_value)) = exploding_pair {
            if let Some(index) = left_lit {
                self.nodes[index] = self.nodes[index].add(left_value);
            }

            if let Some(index) = right_lit {
                self.nodes[index] = self.nodes[index].add(right_value);
            }

            self.nodes[explode_index] = Node::Lit(0);
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        let mut split_index = None;
        self.depth_first_visit::<_, false, false>(|item| match self.nodes[item.index] {
            Node::Lit(value) if value >= 10 => {
                split_index = Some((item.index, value));
                false
            }
            _ => true,
        });

        if let Some((index, value)) = split_index {
            let left_value = value / 2;
            let right_value = value - left_value;

            let lhs_index = self.push_node(Node::Lit(left_value));
            let rhs_index = self.push_node(Node::Lit(right_value));

            self.nodes[index] = Node::Pair(lhs_index, rhs_index);
            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        loop {
            while self.explode() {}

            if self.split() {
                continue;
            }
            break;
        }
    }

    fn add(&mut self, other: &NodeList) {
        let right_start_index = self.nodes.len();
        self.nodes
            .extend(other.nodes.iter().copied().map(|node| match node {
                Node::Lit(_) => node,
                Node::Pair(left, right) => {
                    Node::Pair(right_start_index + left, right_start_index + right)
                }
            }));

        let res_index = self.push_node(Node::Pair(
            self.main_index,
            right_start_index + other.main_index,
        ));
        self.main_index = res_index;
    }

    fn magnitude(&self) -> u32 {
        let mut stack = Vec::new();
        self.depth_first_visit::<_, false, true>(|item| {
            match self.nodes[item.index] {
                Node::Lit(value) => {
                    stack.push(value);
                }
                Node::Pair(_, _) if item.pair_state == PairVisitState::End => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(3 * left + 2 * right);
                }
                _ => {}
            };
            true
        });
        assert_eq!(stack.len(), 1);
        stack.pop().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Lit(u32),
    Pair(usize, usize),
}

impl Node {
    fn add(self, other_value: u32) -> Node {
        match self {
            Node::Lit(v) => Node::Lit(v + other_value),
            Node::Pair(_, _) => panic!("add pair"),
        }
    }
}

fn parse_node(line: &str) -> NodeList {
    let mut node_list = NodeList {
        nodes: Vec::new(),
        main_index: 0,
    };
    let mut stack = Vec::new();
    for c in line.trim().chars() {
        match c {
            '[' => {}
            ']' => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                let node = Node::Pair(lhs, rhs);
                stack.push(node_list.push_node(node));
            }
            ',' => {}
            d if d.is_ascii_digit() => {
                let node = Node::Lit(d.to_digit(10).unwrap());
                stack.push(node_list.push_node(node));
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(stack.len(), 1);
    let main_index = stack.pop().unwrap();
    node_list.main_index = main_index;
    node_list
}

impl ParseInput<Day18> for Aoc2021 {
    type Parsed = Vec<NodeList>;

    fn parse_input(input: &str) -> Vec<NodeList> {
        input.lines().map(parse_node).collect()
    }
}

impl Solution<Day18> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<NodeList>) -> u32 {
        let mut current = input[0].clone();
        for next in &input[1..] {
            current.add(next);
            current.reduce();
        }
        current.magnitude()
    }

    fn part2(input: &Vec<NodeList>) -> u32 {
        let mut max = None;

        for (i, left) in input.iter().enumerate() {
            for (j, right) in input.iter().enumerate() {
                if i == j {
                    continue;
                }

                let mut left = left.clone();
                left.add(right);
                left.reduce();
                let mag = left.magnitude();

                if let Some(m) = max {
                    if m < mag {
                        max = Some(mag);
                    }
                } else {
                    max = Some(mag);
                }
            }
        }

        max.unwrap()
    }
}
