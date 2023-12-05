// https://adventofcode.com/2023/day/5
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u64 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut values = Vec::new();

    // Skip "seeds: "
    input_index += 7;

    // Iterate over the seeds
    loop {
        let seed = read_value(input, &mut input_index);

        values.push(seed);

        if input[input_index] == b'\n' {
            break;
        }

        // Skip " "
        input_index += 1;
    }

    // Skip "\n\n"
    input_index += 2;

    // Iterate over the maps
    loop {
        // Iterate over the maps
        // Skip "<char>:" (minimum)
        input_index += 2;

        while input[input_index] != b'\n' {
            input_index += 1;
        }

        // Skip "\n"
        input_index += 1;

        let mut map = Vec::new();

        // Iterate over the mapping ranges
        loop {
            let mapping_destination_from = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            let mapping_from = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            let length = read_value(input, &mut input_index);

            map.push((
                mapping_from,
                mapping_from + length,
                mapping_destination_from,
            ));

            if input_index == input.len() {
                // EOF (end of maps)
                break;
            } else {
                // EOL (end of mapping)
                // Skip "\n"
                input_index += 1;

                if input[input_index] == b'\n' {
                    // Second EOL (end of map)
                    // Skip '\n'
                    input_index += 1;
                    break;
                }
            }
        }

        map.sort();

        for value in &mut values {
            let mut map_index = map.len() - 1;

            for (index, &(mapping_from, mapping_to, _)) in map.iter().enumerate() {
                // Try to find a mapping range which contains the value
                if *value >= mapping_from && *value <= mapping_to {
                    map_index = index;
                    break;
                }
            }

            let (mapping_from, mapping_to, mapping_destination_from) = map[map_index];

            // The value is contained in the mapping range.
            // |--------------*-------|
            // ^ mapping from ^ value ^ mapping to
            // - Carry the mapping range before the overlap (*) over (with the destination as an offset)
            // - Continue with the next value (because there cannot be more overlaps)
            if *value >= mapping_from && *value < mapping_to {
                *value = mapping_destination_from + *value - mapping_from;
            }
        }

        if input_index == input.len() {
            break;
        }
    }

    let mut min_location_number = u64::MAX;

    for value in values {
        min_location_number = min_location_number.min(value);
    }

    min_location_number
}

#[inline(always)]
fn part2() -> u64 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut ranges = Vec::new();

    // Skip "seeds: "
    input_index += 7;

    // Iterate over the seeds
    loop {
        let seed_from = read_value(input, &mut input_index);

        // Skip " "
        input_index += 1;

        let length = read_value(input, &mut input_index);

        ranges.push((seed_from, seed_from + length));

        if input[input_index] == b'\n' {
            break;
        }

        // Skip " "
        input_index += 1;
    }

    // Skip "\n\n"
    input_index += 2;

    // Iterate over the maps
    loop {
        // Skip "<char>:" (minimum)
        input_index += 2;

        while input[input_index] != b'\n' {
            input_index += 1;
        }

        // Skip "\n"
        input_index += 1;

        let mut map = Vec::new();

        // Iterate over the mapping ranges
        loop {
            let mapping_destination_from = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            let mapping_from = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            let length = read_value(input, &mut input_index);

            map.push((
                mapping_from,
                mapping_from + length,
                mapping_destination_from,
            ));

            if input_index == input.len() {
                // EOF (end of maps)
                break;
            } else {
                // EOL (end of mapping)
                // Skip "\n"
                input_index += 1;

                if input[input_index] == b'\n' {
                    // Second EOL (end of map)
                    // Skip '\n'
                    input_index += 1;
                    break;
                }
            }
        }

        map.sort();

        let mut new_ranges = Vec::new();

        for range in ranges {
            let (mut from, to) = range;

            let mut map_start_index = map.len() - 1;

            for (index, &(mapping_from, mapping_to, _)) in map.iter().enumerate() {
                // Try to find a mapping range which contains the start of our range
                if from >= mapping_from && from <= mapping_to {
                    map_start_index = index;
                    break;
                }
            }

            for &mapping in map.iter().skip(map_start_index) {
                let (mapping_from, mapping_to, mapping_destination_from) = mapping;

                // The range is behind the map range.
                // - Don't carry the range (#) over
                // - Continue with the next mapping
                // |----|          |####|
                // ^ mapping range ^ range
                if from >= mapping_to {
                    continue;
                }

                // The range is before of the map range.
                // - Carry the range (#) over
                // - Continue with the next range
                // |####|  |----|
                // ^ range ^ mapping range
                if to <= mapping_from {
                    new_ranges.push(range);
                    break;
                }

                // The range and map range are overlapping (from < mapping_to && to > mapping_from).
                // There are 4 cases to handle:
                // 1. The range starts before the mapping range.
                // from < mapping_from && to <= mapping_to
                // |##############|**************|------------|
                // ^ from         ^ mapping from ^ to         ^ mapping to
                // - Carry the range (#) before the overlap (*) over
                // - Carry the overlap (*) over (with the destination as an offset)
                // - Continue with the next range (because there cannot be more overlaps)
                //
                // 2. The mapping range is fully contained in the range.
                // from < mapping_from && to > mapping_to
                // |##############|**************|############|
                // ^ from         ^ mapping from ^ mapping to ^ to
                // - Carry the range (#) before the overlap (*) over
                // - Carry the overlap (*) over (with the destination as an offset)
                // - Continue with the next mapping (because there could be more overlaps)
                //
                // 3. The range is fully contained in the mapping range.
                // from >= mapping_from && to <= mapping_to
                // |--------------|**************|------------|
                // ^ mapping from ^ from         ^ to         ^ mapping to
                // - Carry the mapping range (-) before the overlap (*) combined with the overlap (*) over (with the mapping range before the overlap (-) added to the destination as an offset)
                // - Continue with the next range (because there cannot be more overlaps)
                //
                // 4. The range ends after the mapping range.
                // from >= mapping_from && to > mapping_to
                // |--------------|**************|############|
                // ^ mapping from ^ from         ^ mapping to ^ to
                // - Carry the mapping range (-) before the overlap (*) combined with the overlap (*) over (with the mapping range before the overlap (-) added to the destination as an offset)
                // - Continue with the next mapping (because there could be more overlaps)
                if from < mapping_from {
                    new_ranges.push((from, mapping_from));
                    from = mapping_from;
                }

                let start = mapping_destination_from + from - mapping_from;

                if to <= mapping_to {
                    new_ranges.push((start, mapping_destination_from + to - mapping_from));
                    break;
                }

                new_ranges.push((start, mapping_destination_from + mapping_to - mapping_from));

                // Continue with the next mapping
                from = mapping_to;
            }
        }

        ranges = new_ranges;

        if input_index == input.len() {
            break;
        }
    }

    let mut min_location_number = u64::MAX;

    for (from, _) in ranges {
        min_location_number = min_location_number.min(from);
    }

    min_location_number
}

fn read_value(input: &[u8], input_index: &mut usize) -> u64 {
    let mut value = 0;

    while *input_index < input.len() && input[*input_index].is_ascii_digit() {
        value *= 10u64;
        value += (input[*input_index] - b'0') as u64;

        *input_index += 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 403695602);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 219529182);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| black_box(part1()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| black_box(part2()));
    }
}
