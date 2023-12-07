use crate::utils::parse_ints;
use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
struct ElfMap {
    source_range: Range<i64>,
    offset: i64,
}

fn get_seed_destination(maps: &[ElfMap], source: i64) -> i64 {
    // For part 1, each number gets mapped to a single number
    for map in maps {
        if map.source_range.contains(&source) {
            return source + map.offset;
        }
    }
    source
}

fn get_target_ranges(source_ranges: Vec<Range<i64>>, maps: &[ElfMap]) -> Vec<Range<i64>> {
    // For part 2, each list of ranges gets mapped to another list of ranges
    source_ranges
        .iter()
        .flat_map(|source_range| {
            let overlapping: Vec<&ElfMap> = maps
                .iter()
                .filter(|&map| {
                    map.source_range.start < source_range.end
                        && map.source_range.end > source_range.start
                })
                .collect();
            overlapping
                .iter()
                .filter_map(|&map| {
                    let start = map.source_range.start.max(source_range.start) + map.offset;
                    let end = map.source_range.end.min(source_range.end) + map.offset;
                    if end > start {
                        Some(start..end)
                    } else {
                        None
                    }
                })
                .chain(overlapping.windows(2).filter_map(|map_pair| {
                    // these are the spaces between the ranges, with offset 0
                    let start = map_pair[0].source_range.end.max(source_range.start);
                    let end = map_pair[1].source_range.start.min(source_range.end);
                    if end > start {
                        Some(start..end)
                    } else {
                        None
                    }
                }))
                .collect::<Vec<Range<i64>>>()
        })
        .collect()
}

pub fn run(input: &str) {
    let mut parts = input.split("\n\n");
    let seeds: Vec<i64> = parse_ints(parts.next().unwrap(), false);
    let all_maps: Vec<Vec<ElfMap>> = parts
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|line| {
                    let map_nums: Vec<i64> = parse_ints(line, false);
                    ElfMap {
                        source_range: map_nums[1]..(map_nums[1] + map_nums[2]),
                        offset: map_nums[0] - map_nums[1],
                    }
                })
                .sorted_by(|a, b| a.source_range.start.cmp(&b.source_range.start))
                .collect()
        })
        .collect();
    let closest_location = seeds
        .iter()
        .map(|&seed| {
            all_maps
                .iter()
                .fold(seed, |seed, maps| get_seed_destination(maps, seed))
        })
        .min()
        .unwrap();
    println!("{}", closest_location);

    let seed_ranges = seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(&start, &length)| start..(start + length))
        .collect();

    let closest_location = all_maps
        .iter()
        .fold(seed_ranges, |source_ranges, maps| {
            get_target_ranges(source_ranges, maps)
        })
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap();
    println!("{}", closest_location);
}
