use std::io;

fn main() -> io::Result<()> {
    // 7 segments = 'a' to 'g'
    // 10 digits = 0 to 9, represented by a fixed and known number of segments

    // digits with 2 segments
    // 1 =>                top right,                   bottom right

    // digits with 3 segments
    // 7 => top,           top right,                   bottom right

    // digits with 4 segments
    // 4 =>      top left, top right, mid,              bottom right

    // digits with 5 segments
    // 2 => top,           top right, mid, bottom left,               bottom =>           top right,      bottom left
    // 3 => top,           top right, mid,              bottom right, bottom =>           top right,                   bottom right
    // 5 => top, top left,            mid,              bottom right, bottom => top left,                              bottom right

    // digits with 6 segments
    // 0 => top, top left, top right,      bottom left, bottom right, bottom =>           top right,      bottom left
    // 6 => top, top left,            mid, bottom left, bottom right, bottom =>                      mid, bottom left
    // 9 => top, top left, top right, mid,              bottom right, bottom =>           top right, mid

    // digits with 7 segments
    // 8 => top, top left, top right, mid, bottom left, bottom right, bottom

    // Get the input
    let input = include_str!("../input");

    // Split at " | " to get the unique signal patterns (10 digits) and digital output values (4 digits)
    let all_digits = input
        .lines()
        .map(|line| {
            // Replace " | " with " ", so that we can get the digits more easily
            let sanitized_line = line.replace(" | ", " ");

            // Split at "," and get the digits
            let digits = sanitized_line
                .split(' ')
                .map(|value| value.to_string())
                .collect::<Vec<_>>();

            // Map the digits into their segments and sort them in ascending order
            let digits_with_sorted_segments = digits
                .iter()
                .map(|digit| {
                    let mut segments = digit.chars().collect::<Vec<_>>();
                    segments.sort_unstable();

                    segments
                })
                .collect::<Vec<_>>();

            // Get the unique signal patterns (10 digits)
            let unique_signal_patterns = digits_with_sorted_segments[0..10].to_vec();

            // Get the digital output values (4 digits)
            let digital_output_values = digits_with_sorted_segments[10..14].to_vec();

            (unique_signal_patterns, digital_output_values)
        })
        .collect::<Vec<_>>();

    part1(&all_digits);
    part2(&all_digits);

    Ok(())
}

#[allow(clippy::type_complexity)]
fn part1(all_digits: &[(Vec<Vec<char>>, Vec<Vec<char>>)]) {
    let mut num_digits = 0;

    // Iterate over each digit
    for digit in all_digits {
        let digital_output_values = &digit.1;

        // Iterate over each digital output value
        for digital_output_value in digital_output_values {
            // Check (based on the number of segments) if the current digital output value is a digit we are looking for
            match digital_output_value.len() {
                // 2 segments => digit 1
                // 4 segments => digit 4
                // 3 segments => digit 7
                // 7 segments => digit 8
                2 | 4 | 3 | 7 => num_digits += 1,
                _ => (),
            }
        }
    }

    assert_eq!(num_digits, 476);

    println!("part1: {num_digits}");
}

#[allow(clippy::type_complexity)]
fn part2(all_digits: &[(Vec<Vec<char>>, Vec<Vec<char>>)]) {
    let mut total_output_value = 0;

    // Iterate over each digit
    for digit in all_digits {
        let mut digits: Vec<Vec<char>> = vec![Vec::new(); 10];
        let unique_signal_patterns = &digit.0;

        // Iterate over each unique signal pattern
        for unique_signal_pattern in unique_signal_patterns {
            // Get the known digits
            match unique_signal_pattern.len() {
                // 2 segments => digit 1
                2 => digits[1].clone_from(unique_signal_pattern),
                // 4 segments => digit 4
                4 => digits[4].clone_from(unique_signal_pattern),
                // 3 segments => digit 7
                3 => digits[7].clone_from(unique_signal_pattern),
                // 7 segments => digit 8
                7 => digits[8].clone_from(unique_signal_pattern),
                _ => (),
            }
        }

        // Get the unknown segments and digits

        // Get the top by (digit 7 - digit 1)
        let top = get_differences(&digits[7], &digits[1], 1)[0];

        // Find the digit 3 by (len 5 and contains digit 1)
        digits[3] = find_digit(unique_signal_patterns, &digits[1], 5);

        // Get the mid and bottom by (digit 3 - digit 7)
        let mid_and_bottom = get_differences(&digits[3], &digits[7], 2);

        // Get the top left and mid by (digit 4 - digit 1)
        let top_left_and_mid = get_differences(&digits[4], &digits[1], 2);

        // Get the mid by (mid_and_bottom ∩ top_left_and_mid)
        let mid = get_single_intersection(&mid_and_bottom, &top_left_and_mid);

        // Get bottom by (mid_and_bottom - mid)
        let bottom = get_differences(&mid_and_bottom, &[mid], 1)[0];

        // Get the top left by (top_left_and_mid - mid)
        let top_left = get_differences(&top_left_and_mid, &[mid], 1)[0];

        // Get the bottom left by (digit 8 - digit 4 - top - bottom)
        let bottom_left = get_differences(
            &digits[8],
            &[digits[4].clone(), vec![top, bottom]].concat(),
            1,
        )[0];

        // Find the digit 2 by (len 5 and contains bottom left)
        digits[2] = find_digit(unique_signal_patterns, &[bottom_left], 5);

        // Get the top right by (digit 2 - top - mid - bottom left - bottom)
        let top_right = get_differences(&digits[2], &[top, mid, bottom_left, bottom], 1)[0];

        // Find the digit 5 by (len 5 and contains top left)
        digits[5] = find_digit(unique_signal_patterns, &[top_left], 5);

        // Find the digit 0 by (len 6 and contains top left and bottom left)
        digits[0] = find_digit(unique_signal_patterns, &[top_right, bottom_left], 6);

        // Find the digit 6 by (len 6 and contains mid and bottom left)
        digits[6] = find_digit(unique_signal_patterns, &[mid, bottom_left], 6);

        // Find the digit 9 by (len 6 and contains top right and mid)
        digits[9] = find_digit(unique_signal_patterns, &[top_right, mid], 6);

        let digital_output_values = &digit.1;

        let mut output_value = 0;

        for (output_index, digital_output_value) in digital_output_values.iter().enumerate() {
            let mut found_digit = 0u32;

            for (index, digit) in digits.iter().enumerate() {
                let digit_str: String = digit.iter().collect();
                let digital_output_value_str: String = digital_output_value.iter().collect();

                if digit_str == digital_output_value_str {
                    found_digit = index as u32;
                    break;
                }
            }

            output_value += found_digit
                * 10u32.pow(digital_output_values.len() as u32 - output_index as u32 - 1);
        }

        total_output_value += output_value;
    }

    assert_eq!(total_output_value, 1011823);

    println!("part2: {total_output_value}");
}

fn get_differences(
    segments_a: &[char],
    segments_b: &[char],
    num_required_segments: usize,
) -> Vec<char> {
    // Get all segments that are in a but not in b
    let differences = segments_a
        .iter()
        .filter(|segment| !segments_b.contains(segment))
        .map(|segment| segment.to_owned())
        .collect::<Vec<_>>();

    assert_eq!(differences.len(), num_required_segments);

    differences
}

fn get_single_intersection(segments_a: &[char], segments_b: &[char]) -> char {
    // Get all segments that are in a and in b
    let intersection = segments_a
        .iter()
        .filter(|segment| segments_b.contains(*segment))
        .collect::<Vec<_>>();

    assert_eq!(intersection.len(), 1);

    *intersection[0]
}

fn find_digit(
    digits: &[Vec<char>],
    required_segments: &[char],
    num_required_segments: usize,
) -> Vec<char> {
    let matches = digits
        .iter()
        .filter(|segments| {
            segments.len() == num_required_segments
                && required_segments
                    .iter()
                    .all(|segment| segments.contains(segment))
        })
        .collect::<Vec<_>>();

    assert_eq!(matches.len(), 1);

    matches[0].to_vec()
}
