use std::{collections::VecDeque, fs, io};

fn main() -> io::Result<()> {
    // Read the input
    let input = fs::read_to_string("input")?;

    // Split at "," to get the fish timers
    let mut fish_timers = input
        .split(',')
        .map(|fish_timer_str| fish_timer_str.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    part1(&mut fish_timers.clone());
    part2(&mut fish_timers);

    Ok(())
}

fn part1(fish_timers: &mut Vec<u8>) {
    // Simulate 80 days
    for _ in 0..80 {
        let mut num_new_fishes = 0;

        // Iterate over each fish timer
        for fish_timer in fish_timers.iter_mut() {
            if *fish_timer == 0 {
                // Fish is ready to create a new fish
                // Reset its timer
                *fish_timer = 6;

                // Increase the number of new fish
                num_new_fishes += 1;
            } else {
                // Fish is not ready to create a new fish yet
                *fish_timer -= 1;
            }
        }

        // Add the new fishes
        for _ in 0..num_new_fishes {
            fish_timers.push(8);
        }
    }

    let num_fish_timers = fish_timers.len();

    println!("part1: {}", num_fish_timers);
}

fn part2(fish_timers: &mut Vec<u8>) {
    // Create a fish timer queue for fish timers with values from 0 to 8
    let mut fish_timer_queue = VecDeque::new();
    fish_timer_queue.extend(&[0; 9]);

    // Iterate over each fish timer
    for fish_timers in fish_timers {
        // Increase the number of fish timers with particular value
        fish_timer_queue[*fish_timers as usize] += 1;
    }

    // Simulate 256 days
    for _ in 0..256 {
        // Fishes at the front are ready to create new fishes
        let current_fish_timer = fish_timer_queue.pop_front().unwrap();

        // Move the new fishes to the back (index: 8)
        // We have to wait 8 days until they are ready to create new fishes themselves
        fish_timer_queue.push_back(current_fish_timer);

        // Remember the fishes which just created new ones
        fish_timer_queue[6] += current_fish_timer;
    }

    let num_fish_timers = fish_timer_queue.iter().sum::<u64>();

    println!("part2: {}", num_fish_timers);
}
