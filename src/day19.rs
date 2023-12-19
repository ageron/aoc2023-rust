use crate::utils::parse_ints;
use std::collections::HashMap;

fn parse_part_rating(line: &str) -> [i32; 4] {
    let ratings = parse_ints(line, false);
    [ratings[0], ratings[1], ratings[2], ratings[3]] // x m a s
}

#[derive(Debug, Clone)]
struct Rule {
    var_index: usize,
    is_lower: bool,
    value: i32,
    action: String,
}

impl Rule {
    const UNCONDITIONAL: usize = 4;

    fn new(rule: &str) -> Self {
        if rule.contains(':') {
            let mut parts = rule.split(':');
            let condition = parts.next().unwrap();
            let var_name = condition.as_bytes()[0];
            let var_index = b"xmas".iter().position(|&c| c == var_name).unwrap();
            let is_lower = condition.as_bytes()[1] == b'<';
            let value: i32 = std::str::from_utf8(&condition.as_bytes()[2..])
                .unwrap()
                .parse()
                .unwrap();
            let action = parts.next().unwrap().to_string();
            Self {
                var_index,
                is_lower,
                value,
                action,
            }
        } else {
            let action = rule.as_bytes();
            let action = std::str::from_utf8(&action[..action.len() - 1])
                .unwrap()
                .to_string();
            Self {
                var_index: Rule::UNCONDITIONAL,
                is_lower: false,
                value: 0,
                action,
            }
        }
    }

    fn restrict_variable_range(
        &self,
        allowed_values: &mut [[i32; 2]; 4],
        is_reversed: bool,
    ) -> bool {
        if self.var_index == Rule::UNCONDITIONAL {
            return !is_reversed;
        }
        let is_lower = self.is_lower ^ is_reversed;
        let value = self.value
            + if is_reversed {
                if is_lower {
                    1
                } else {
                    -1
                }
            } else {
                0
            };
        if is_lower {
            allowed_values[self.var_index][1] = allowed_values[self.var_index][1].min(value - 1);
        } else {
            allowed_values[self.var_index][0] = allowed_values[self.var_index][0].max(value + 1);
        }
        allowed_values[self.var_index][1] - allowed_values[self.var_index][0] >= 0
    }

    fn count_combinations(allowed_values: &[[i32; 2]; 4]) -> u64 {
        allowed_values
            .iter()
            .map(|range| (range[1] - range[0] + 1).max(0) as u64)
            .product()
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: &str) -> Self {
        let mut parts = line.split('{');
        let name = parts.next().unwrap().to_string();
        let rest = parts.next().unwrap();
        let rules = rest.split(',').map(Rule::new).collect();
        Self { name, rules }
    }

    fn get_action(&self, part_rating: [i32; 4]) -> String {
        for rule in &self.rules {
            if rule.var_index == Rule::UNCONDITIONAL {
                return rule.action.to_string();
            }
            let value = part_rating[rule.var_index];
            if rule.is_lower {
                if value < rule.value {
                    return rule.action.to_string();
                }
            } else if value > rule.value {
                return rule.action.to_string();
            }
        }
        unreachable!();
    }

    fn is_accepted(workflows: &HashMap<String, Workflow>, part_rating: [i32; 4]) -> bool {
        let mut workflow_name = "in".to_string();
        loop {
            let workflow = workflows.get(&workflow_name).unwrap();
            let action = workflow.get_action(part_rating);
            match action.as_str() {
                "A" => {
                    return true;
                }
                "R" => {
                    return false;
                }
                _ => {
                    workflow_name = action;
                }
            }
        }
    }

    fn count_combinations_that_match_rule(
        &self,
        workflows: &HashMap<String, Workflow>,
        rule_index: usize,
        allowed_values: &mut [[i32; 2]; 4],
    ) -> u64 {
        if !self.rules[rule_index].restrict_variable_range(allowed_values, false) {
            return 0;
        }
        for rule in self.rules[0..rule_index].iter().rev() {
            if !rule.restrict_variable_range(allowed_values, true) {
                return 0;
            }
        }
        if self.name == "in" {
            return Rule::count_combinations(allowed_values);
        }
        workflows
            .iter()
            .map(|(_, workflow)| {
                workflow
                    .rules
                    .iter()
                    .enumerate()
                    .filter(|&(_, rule)| rule.action == self.name)
                    .map(|(rule_index, _)| {
                        workflow.count_combinations_that_match_rule(
                            workflows,
                            rule_index,
                            &mut allowed_values.clone(),
                        )
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    fn count_accepted_combinations(workflows: &HashMap<String, Workflow>) -> u64 {
        workflows
            .iter()
            .map(|(_, workflow)| {
                workflow
                    .rules
                    .iter()
                    .enumerate()
                    .filter(|(_, rule)| rule.action == "A")
                    .map(|(rule_index, _)| {
                        workflow.count_combinations_that_match_rule(
                            workflows,
                            rule_index,
                            &mut [[1, 4000]; 4],
                        )
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

pub fn run(input: &str) {
    let mut parts = input.split("\n\n");
    let workflows: HashMap<String, Workflow> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let workflow = Workflow::new(line);
            (workflow.name.clone(), workflow)
        })
        .collect();
    let part_ratings: Vec<_> = parts
        .next()
        .unwrap()
        .lines()
        .map(parse_part_rating)
        .collect();

    let total_accepted_value: i32 = part_ratings
        .iter()
        .filter(|&&part_rating| Workflow::is_accepted(&workflows, part_rating))
        .map(|part_rating| part_rating.iter().sum::<i32>())
        .sum();
    println!("{}", total_accepted_value);

    let num_accepted_combinations = Workflow::count_accepted_combinations(&workflows);
    println!("{}", num_accepted_combinations);
}
