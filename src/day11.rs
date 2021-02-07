use std::{
    cmp::{max, min},
    fmt::Display,
};

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    occupied_after_all_moves(grid)
}
#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    occupied_after_all_moves2(grid)
}

fn occupied_after_all_moves(mut grid: Grid) -> usize {
    let mut other_grid = grid.clone();
    loop {
        if tick(&grid, &mut other_grid) {
            std::mem::swap(&mut grid, &mut other_grid);
        } else {
            return grid
                .grid
                .iter()
                .filter(|e| {
                    if let Position::Occupied = e {
                        true
                    } else {
                        false
                    }
                })
                .count();
        }
    }
}

fn occupied_after_all_moves2(mut grid: Grid) -> usize {
    loop {
        if let Some(new_grid) = tick2(&grid) {
            grid = new_grid;
        } else {
            return grid
                .grid
                .iter()
                .filter(|e| {
                    if let Position::Occupied = e {
                        true
                    } else {
                        false
                    }
                })
                .count();
        }
    }
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Position>,
    width: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, p) in self.grid.iter().enumerate() {
            if idx % self.width == 0 {
                writeln!(f)?;
            }
            let c = match p {
                Position::Seat => 'L',
                Position::Occupied => '#',
                Position::Floor => '.',
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        for char in input.chars() {
            match char {
                'L' => grid.push(Position::Seat),
                '#' => grid.push(Position::Occupied),
                '.' => grid.push(Position::Floor),
                _ => {}
            }
        }
        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = input.lines().count();
        assert_eq!(width * height, grid.len());
        Self {
            grid,
            width,
            height,
        }
    }
    fn occupied(&self, pos: (usize, usize)) -> u8 {
        let mut occupied = 0;
        for x in max(pos.0, 1) - 1..=min(pos.0, self.width - 2) + 1 {
            for y in max(pos.1, 1) - 1..=min(pos.1, self.height - 2) + 1 {
                if (x, y) == pos {
                    continue;
                }
                if let Position::Occupied = self.grid[y * self.width + x] {
                    occupied += 1;
                }
            }
        }
        occupied
    }
    fn get(&self, pos: (usize, usize)) -> Option<&Position> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            Some(&self.grid[pos.1 * self.width + pos.0])
        } else {
            None
        }
    }
    fn occupied2(&self, pos: (usize, usize)) -> u8 {
        let mut occupied = 0;
        let directions = [
            (1, 1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (1, -1),
        ];
        'directions: for direction in &directions {
            let mut pos = pos;
            loop {
                pos.0 = (pos.0 as i32 + direction.0) as usize;
                pos.1 = (pos.1 as i32 + direction.1) as usize;
                match self.get(pos) {
                    Some(Position::Floor) => {}
                    Some(Position::Occupied) => {
                        occupied += 1;
                        continue 'directions;
                    }
                    None | Some(Position::Seat) => {
                        continue 'directions;
                    }
                }
            }
        }
        occupied
    }
}

#[derive(Clone)]
enum Position {
    Seat,
    Occupied,
    Floor,
}

fn tick(grid: &Grid, target: &mut Grid) -> bool {
    let mut changed = false;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let occupied = grid.occupied((x, y));
            match &grid.grid[y * grid.width + x] {
                Position::Seat if occupied == 0 => {
                    target.grid[y * grid.width + x] = Position::Occupied;
                    changed = true;
                }
                Position::Occupied if occupied >= 4 => {
                    target.grid[y * grid.width + x] = Position::Seat;
                    changed = true;
                }
                p => {
                    target.grid[y * grid.width + x] = p.clone();
                }
            }
        }
    }
    changed
}

fn tick2(grid: &Grid) -> Option<Grid> {
    let mut new_grid = grid.clone();
    let mut changed = false;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let occupied = grid.occupied2((x, y));
            match grid.grid[y * grid.width + x] {
                Position::Seat => {
                    if occupied == 0 {
                        new_grid.grid[y * grid.width + x] = Position::Occupied;
                        changed = true;
                    }
                }
                Position::Occupied => {
                    if occupied >= 5 {
                        new_grid.grid[y * grid.width + x] = Position::Seat;
                        changed = true;
                    }
                }
                Position::Floor => {}
            }
        }
    }
    if changed {
        Some(new_grid)
    } else {
        None
    }
}

#[test]
fn e1() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let grid = Grid::new(input);
    assert_eq!(occupied_after_all_moves(grid), 37);
}
#[test]
fn e2() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let grid = Grid::new(input);
    assert_eq!(occupied_after_all_moves2(grid), 26);
}
