use core::panic;

use euclid::default::Vector2D;

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let mut ship = Ship::new();
    for line in input.lines() {
        let (op, payload) = line.split_at(1);
        let payload: i32 = payload.parse().unwrap();
        match op {
            "N" => ship.move_in_dir(NORTH, payload),
            "S" => ship.move_in_dir(SOUTH, payload),
            "W" => ship.move_in_dir(WEST, payload),
            "E" => ship.move_in_dir(EAST, payload),
            "L" => ship.direction.rotate_left(payload),
            "R" => ship.direction.rotate_right(payload),
            "F" => ship.move_in_dir(ship.direction, payload),
            _ => panic!(),
        }
    }
    ship.position.x.abs() + ship.position.y.abs()
}
#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let mut ship = Ship::new();
    for line in input.lines() {
        let (op, payload) = line.split_at(1);
        let payload: i32 = payload.parse().unwrap();
        match op {
            "N" => ship.move_waypoint(NORTH, payload),
            "S" => ship.move_waypoint(SOUTH, payload),
            "W" => ship.move_waypoint(WEST, payload),
            "E" => ship.move_waypoint(EAST, payload),
            "L" => ship.waypoint.rotate_left(payload),
            "R" => ship.waypoint.rotate_right(payload),
            "F" => ship.move_to_waypoint(payload),
            _ => panic!(),
        }
    }
    ship.position.x.abs() + ship.position.y.abs()
}

const EAST: Vector2D<i32> = Vector2D::new(1, 0);
const SOUTH: Vector2D<i32> = Vector2D::new(0, 1);
const WEST: Vector2D<i32> = Vector2D::new(-1, 0);
const NORTH: Vector2D<i32> = Vector2D::new(0, -1);

struct Ship {
    direction: Vector2D<i32>,
    position: Vector2D<i32>,
    waypoint: Vector2D<i32>,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: EAST,
            position: Vector2D::new(0, 0),
            waypoint: Vector2D::new(10, -1),
        }
    }

    fn move_in_dir(&mut self, dir: Vector2D<i32>, n: i32) {
        self.position += dir * n;
    }
    fn move_waypoint(&mut self, dir: Vector2D<i32>, n: i32) {
        self.waypoint += dir * n;
    }
    fn move_to_waypoint(&mut self, n: i32) {
        self.position += self.waypoint * n;
    }
}

trait Rotate {
    fn rotate_right(&mut self, degrees: i32);
    fn rotate_left(&mut self, degrees: i32) {
        self.rotate_right(-degrees)
    }
}

impl Rotate for Vector2D<i32> {
    fn rotate_right(&mut self, degrees: i32) {
        let d = (degrees / 90).rem_euclid(4);
        (self.x, self.y) = match d {
            0 => (self.x, self.y),
            1 => (-self.y, self.x),
            2 => (-self.x, -self.y),
            3 => (self.y, -self.x),
            _ => unreachable!(),
        }
    }
}
