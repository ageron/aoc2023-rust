fn compute_elf_hash(step: &[u8]) -> u32 {
    step.iter().fold(0, |current_value, &c| {
        ((current_value + c as u32) * 17) % 256
    })
}

fn apply_step(boxes: &mut [Vec<(String, u32)>], step: &str) {
    let step_bytes = step.as_bytes();
    let last_index = step_bytes.len() - 1;
    if step_bytes[last_index] == b'-' {
        let label_bytes = &step_bytes[..last_index];
        let hash = compute_elf_hash(label_bytes);
        let label = std::str::from_utf8(label_bytes).unwrap();
        boxes[hash as usize].retain(|(existing_label, _)| existing_label != label);
    } else {
        let mut parts = step.split('=');
        let label = parts.next().unwrap();
        let hash = compute_elf_hash(label.as_bytes());
        let focal_length: u32 = parts.next().unwrap().parse().unwrap();
        let old_position = boxes[hash as usize]
            .iter()
            .position(|(existing_label, _)| existing_label == label);
        if let Some(old_position) = old_position {
            boxes[hash as usize][old_position] = (label.to_string(), focal_length);
        } else {
            boxes[hash as usize].push((label.to_string(), focal_length));
        }
    }
}

fn total_focusing_power(boxes: &[Vec<(String, u32)>]) -> u32 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot_number, &(_, focal_length))| {
                    focusing_power(box_number, slot_number, focal_length)
                })
                .sum::<u32>()
        })
        .sum()
}

fn focusing_power(box_number: usize, slot_number: usize, focal_length: u32) -> u32 {
    (box_number as u32 + 1) * (slot_number as u32 + 1) * focal_length
}

pub fn run(input: &str) {
    let sum_of_hashes: u32 = input
        .split(',')
        .map(|step| compute_elf_hash(step.as_bytes()))
        .sum();
    println!("{:?}", sum_of_hashes);

    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    input
        .split(',')
        .for_each(|step| apply_step(&mut boxes, step));
    let sum_of_focusing_powers: u32 = total_focusing_power(&boxes);
    println!("{:?}", sum_of_focusing_powers);
}
