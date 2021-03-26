#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let map = Map::new(input);

    trees_for_slope(&map, 3, 1)
}
#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let map = Map::new(input);
    [
        trees_for_slope(&map, 1, 1),
        trees_for_slope(&map, 3, 1),
        trees_for_slope(&map, 5, 1),
        trees_for_slope(&map, 7, 1),
        trees_for_slope(&map, 1, 2),
    ]
    .iter()
    .product()
}

fn trees_for_slope(map: &Map, right: usize, down: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    loop {
        x += right;
        y += down;
        if y >= map.height {
            break trees;
        }
        if map.get(x, y) {
            trees += 1;
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl Map {
    fn new(str: &str) -> Self {
        let mut height = 0;
        let mut grid = Vec::with_capacity(str.len());
        for line in str.lines() {
            height += 1;
            for char in line.chars() {
                match char {
                    '.' => grid.push(false),
                    '#' => grid.push(true),
                    _ => unreachable!(),
                }
            }
        }
        let width = grid.len() / height;
        Self {
            width,
            height,
            grid,
        }
    }

    fn get(&self, mut x: usize, y: usize) -> bool {
        x %= self.width;
        self.grid[y * self.width + x]
    }
}
