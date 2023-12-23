// https://adventofcode.com/2023/day/10
#![feature(test)]
extern crate test;

#[derive(PartialEq)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    get_loop().len() as u32 / 2
}

#[inline(always)]
fn part2() -> u32 {
    let r#loop = get_loop();
    let area = get_area(&r#loop);

    get_num_enclosed_tiles(area, r#loop.len())
}

#[inline(always)]
fn get_loop() -> Vec<(usize, usize)> {
    let input = include_bytes!("../input");

    let width = input
        .iter()
        .position(|&character| character == b'\n')
        .unwrap();
    let height = input.len() / (width + 1) + 1;

    let mut r#loop = Vec::new();

    // Find S and then the loop
    'outer: for y in 0..height {
        for x in 0..width {
            let tile = get_tile(y, x, input, width);

            if tile != b'S' {
                continue;
            }

            let mut c_y = y;
            let mut c_x = x;
            let mut last_direction = Direction::None;

            r#loop.push((c_y, c_x));

            loop {
                let tile = get_tile(c_y, c_x, input, width);
                let mut found_next_tile = false;

                if last_direction != Direction::South
                    && (tile == b'|' || tile == b'L' || tile == b'J' || tile == b'S')
                    && c_y != 0
                {
                    // Try to follow the north tile
                    let top_tile = get_tile(c_y - 1, c_x, input, width);

                    if top_tile == b'|' {
                        r#loop.push((c_y - 1, c_x));
                        c_y -= 2;
                        last_direction = Direction::North;
                        found_next_tile = true;
                    } else if top_tile == b'7' {
                        r#loop.push((c_y - 1, c_x));
                        c_y -= 1;
                        c_x -= 1;
                        last_direction = Direction::West;
                        found_next_tile = true;
                    } else if top_tile == b'F' {
                        r#loop.push((c_y - 1, c_x));
                        c_y -= 1;
                        c_x += 1;
                        last_direction = Direction::East;
                        found_next_tile = true;
                    }
                }

                if !found_next_tile
                    && last_direction != Direction::West
                    && (tile == b'-' || tile == b'L' || tile == b'F' || tile == b'S')
                    && c_x != width - 1
                {
                    // Try to follow the east tile
                    let right_tile = get_tile(c_y, c_x + 1, input, width);

                    if right_tile == b'-' {
                        r#loop.push((c_y, c_x + 1));
                        c_x += 2;
                        last_direction = Direction::East;
                        found_next_tile = true;
                    } else if right_tile == b'J' {
                        r#loop.push((c_y, c_x + 1));
                        c_x += 1;
                        c_y -= 1;
                        last_direction = Direction::North;
                        found_next_tile = true;
                    } else if right_tile == b'7' {
                        r#loop.push((c_y, c_x + 1));
                        c_x += 1;
                        c_y += 1;
                        last_direction = Direction::South;
                        found_next_tile = true;
                    }
                }

                if !found_next_tile
                    && last_direction != Direction::North
                    && (tile == b'|' || tile == b'7' || tile == b'F' || tile == b'S')
                    && c_y != height - 1
                {
                    // Try to follow the south tile
                    let bottom_tile = get_tile(c_y + 1, c_x, input, width);

                    if bottom_tile == b'|' {
                        r#loop.push((c_y + 1, c_x));
                        c_y += 2;
                        last_direction = Direction::South;
                        found_next_tile = true;
                    } else if bottom_tile == b'L' {
                        r#loop.push((c_y + 1, c_x));
                        c_y += 1;
                        c_x += 1;
                        last_direction = Direction::East;
                        found_next_tile = true;
                    } else if bottom_tile == b'J' {
                        r#loop.push((c_y + 1, c_x));
                        c_y += 1;
                        c_x -= 1;
                        last_direction = Direction::West;
                        found_next_tile = true;
                    }
                }

                if !found_next_tile
                    && last_direction != Direction::East
                    && (tile == b'-' || tile == b'J' || tile == b'7' || tile == b'S')
                    && c_x != 0
                {
                    // Try to follow the west tile
                    let left_tile = get_tile(c_y, c_x - 1, input, width);

                    if left_tile == b'-' {
                        r#loop.push((c_y, c_x - 1));
                        c_x -= 2;
                        last_direction = Direction::West;
                    } else if left_tile == b'L' {
                        r#loop.push((c_y, c_x - 1));
                        c_x -= 1;
                        c_y -= 1;
                        last_direction = Direction::North;
                    } else if left_tile == b'F' {
                        r#loop.push((c_y, c_x - 1));
                        c_x -= 1;
                        c_y += 1;
                        last_direction = Direction::South;
                    }
                }

                if c_y == y && c_x == x {
                    break 'outer;
                }

                r#loop.push((c_y, c_x));
            }
        }
    }

    r#loop
}

#[inline(always)]
fn get_tile(y: usize, x: usize, input: &[u8], width: usize) -> u8 {
    input[y * (width + 1) + x]
}

#[inline(always)]
fn get_area(r#loop: &[(usize, usize)]) -> u32 {
    // For reference: https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;

    for point_index in 0..r#loop.len() - 1 {
        let point_a = r#loop[point_index];
        let point_b = r#loop[point_index + 1];

        // (y0 + y1) * (x0 - x1)
        area += (point_a.0 + point_b.0) as isize * (point_a.1 as isize - point_b.1 as isize);
    }

    area as u32 / 2
}

#[inline(always)]
fn get_num_enclosed_tiles(area: u32, num_loop_points: usize) -> u32 {
    // For reference: https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = I + R/2 - 1
    // -> I = A - R/2 + 1
    // I = number of enclosed tiles, A = area, R = number of loop points
    area - num_loop_points as u32 / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6856);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 501);
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
