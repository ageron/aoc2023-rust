use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

fn parse_line(line: &str) -> (&str, (Module, Vec<&str>)) {
    let mut parts = line.split(" -> ");
    let prefix = parts.next().unwrap();
    let module = match prefix.chars().next().unwrap() {
        '%' => Module::FlipFlop(true),
        '&' => Module::Conjunction(HashMap::new()),
        'b' => Module::Broadcaster,
        _ => unreachable!(),
    };
    let destinations: Vec<_> = parts.next().unwrap().split(", ").collect();
    let name = if module == Module::Broadcaster {
        prefix
    } else {
        std::str::from_utf8(&prefix.as_bytes()[1..]).unwrap()
    };
    (name, (module, destinations))
}

fn push_button(
    module_config: &mut HashMap<&str, (Module, Vec<&str>)>,
    index: u64,
    first_low_pulse: &mut HashMap<String, u64>,
) -> (u32, u32) {
    let mut pulses = VecDeque::from([("button", "broadcaster", true)]);
    let mut num_low = 0;
    let mut num_high = 0;
    while !pulses.is_empty() {
        let (from_module, module_name, is_low_pulse) = pulses.pop_front().unwrap();
        if is_low_pulse {
            num_low += 1;
            first_low_pulse
                .entry(module_name.to_string())
                .or_insert(index);
        } else {
            num_high += 1;
        }
        let module_entry = module_config.get_mut(module_name);
        if let Some(module_entry) = module_entry {
            let (ref mut module, destinations) = module_entry;
            match module {
                Module::FlipFlop(ref mut is_low_state) => {
                    if is_low_pulse {
                        *is_low_state = !*is_low_state;
                        destinations.iter().for_each(|&destination_name| {
                            pulses.push_back((module_name, destination_name, *is_low_state))
                        });
                    }
                }
                Module::Conjunction(ref mut state) => {
                    state.insert(from_module.to_string(), is_low_pulse);
                    let is_low_out = !state.values().any(|&is_low_pulse| is_low_pulse);
                    destinations.iter().for_each(|&destination_name| {
                        pulses.push_back((module_name, destination_name, is_low_out))
                    });
                }
                Module::Broadcaster => {
                    destinations.iter().for_each(|&destination_name| {
                        pulses.push_back((module_name, destination_name, is_low_pulse))
                    });
                }
            }
        }
    }
    (num_low, num_high)
}

pub fn run(input: &str) {
    let mut module_inputs = HashMap::new();
    let mut module_config: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, (module, destinations)) = parse_line(line);
            destinations.iter().for_each(|&destination| {
                module_inputs
                    .entry(destination)
                    .and_modify(|set: &mut HashSet<_>| {
                        set.insert(name);
                    })
                    .or_insert(HashSet::from([name]));
            });
            (name, (module, destinations))
        })
        .collect();
    for (name, (module, _)) in module_config.iter_mut() {
        if let Module::Conjunction(ref mut state) = module {
            for &input_name in module_inputs.get(name).unwrap() {
                state.insert(input_name.to_string(), true);
            }
        }
    }
    let mut first_low_pulse = HashMap::new();
    let (total_low, total_high) = (0..1000)
        .map(|index| push_button(&mut module_config, index, &mut first_low_pulse))
        .reduce(|(low1, high1), (low2, high2)| (low1 + low2, high1 + high2))
        .unwrap();
    println!("{}", total_low * total_high);

    for index in 1000.. {
        push_button(&mut module_config, index, &mut first_low_pulse);
        if first_low_pulse.len() >= module_config.len() {
            break;
        }
    }
    let rx_inputs = module_inputs.get("rx").unwrap();
    let &rx_input_name = rx_inputs.iter().next().unwrap();
    let num_pushes_for_rx: u64 = module_inputs
        .get(rx_input_name)
        .unwrap()
        .iter()
        .map(|&module_name| first_low_pulse.get(module_name).unwrap() + 1)
        .product();
    println!("{}", num_pushes_for_rx);
    println!("{:?}", first_low_pulse);
}
