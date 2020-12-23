use std::str::FromStr;
use crate::day12::Command::{Forward, North, East, South, West, Left, Right};

const TEST_DATA: &str = "F10
N3
F7
R90
F11";

#[derive(Debug, Copy, Clone)]
enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64)
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_iter = s.chars();

        let first_char = char_iter.nth(0).unwrap();
        let n: i64 = char_iter.collect::<String>().parse().unwrap();

        match first_char {
            'N' => Ok(North(n)),
            'E' => Ok(East(n)),
            'S' => Ok(South(n)),
            'W' => Ok(West(n)),
            'L' => Ok(Left(n)),
            'R' => Ok(Right(n)),
            'F' => Ok(Forward(n)),
            _ => Err("Invalid first character".to_string())
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    heading: Command,
    waypoint_x: i64,
    waypoint_y: i64
}

impl Ship {
    fn new() -> Ship {
        Ship{
            x: 0,
            y: 0,
            heading: Command::East(0),
            waypoint_x: 10,
            waypoint_y: 1
        }
    }

    fn turn_cw(&mut self) {
        match self.heading {
            Command::North(_) => {
                self.heading = Command::East(0);
            }
            Command::South(_) => {
                self.heading = Command::West(0);
            }
            Command::East(_) => {
                self.heading = Command::South(0);
            }
            Command::West(_) => {
                self.heading = Command::North(0);
            }
            _ => {}
        }
    }

    fn turn_ccw(&mut self) {
        match self.heading {
            Command::North(_) => {
                self.heading = Command::West(0);
            }
            Command::South(_) => {
                self.heading = Command::East(0);
            }
            Command::East(_) => {
                self.heading = Command::North(0);
            }
            Command::West(_) => {
                self.heading = Command::South(0);
            }
            _ => {}
        }
    }

    fn turn_by(&mut self, degrees: i64) {
        let turns = degrees / 90;

        if turns < 0 {
            for _ in 0..turns * -1 {
                self.turn_ccw();
            }
        } else {
            for _ in 0..turns {
                self.turn_cw();
            }
        }
    }

    fn turn_wp_by(&mut self, degrees: i64) {
        let rad = (degrees) as f64 * 0.0174533;

        let s = rad.sin();
        let c = rad.cos();

        let x: f64 = (self.waypoint_x) as f64;
        let y: f64 = (self.waypoint_y) as f64;

        let nx = x * c - y * s;
        let ny = x * s + y * c;

        println!("X: {} Y: {} | S: {} C: {} | D: {}", nx, ny, s, c, degrees);

        self.waypoint_x = nx.round() as i64;
        self.waypoint_y = ny.round() as i64;
    }

    fn process_command(&mut self, cmd: Command) {
        match cmd {
            Command::North(u) => {
                self.y += u;
            }
            Command::South(u) => {
                self.y -= u;
            }
            Command::East(u) => {
                self.x += u;
            }
            Command::West(u) => {
                self.x -= u;
            }
            Command::Left(degrees) => {
                self.turn_by(-degrees);
            }
            Command::Right(degrees) => {
                self.turn_by(degrees);
            }
            Command::Forward(u) => {
                match self.heading {
                    Command::North(_) => {
                        self.y += u;
                    }
                    Command::South(_) => {
                        self.y -= u;
                    }
                    Command::East(_) => {
                        self.x += u;
                    }
                    Command::West(_) => {
                        self.x -= u;
                    }
                    _ => {}
                }
            }
        };
    }

    fn process_command_b(&mut self, cmd: Command) {
        match cmd {
            Command::North(u) => {
                self.waypoint_y += u;
            }
            Command::South(u) => {
                self.waypoint_y -= u;
            }
            Command::East(u) => {
                self.waypoint_x += u;
            }
            Command::West(u) => {
                self.waypoint_x -= u;
            }
            Command::Left(degrees) => {
                self.turn_wp_by(-degrees);
            }
            Command::Right(degrees) => {
                self.turn_wp_by(degrees);
            }
            Command::Forward(u) => {
                let mx = self.waypoint_x;
                let my = self.waypoint_y;
                
                self.x += mx * u;
                self.y += my * u;
            }
        };
    }

    fn manhattan_dist(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn day_12_a(input: &str) -> i64 {
    let commands: Vec<Command> = input
        .split('\n')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut ship = Ship::new();

    commands
        .iter()
        .for_each(|c| {
            println!("{:?}", c);
            ship.process_command(*c);
            println!("{:?}", ship);
        });

    ship.manhattan_dist()
}

#[test]
fn test_day_12_a() {
    println!("{:?}", day_12_a(TEST_DATA));
}

fn day_12_b(input: &str) -> i64 {
    let commands: Vec<Command> = input
        .split('\n')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut ship = Ship::new();

    commands
        .iter()
        .for_each(|c| {
            println!("{:?}", c);
            ship.process_command_b(*c);
            println!("{:?}", ship);
        });

    ship.manhattan_dist()
}

#[test]
fn test_day_12_b() {
    println!("{:?}", day_12_b(TEST_DATA));
}