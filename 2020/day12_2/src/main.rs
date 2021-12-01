use core::panic;
use std::{error::Error, f64::consts::PI, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for line in lines {
        let action = &line[0..1];
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            "N" => waypoint_y += value, // Move waypoint north
            "S" => waypoint_y -= value, // Move waypoint south
            "E" => waypoint_x += value, // Move waypoint east
            "W" => waypoint_x -= value, // Move waypoint west
            "L" => {
                // Rotate waypoint counter-clockwise around the ship
                let (new_waypoint_x, new_waypoint_y) =
                    rotate_2d_point(waypoint_x, waypoint_y, value as f64, false);

                waypoint_x = new_waypoint_x;
                waypoint_y = new_waypoint_y;
            }
            "R" => {
                // Rotate waypoint clockwise around the ship
                let (new_waypoint_x, new_waypoint_y) =
                    rotate_2d_point(waypoint_x, waypoint_y, value as f64, true);

                waypoint_x = new_waypoint_x;
                waypoint_y = new_waypoint_y;
            }
            "F" => {
                // Move ship towards the waypoint
                x += value * waypoint_x;
                y += value * waypoint_y;
            }
            _ => panic!("Unhandled action! ({action})"),
        }
    }

    // Print manhattan distance
    println!("{} + {} = {}", x.abs(), y.abs(), x.abs() + y.abs());

    Ok(())
}

fn rotate_2d_point(x: i32, y: i32, degrees: f64, clockwise: bool) -> (i32, i32) {
    let radians = degrees * (PI / 180f64);
    let s = radians.sin();
    let c = radians.cos();

    let x_new;
    let y_new;

    // Rotate around center (0,0) (which is our ship, because the position of the waypoint is relative to it)
    if clockwise {
        x_new = x as f64 * c + y as f64 * s;
        y_new = -x as f64 * s + y as f64 * c;
    } else {
        x_new = x as f64 * c - y as f64 * s;
        y_new = x as f64 * s + y as f64 * c;
    }

    (x_new.round() as i32, y_new.round() as i32)
}
